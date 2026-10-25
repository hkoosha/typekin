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
}
