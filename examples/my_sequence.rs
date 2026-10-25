use core::ops::{
    Add,
    Sub,
};

#[typekin::integral(
    konst = none,
    profile = [
        default {
            without = math_add,
        },
        circular {},
    ],
    out_type = SeqPoint::into_offset(_) -> SeqOffset,
)]
struct SeqPoint(u32);

#[typekin::integral(
    konst = none,
    profile = circular {
        precedes = false,
    },
)]
struct SeqOffset(i32);

impl SeqPoint {
    fn into_offset(self) -> SeqOffset {
        return SeqOffset::make(self.raw() as i32);
    }
}

impl Add<SeqOffset> for SeqPoint {
    type Output = Self;

    #[inline(always)]
    fn add(
        self,
        rhs: SeqOffset,
    ) -> Self::Output {
        return Self::make(self.raw().wrapping_add(rhs.raw() as u32));
    }
}

impl Sub<SeqOffset> for SeqPoint {
    type Output = Self;

    #[inline(always)]
    fn sub(
        self,
        rhs: SeqOffset,
    ) -> Self::Output {
        return Self::make(self.raw().wrapping_sub(rhs.raw() as u32));
    }
}

fn main() {
    let maximum = SeqPoint::make(u32::MAX);
    let zero = SeqPoint::make(0);
    let offset = zero - maximum;

    assert_eq!(offset.raw(), 1);
    assert!(maximum.precedes(zero));
    assert!(!zero.precedes(SeqPoint::make(1 << 31)));

    let wrapped = maximum + SeqOffset::make(1);
    assert_eq!(wrapped.raw(), 0);

    let offset = SeqOffset::make(i32::MAX) + SeqOffset::make(1);
    assert_eq!(offset.raw(), i32::MIN);
}
