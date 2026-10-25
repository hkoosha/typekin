#[typekin::integral(
    konst = none,
    profile = circular {},
    out_type = CircularSequence::into_raw(_) -> u32,
)]
pub struct CircularSequence(u32);

impl CircularSequence {
    fn into_raw(self) -> u32 {
        return self.raw();
    }
}

pub struct Out;

#[typekin::integral(
    konst = none,
    profile = circular {},
    without = math_add,
    out_type = Self::into_offset(_) -> GenericOffset<D>,
    facets = [
        circular {
            with = math_add,
            friends = Self::offset_raw(GenericOffset<D>) -> Math,
            out_type = Self,
        },
    ],
)]
pub struct GenericPoint<D>(u32);

#[typekin::integral(konst = none, profile = circular { precedes = false })]
pub struct GenericOffset<D>(i32);

impl<D> GenericPoint<D> {
    #[inline(always)]
    fn into_offset(self) -> GenericOffset<D> {
        return GenericOffset::make(self.raw() as i32);
    }

    #[inline(always)]
    fn offset_raw(offset: GenericOffset<D>) -> u32 {
        return offset.raw() as u32;
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_sequence(
    lhs: u32,
    rhs: u32,
) -> u32 {
    return lhs.wrapping_sub(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_sequence(
    lhs: CircularSequence,
    rhs: CircularSequence,
) -> u32 {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_precedes_sequence(
    lhs: u32,
    rhs: u32,
) -> bool {
    let difference = rhs.wrapping_sub(lhs);
    return difference != 0 && difference < (1 << 31);
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
pub extern "C" fn raw_sub_generic_points(lhs: u32, rhs: u32) -> i32 {
    return lhs.wrapping_sub(rhs) as i32;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_generic_points(
    lhs: GenericPoint<Out>,
    rhs: GenericPoint<Out>,
) -> i32 {
    return (lhs - rhs).raw();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_generic_offset(lhs: u32, rhs: i32) -> u32 {
    return lhs.wrapping_add(rhs as u32);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_generic_offset(
    lhs: GenericPoint<Out>,
    rhs: GenericOffset<Out>,
) -> u32 {
    return (lhs + rhs).raw();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_generic_offset(lhs: u32, rhs: i32) -> u32 {
    return lhs.wrapping_sub(rhs as u32);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_generic_offset(
    lhs: GenericPoint<Out>,
    rhs: GenericOffset<Out>,
) -> u32 {
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

fn main() {
    assert_eq!(raw_sub_sequence(0, u32::MAX), 1);
    assert_eq!(
        wrapped_sub_sequence(
            CircularSequence::make(0),
            CircularSequence::make(u32::MAX),
        ),
        1,
    );
    assert!(raw_precedes_sequence(u32::MAX, 0));
    assert!(wrapped_precedes_sequence(
        CircularSequence::make(u32::MAX),
        CircularSequence::make(0),
    ));
    assert_eq!(raw_sub_generic_points(0, 1), -1);
    assert_eq!(wrapped_sub_generic_points(
        GenericPoint::make(0), GenericPoint::make(1),
    ), -1);
    assert_eq!(raw_add_generic_offset(u32::MAX, 1), 0);
    assert_eq!(wrapped_add_generic_offset(
        GenericPoint::make(u32::MAX), GenericOffset::make(1),
    ), 0);
    assert_eq!(raw_sub_generic_offset(0, 1), u32::MAX);
    assert_eq!(wrapped_sub_generic_offset(
        GenericPoint::make(0), GenericOffset::make(1),
    ), u32::MAX);
    assert!(wrapped_precedes_generic_point(
        GenericPoint::make(u32::MAX), GenericPoint::make(0),
    ));
}
