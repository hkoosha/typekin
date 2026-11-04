use crate::fx_compile_pass::LITERAL_PROVIDER;
use crate::*;
use std::sync::LazyLock;

/// Depends on `literal_provider`, a sibling fixture directory.
pub(crate) const REJECTED: Fixture = Fixture {
    name: "rejected",
    toolchain: Toolchain::Nightly,
    target: None,
    header: "",
    dependencies: r#"literal_provider = { path = "../literal_provider" }"#,
};

pub(crate) static CHECKED: LazyLock<Checked> = LazyLock::new(|| {
    write_crate("literal_provider", "", LITERAL_PROVIDER);
    return check(
        &REJECTED,
        &rejections(&[
            &HIDDEN_TUPLE_CONSTRUCTION,
            &HIDDEN_FIELD_ACCESS,
            &HIDDEN_FN_LOCAL_VALIDATOR,
            &FRIEND_DIRECTION,
            &LOSSY_INTEGRAL_FRIEND,
            &FLOAT_FRIEND_OF_INTEGRAL,
            &LIFETIME_ESCAPE,
            &LITERAL_SAME_NAMED_TYPE,
            &LITERAL_OUTER_GENERICS,
            &LITERAL_OUTSIDE_DOMAIN,
            &LITERAL_FLOAT_OUTSIDE_DOMAIN,
            &LITERAL_FAILING_CALLBACK,
            &SQRT_DOMAIN_NOT_CONST,
            &WITHIN_OUTSIDE_DOMAIN,
            &WITHIN_OUTSIDE_RAW_RANGE,
            &WITHIN_FAILING_CALLBACK,
            &WITHIN_OPAQUE_SOURCE,
            &WITHIN_OVER_BUDGET,
            &WITHIN_UNPUBLISHED_SOURCE,
            &WITHIN_OTHER_CRATE,
            &WITHIN_POINTER_WIDTH,
        ]),
    );
});

/// Built, not checked: constants used only by generic code are evaluated
/// where it is instantiated.
pub(crate) const REJECTED_BUILT: Fixture = Fixture {
    name: "rejected_built",
    toolchain: Toolchain::Nightly,
    target: None,
    header: "",
    dependencies: "",
};

pub(crate) static BUILT: LazyLock<Checked> = LazyLock::new(|| {
    return compile(&REJECTED_BUILT, &rejections(&[&WITHIN_GENERIC_TARGET]));
});

// -------------------------------------------------------------------------
// hide_in

pub(crate) const HIDDEN_TUPLE_CONSTRUCTION: Rejection = Rejection {
    name: "hidden_type_rejects_tuple_construction_in_defining_module",
    expected: "cannot initialize a tuple struct which contains private fields",
    source: r#"
#[typekin::integral(konst = none)]
pub struct Port(u16);

pub fn forge() -> Port {
    return Port(5);
}
"#,
};

pub(crate) const HIDDEN_FIELD_ACCESS: Rejection = Rejection {
    name: "hidden_type_rejects_field_access_in_defining_module",
    expected: "field `0` of struct `hidden_type_rejects_field_access_in_defining_module::__port::Port` is private",
    source: r#"
#[typekin::integral(konst = none)]
pub struct Port(u16);

pub fn peek(port: Port) -> u16 {
    return port.0;
}
"#,
};

pub(crate) const HIDDEN_FN_LOCAL_VALIDATOR: Rejection = Rejection {
    name: "hidden_type_cannot_see_fn_local_validator",
    expected: "cannot find function `is_valid` in this scope",
    source: r#"
pub fn local() {
    const fn is_valid(raw: u8) -> bool { raw != 0 }

    #[typekin::integral(konst = none, valid = is_valid)]
    struct Local(u8);
}
"#,
};

// -------------------------------------------------------------------------
// Friends

/// `of` is bounded by the generated, private `Make` protocol, which no
/// in-crate assertion can name.
pub(crate) const FRIEND_DIRECTION: Rejection = Rejection {
    name: "friend_construction_cannot_change_direction",
    expected: "the trait bound `Mark<Inbound>: __mark::_::Make<Outbound>` is not satisfied",
    source: r#"
#[typekin::circular(
    tree = Lane {
        Inbound,
        Outbound,
    },
    without = math_add,
    produce = Self::into_gap(_) -> Gap<L>,
    facet = circular {
        with = math_add,
        produce = Self,
        friend = Self::gap_raw(Gap<L>) -> Math,
    },
)]
struct Mark<L: Lane>(u32);

#[typekin::delta]
struct Gap<L: Lane>(i32);

impl<L: Lane> Mark<L> {
    fn into_gap(self) -> Gap<L> {
        return Gap::make(self.raw() as i32);
    }

    const fn gap_raw(gap: Gap<L>) -> u32 {
        return gap.raw() as u32;
    }
}

fn forbidden() {
    let inbound = Mark::<Inbound>::make(1);
    let _ = Mark::<Outbound>::of(inbound);
}
"#,
};

/// `u64 -> f64` can round, so the integral newtype has no `Into<f64>`.
pub(crate) const LOSSY_INTEGRAL_FRIEND: Rejection = Rejection {
    name: "float_rejects_a_lossy_integral_newtype_friend",
    expected: "the trait `From<float_rejects_a_lossy_integral_newtype_friend::__count::Count>` is not implemented for `f64`",
    source: r#"
#[typekin::integral]
pub struct Count(u64);

#[typekin::float(friend = _(Count) -> Make)]
pub struct Meters(f64);
"#,
};

/// Floats never convert to integers implicitly.
pub(crate) const FLOAT_FRIEND_OF_INTEGRAL: Rejection = Rejection {
    name: "integral_rejects_a_float_newtype_friend",
    expected: "the trait `From<integral_rejects_a_float_newtype_friend::__meters::Meters>` is not implemented for `u64`",
    source: r#"
#[typekin::float]
pub struct Meters(f64);

#[typekin::integral(friend = _(Meters) -> Make)]
pub struct Count(u64);
"#,
};

// -------------------------------------------------------------------------
// Generic markers

pub(crate) const LIFETIME_ESCAPE: Rejection = Rejection {
    name: "lifetime_markers_cannot_escape_their_borrow",
    expected: "cannot return value referencing local variable `owner`",
    source: r#"
#[typekin::integral]
struct Borrowed<'a>(u32);

fn tied_to<'a>(_: &'a str) -> Borrowed<'a> {
    return Borrowed::make(1);
}

fn forbidden() -> Borrowed<'static> {
    let owner = String::from("local");
    return tied_to(&owner);
}
"#,
};

// -------------------------------------------------------------------------
// Literal macros

pub(crate) const LITERAL_SAME_NAMED_TYPE: Rejection = Rejection {
    name: "literal_rejects_a_same_named_type_at_the_call_site",
    expected: "no method named `__typekin_lit_",
    source: r#"
pub mod net {
    #[typekin::integral(domain = 1..)]
    pub struct Port(u16);
}

pub struct Port(u16);

impl Port {
    pub const fn try_make(raw: u16) -> Result<Self, u16> {
        return Ok(Self(raw));
    }
}

pub fn wrong() -> Port {
    return net::port!(443);
}
"#,
};

pub(crate) const LITERAL_OUTER_GENERICS: Rejection = Rejection {
    name: "literal_item_form_rejects_outer_generic_parameters",
    expected: "can't use generic parameters from outer item",
    source: r#"
#[typekin::integral(domain = 1..)]
pub struct Tagged<D>(u32);

pub fn unused<D>() -> Tagged<D> {
    return tagged!([D] 5);
}
"#,
};

pub(crate) const LITERAL_OUTSIDE_DOMAIN: Rejection = Rejection {
    name: "literal_rejects_a_value_outside_the_domain",
    expected: "0 is outside the domain of Port",
    source: r#"
#[typekin::integral(domain = 1..=1023)]
pub struct Port(u16);

pub fn unused() -> Port {
    return port!(0);
}
"#,
};

pub(crate) const LITERAL_FLOAT_OUTSIDE_DOMAIN: Rejection = Rejection {
    name: "literal_rejects_a_float_outside_the_domain",
    expected: "1.5 is outside the domain of Unit",
    source: r#"
#[typekin::float(domain = 0.0..=1.0)]
pub struct Unit(f64);

pub fn unused() -> Unit {
    return unit!(1.5);
}
"#,
};

pub(crate) const LITERAL_FAILING_CALLBACK: Rejection = Rejection {
    name: "literal_rejects_a_value_failing_a_valid_callback",
    expected: "3 is outside the domain of Even",
    source: r#"
const fn is_even(raw: u8) -> bool {
    return raw % 2 == 0;
}

#[typekin::integral(valid = is_even)]
pub struct Even(u8);

pub const THREE: Even = even!(3);
"#,
};

// -------------------------------------------------------------------------
// Constness

pub(crate) const SQRT_DOMAIN_NOT_CONST: Rejection = Rejection {
    name: "float_domain_sqrt_is_not_const",
    expected: "cannot call non-const method `std::f64::<impl f64>::sqrt` in constant functions",
    source: r#"
#[typekin::float(konst = stable, math = std, domain = sqrt in ..=2.0)]
pub struct Root(f64);
"#,
};

// -------------------------------------------------------------------------
// Within

pub(crate) const WITHIN_OUTSIDE_DOMAIN: Rejection = Rejection {
    name: "within_refutes_a_value_outside_the_domain",
    expected: "`Byte` is not within `Positive`: 0 is outside the domain of `Positive`",
    source: r#"
#[typekin::integral(domain = ..=255)]
pub struct Byte(u16);

#[typekin::integral(domain = 1.., friend = _(Byte) -> Within)]
pub struct Positive(u16);
"#,
};

pub(crate) const WITHIN_OUTSIDE_RAW_RANGE: Rejection = Rejection {
    name: "within_refutes_a_value_outside_the_raw_range",
    expected: "`Word` is not within `Small`: 256 does not fit `u8`",
    source: r#"
#[typekin::integral]
pub struct Word(u16);

#[typekin::integral(friend = _(Word) -> Within)]
pub struct Small(u8);
"#,
};

/// The source's table captures its callback; the target's own validation
/// runs at every value it admits.
pub(crate) const WITHIN_FAILING_CALLBACK: Rejection = Rejection {
    name: "within_table_refutes_a_failing_callback",
    expected: "`Odd` is not within `NotSeven`: 7 is outside the domain of `NotSeven`",
    source: r#"
const fn is_odd(raw: i8) -> bool {
    return raw % 2 != 0;
}

#[typekin::integral(domain = -50..=50, valid = is_odd)]
pub struct Odd(i8);

const fn not_seven(raw: i16) -> bool {
    return raw != 7;
}

#[typekin::integral(
    konst = stable,
    valid = not_seven,
    friend = _(Odd) -> Within,
)]
pub struct NotSeven(i16);
"#,
};

pub(crate) const WITHIN_OPAQUE_SOURCE: Rejection = Rejection {
    name: "within_reports_an_opaque_source",
    expected: "cannot prove `Wide` within `Narrow`: `Wide`'s domain uses `count_ones`, which is opaque to subdomain proofs",
    source: r#"
#[typekin::integral(domain = count_ones in {3})]
pub struct Wide(u64);

#[typekin::integral(friend = _(Wide) -> Within)]
pub struct Narrow(u64);
"#,
};

pub(crate) const WITHIN_OVER_BUDGET: Rejection = Rejection {
    name: "within_reports_a_check_over_budget",
    expected: "cannot prove `Big` within `Spread`: the check needs 101909 steps, above the subdomain proof budget of 65536",
    source: r#"
#[typekin::integral(domain = mul(997))]
pub struct Big(u64);

#[typekin::integral(
    domain = rem_euclid(1009) in ..100,
    friend = _(Big) -> Within,
)]
pub struct Spread(u64);
"#,
};

/// `without = within_source` withholds the constants a target reads.
pub(crate) const WITHIN_UNPUBLISHED_SOURCE: Rejection = Rejection {
    name: "within_needs_the_source_constants",
    expected: "no associated function or constant named `__",
    source: r#"
#[typekin::integral(domain = ..=100, without = within_source)]
pub struct Percent(u8);

#[typekin::integral(domain = ..=255, friend = _(Percent) -> Within)]
pub struct Byte(u16);
"#,
};

pub(crate) const WITHIN_OTHER_CRATE: Rejection = Rejection {
    name: "within_refutes_a_source_from_another_crate",
    expected: "`literal_provider::net::Port` is not within `Small`: 1001 is outside the domain of `Small`",
    source: r#"
#[typekin::integral(
    domain = ..=1000,
    friend = _(literal_provider::net::Port) -> Within,
)]
pub struct Small(u16);
"#,
};

/// A `usize` source that fits `u32` only on 32-bit targets
/// (`test_pointer_width_compile` accepts it there).
pub(crate) const WITHIN_POINTER_WIDTH: Rejection = Rejection {
    name: "within_refutes_a_usize_source_on_64_bit_targets",
    expected: "`Index` is not within `Small`: 4294967296 does not fit `u32`",
    source: r#"
#[typekin::integral]
pub struct Index(usize);

#[typekin::integral(friend = _(Index) -> Within)]
pub struct Small(u32);
"#,
};

/// A generic target proves where it is instantiated; `cargo check` accepts
/// the same code (`test_compile_pass`), which is why this case is built.
/// `no_mangle` makes `half` a codegen root of the private case module.
pub(crate) const WITHIN_GENERIC_SOURCE: &str = r#"
#[typekin::integral(domain = ..=100)]
pub struct Tagged<D>(u16);

#[typekin::integral(domain = ..=50, friend = _(Tagged<D>) -> Within)]
pub struct Half<D>(u16);

pub struct In;

#[unsafe(no_mangle)]
pub fn half(tagged: Tagged<In>) -> Half<In> {
    return Half::of(tagged);
}
"#;

pub(crate) const WITHIN_GENERIC_TARGET: Rejection = Rejection {
    name: "within_refutes_a_generic_target_where_instantiated",
    expected: "`Tagged<D>` is not within `Half`: 51 is outside the domain of `Half`",
    source: WITHIN_GENERIC_SOURCE,
};
