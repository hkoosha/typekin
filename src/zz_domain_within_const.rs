// The subset check of `Within` proofs, over two forms (`zz_domain_within`).
//
// Typekin compiles this file as a module, deciding primitive sources at
// expansion, and emits its text verbatim into the generated scope of every
// target with a `Within` friend, where rustc's const evaluator runs it on
// the form the source publishes. One implementation serves both, which
// shapes the file: `//` comments only, `::core` paths, no `use`, no
// edition-specific syntax, `__typekin`-prefixed items, and stable
// `const fn` throughout.
//
// Values are sign and magnitude, `(negative, magnitude)`, and zero is never
// negative, so one implementation covers every integer type up to `u128`
// and `i128`. A form is a modulus and its pieces, sorted by
// `(residue, lo)`; a piece `(residue, lo_negative, lo, hi_negative, hi)`
// holds the members `lo, lo + modulus, …, hi`, which all leave `residue`
// modulo the modulus.

pub(crate) type __TypekinWithinValue = (bool, u128);

pub(crate) type __TypekinWithinPiece = (u128, bool, u128, bool, u128);

pub(crate) type __TypekinWithinForm<'a> = (u128, &'a [__TypekinWithinPiece]);

// The verdict of `__typekin_within_subset`.
pub(crate) enum __TypekinWithin {
    Proven,
    // A source member outside the target's raw range.
    Range(bool, u128),
    // A source member inside the raw range but outside the target's form.
    Domain(bool, u128),
    // The walk would take this many steps, above the budget.
    Budget(u128),
}

// The steps the walk may take: far below `long_running_const_eval`.
pub(crate) const __TYPEKIN_WITHIN_BUDGET: u128 = 65_536;

// Whether every member of `source` lies in `min..=max` and in `target`; a
// counterexample is the least member that does not.
pub(crate) const fn __typekin_within_subset(
    source: __TypekinWithinForm,
    target: __TypekinWithinForm,
    min: __TypekinWithinValue,
    max: __TypekinWithinValue,
) -> __TypekinWithin {
    if let ::core::option::Option::Some(it) =
        __typekin_within_outside(source, min, max)
    {
        return __TypekinWithin::Range(it.0, it.1);
    }

    // With `step` the lcm of the moduli, each source piece splits into
    // `step / source_modulus` progressions of stride `step`, and every point
    // of one progression leaves the same residue modulo the target's.
    let (source_modulus, source_pieces) = source;
    let (target_modulus, target_pieces) = target;
    let divisor = __typekin_within_gcd(source_modulus, target_modulus);
    let step = match (source_modulus / divisor).checked_mul(target_modulus) {
        ::core::option::Option::Some(it) => it,
        ::core::option::Option::None => {
            return __TypekinWithin::Budget(u128::MAX);
        }
    };
    let progressions = step / source_modulus;
    let walks = match progressions.checked_mul(source_pieces.len() as u128) {
        ::core::option::Option::Some(it) => it,
        ::core::option::Option::None => {
            return __TypekinWithin::Budget(u128::MAX);
        }
    };
    let steps = match walks.checked_mul(target_pieces.len() as u128 + 1) {
        ::core::option::Option::Some(it) => it,
        ::core::option::Option::None => {
            return __TypekinWithin::Budget(u128::MAX);
        }
    };
    if steps > __TYPEKIN_WITHIN_BUDGET {
        return __TypekinWithin::Budget(steps);
    }

    let mut least = ::core::option::Option::None;
    let mut index = 0;
    while index < source_pieces.len() {
        let (_, lo_negative, lo, hi_negative, hi) = source_pieces[index];
        let mut offset = 0;
        while offset < progressions {
            if let ::core::option::Option::Some(point) = __typekin_within_walk(
                (lo_negative, lo),
                offset * source_modulus,
                (hi_negative, hi),
                step,
                target,
            ) {
                least = __typekin_within_least(least, point);
            }
            offset += 1;
        }
        index += 1;
    }
    return match least {
        ::core::option::Option::Some(it) => __TypekinWithin::Domain(it.0, it.1),
        ::core::option::Option::None => __TypekinWithin::Proven,
    };
}

// The least member of `form` outside `min..=max`. Piece ends are members: a
// piece starting below `min` starts with one, and a piece ending above `max`
// holds a first member past it.
const fn __typekin_within_outside(
    form: __TypekinWithinForm,
    min: __TypekinWithinValue,
    max: __TypekinWithinValue,
) -> ::core::option::Option<__TypekinWithinValue> {
    let (modulus, pieces) = form;
    let mut least = ::core::option::Option::None;
    let mut index = 0;
    while index < pieces.len() {
        let (_, lo_negative, lo, hi_negative, hi) = pieces[index];
        let (lo, hi) = ((lo_negative, lo), (hi_negative, hi));
        if __typekin_within_less(lo, min) {
            least = __typekin_within_least(least, lo);
        }
        else if __typekin_within_less(max, hi) {
            let past = match __typekin_within_after(lo, max, modulus) {
                ::core::option::Option::Some(it) => it,
                ::core::option::Option::None => hi,
            };
            least = __typekin_within_least(least, past);
        }
        index += 1;
    }
    return least;
}

// The least of `start, start + step, …` above `bound`; `None` beyond
// `u128::MAX`.
const fn __typekin_within_after(
    start: __TypekinWithinValue,
    bound: __TypekinWithinValue,
    step: u128,
) -> ::core::option::Option<__TypekinWithinValue> {
    if __typekin_within_less(bound, start) {
        return ::core::option::Option::Some(start);
    }
    return match __typekin_within_gap(start, bound) {
        ::core::option::Option::Some(gap) => {
            __typekin_within_jump(start, gap, step)
        }
        ::core::option::Option::None => ::core::option::Option::None,
    };
}

// The least of `start + step, start + 2 * step, …` above `start + gap`;
// `None` beyond `u128::MAX`.
const fn __typekin_within_jump(
    start: __TypekinWithinValue,
    gap: u128,
    step: u128,
) -> ::core::option::Option<__TypekinWithinValue> {
    let jumps = match (gap / step).checked_add(1) {
        ::core::option::Option::Some(it) => it,
        ::core::option::Option::None => return ::core::option::Option::None,
    };
    return match jumps.checked_mul(step) {
        ::core::option::Option::Some(jump) => __typekin_within_add(start, jump),
        ::core::option::Option::None => ::core::option::Option::None,
    };
}

// The lesser of `least`, if any, and `value`.
const fn __typekin_within_least(
    least: ::core::option::Option<__TypekinWithinValue>,
    value: __TypekinWithinValue,
) -> ::core::option::Option<__TypekinWithinValue> {
    return match least {
        ::core::option::Option::Some(it)
            if !__typekin_within_less(value, it) =>
        {
            ::core::option::Option::Some(it)
        }
        _ => ::core::option::Option::Some(value),
    };
}

// The first point of `lo + offset, lo + offset + step, …` up to `end` that
// no target piece covers. The points share one residue modulo the target's
// modulus, so the walk only visits that residue's pieces, in order.
const fn __typekin_within_walk(
    lo: __TypekinWithinValue,
    offset: u128,
    end: __TypekinWithinValue,
    step: u128,
    target: __TypekinWithinForm,
) -> ::core::option::Option<__TypekinWithinValue> {
    let mut point = match __typekin_within_add(lo, offset) {
        ::core::option::Option::Some(it) => it,
        ::core::option::Option::None => return ::core::option::Option::None,
    };
    if __typekin_within_less(end, point) {
        return ::core::option::Option::None;
    }
    let (target_modulus, pieces) = target;
    let residue = __typekin_within_residue(point, target_modulus);
    let mut index = 0;
    while index < pieces.len() && pieces[index].0 < residue {
        index += 1;
    }
    loop {
        while index < pieces.len()
            && pieces[index].0 == residue
            && __typekin_within_less((pieces[index].3, pieces[index].4), point)
        {
            index += 1;
        }
        if index == pieces.len()
            || pieces[index].0 != residue
            || __typekin_within_less(point, (pieces[index].1, pieces[index].2))
        {
            return ::core::option::Option::Some(point);
        }

        // The piece covers every point up to its end; jump past it. A gap
        // too large to measure is reported as uncovered, and a jump beyond
        // `u128::MAX` lands beyond every value of any type, so beyond `end`.
        let gap = match __typekin_within_gap(
            point,
            (pieces[index].3, pieces[index].4),
        ) {
            ::core::option::Option::Some(it) => it,
            ::core::option::Option::None => {
                return ::core::option::Option::Some(point);
            }
        };
        point = match __typekin_within_jump(point, gap, step) {
            ::core::option::Option::Some(it) => it,
            ::core::option::Option::None => {
                return ::core::option::Option::None;
            }
        };
        if __typekin_within_less(end, point) {
            return ::core::option::Option::None;
        }
    }
}

// `left < right`.
const fn __typekin_within_less(
    left: __TypekinWithinValue,
    right: __TypekinWithinValue,
) -> bool {
    return match (left.0, right.0) {
        (true, false) => true,
        (false, true) => false,
        (false, false) => left.1 < right.1,
        (true, true) => left.1 > right.1,
    };
}

// `value + delta`; `None` above `u128::MAX`.
const fn __typekin_within_add(
    value: __TypekinWithinValue,
    delta: u128,
) -> ::core::option::Option<__TypekinWithinValue> {
    if !value.0 {
        return match value.1.checked_add(delta) {
            ::core::option::Option::Some(it) => {
                ::core::option::Option::Some((false, it))
            }
            ::core::option::Option::None => ::core::option::Option::None,
        };
    }
    if delta >= value.1 {
        return ::core::option::Option::Some((false, delta - value.1));
    }
    return ::core::option::Option::Some((true, value.1 - delta));
}

// `hi - lo` for `lo <= hi`; `None` otherwise or above `u128::MAX`.
const fn __typekin_within_gap(
    lo: __TypekinWithinValue,
    hi: __TypekinWithinValue,
) -> ::core::option::Option<u128> {
    return match (lo.0, hi.0) {
        (false, false) => hi.1.checked_sub(lo.1),
        (true, true) => lo.1.checked_sub(hi.1),
        (true, false) => lo.1.checked_add(hi.1),
        (false, true) => ::core::option::Option::None,
    };
}

// The Euclidean residue of `value` modulo `modulus`.
const fn __typekin_within_residue(
    value: __TypekinWithinValue,
    modulus: u128,
) -> u128 {
    let rest = value.1 % modulus;
    if value.0 && rest != 0 {
        return modulus - rest;
    }
    return rest;
}

const fn __typekin_within_gcd(
    left: u128,
    right: u128,
) -> u128 {
    let (mut left, mut right) = (left, right);
    while right != 0 {
        let rest = left % right;
        left = right;
        right = rest;
    }
    return left;
}

// A message assembled during const evaluation, which `panic!` takes as one
// `&str`.
pub(crate) struct __TypekinWithinText {
    bytes: [u8; 512],
    len: usize,
}

impl __TypekinWithinText {
    pub(crate) const fn new() -> Self {
        return Self {
            bytes: [0; 512],
            len: 0,
        };
    }

    // Appends `text`, dropping the characters that do not fit.
    pub(crate) const fn push(
        &mut self,
        text: &str,
    ) {
        let bytes = text.as_bytes();
        let mut index = 0;
        while index < bytes.len() {
            let lead = bytes[index];
            let width = if lead < 0x80 {
                1
            }
            else if lead >= 0xf0 {
                4
            }
            else if lead >= 0xe0 {
                3
            }
            else {
                2
            };
            if self.len + width > self.bytes.len() {
                return;
            }
            let mut offset = 0;
            while offset < width {
                self.bytes[self.len] = bytes[index + offset];
                self.len += 1;
                offset += 1;
            }
            index += width;
        }
    }

    // Appends `value` in decimal.
    pub(crate) const fn push_value(
        &mut self,
        value: __TypekinWithinValue,
    ) {
        if value.0 {
            self.push("-");
        }
        let mut digits = [0u8; 39];
        let mut count = 0;
        let mut rest = value.1;
        loop {
            digits[count] = b'0' + (rest % 10) as u8;
            count += 1;
            rest /= 10;
            if rest == 0 {
                break;
            }
        }
        while count > 0 && self.len < self.bytes.len() {
            count -= 1;
            self.bytes[self.len] = digits[count];
            self.len += 1;
        }
    }

    pub(crate) const fn as_str(&self) -> &str {
        let (text, _) = self.bytes.split_at(self.len);
        return match ::core::str::from_utf8(text) {
            ::core::result::Result::Ok(it) => it,
            ::core::result::Result::Err(_) => "",
        };
    }
}
