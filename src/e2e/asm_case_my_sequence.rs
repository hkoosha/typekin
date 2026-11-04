//! Circular sequences on 16 bits, a width no other case does arithmetic on,
//! so their probes cannot merge with `my_u32`'s.

#[typekin::circular(
    produce = CircularSequence::into_raw(_) -> u16,
)]
pub struct CircularSequence(u16);

impl CircularSequence {
    fn into_raw(self) -> u16 {
        return self.raw();
    }
}

pub struct Out;

#[inline]
const fn is_valid_generic_point(_: u16) -> bool {
    return true;
}

#[typekin::integral(
    valid = is_valid_generic_point,
    profile = [circular {}, point {}],
    produce = Self::into_offset(_) -> GenericOffset<D>,
    facet = [
        circular {
            with = math_add,
            friend = Self::offset_raw(GenericOffset<D>) -> Math,
            produce = Self,
        },
    ],
)]
pub struct GenericPoint<D>(u16);

#[typekin::delta]
pub struct GenericOffset<D>(i16);

impl<D> GenericPoint<D> {
    #[inline(always)]
    fn into_offset(self) -> GenericOffset<D> {
        return GenericOffset::make(self.raw() as i16);
    }

    #[inline(always)]
    fn offset_raw(offset: GenericOffset<D>) -> u16 {
        return offset.raw() as u16;
    }
}

fn generic_point<D>(raw: u16) -> GenericPoint<D> {
    return match GenericPoint::try_make(raw) {
        Ok(value) => value,
        Err(_) => unreachable!(),
    };
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_sequence(
    lhs: u16,
    rhs: u16,
) -> u16 {
    return lhs.wrapping_sub(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_sequence(
    lhs: CircularSequence,
    rhs: CircularSequence,
) -> u16 {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_precedes_sequence(
    lhs: u16,
    rhs: u16,
) -> bool {
    let difference = rhs.wrapping_sub(lhs);
    return difference != 0 && difference < (1 << 15);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_precedes_sequence(
    lhs: CircularSequence,
    rhs: CircularSequence,
) -> bool {
    return lhs.precedes(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_generic_points(
    lhs: u16,
    rhs: u16,
) -> i16 {
    return lhs.wrapping_sub(rhs) as i16;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_generic_points(
    lhs: GenericPoint<Out>,
    rhs: GenericPoint<Out>,
) -> i16 {
    return (lhs - rhs).raw();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_generic_offset(
    lhs: u16,
    rhs: i16,
) -> u16 {
    return lhs.wrapping_add(rhs as u16);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_generic_offset(
    lhs: GenericPoint<Out>,
    rhs: GenericOffset<Out>,
) -> u16 {
    return (lhs + rhs).raw();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_generic_offset(
    lhs: u16,
    rhs: i16,
) -> u16 {
    return lhs.wrapping_sub(rhs as u16);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_generic_offset(
    lhs: GenericPoint<Out>,
    rhs: GenericOffset<Out>,
) -> u16 {
    return (lhs - rhs).raw();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_precedes_generic_point(
    lhs: GenericPoint<Out>,
    rhs: GenericPoint<Out>,
) -> bool {
    return lhs.precedes(rhs);
}

pub fn main() {
    assert_eq!(raw_sub_sequence(0, u16::MAX), 1);
    assert_eq!(
        wrapped_sub_sequence(
            CircularSequence::make(0),
            CircularSequence::make(u16::MAX),
        ),
        1,
    );
    assert!(raw_precedes_sequence(u16::MAX, 0));
    assert!(wrapped_precedes_sequence(
        CircularSequence::make(u16::MAX),
        CircularSequence::make(0),
    ));
    assert_eq!(raw_sub_generic_points(0, 1), -1);
    assert_eq!(
        wrapped_sub_generic_points(
            generic_point::<Out>(0),
            generic_point::<Out>(1),
        ),
        -1
    );
    assert_eq!(raw_add_generic_offset(u16::MAX, 1), 0);
    assert_eq!(
        wrapped_add_generic_offset(
            generic_point::<Out>(u16::MAX),
            GenericOffset::make(1),
        ),
        0
    );
    assert_eq!(raw_sub_generic_offset(0, 1), u16::MAX);
    assert_eq!(
        wrapped_sub_generic_offset(
            generic_point::<Out>(0),
            GenericOffset::make(1),
        ),
        u16::MAX
    );
    assert!(wrapped_precedes_generic_point(
        generic_point::<Out>(u16::MAX),
        generic_point::<Out>(0),
    ));
}
