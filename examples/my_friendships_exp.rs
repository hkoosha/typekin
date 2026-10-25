use std::ops::BitXor;

#[derive(Debug, Clone, Copy)]
struct Thing0(u128);

fn convert_for_make(it: Thing0) -> u128 {
    return it.0 / 4;
}

fn convert_for_bit(it: Thing0) -> u128 {
    return it.0.bitxor(0b01);
}

#[derive(Debug, Clone, Copy)]
pub struct Thingy(u128);

mod things {
    pub(super) trait Seal {}
    #[allow(unused, dead_code)]
    pub(super) trait Bit: Seal {
        fn to_u128(self) -> u128;
    }
    #[allow(unused, dead_code)]
    pub(super) trait Make: Seal {
        fn to_u128(self) -> u128;
    }
    impl Seal for super::Thing0 {}
    impl Bit for super::Thing0 {
        #[inline(always)]
        fn to_u128(self) -> u128 {
            return super::convert_for_bit(self);
        }
    }
    impl Make for super::Thing0 {
        #[inline(always)]
        fn to_u128(self) -> u128 {
            return super::convert_for_make(self);
        }
    }
    impl super::Thingy {
        #[allow(private_bounds)]
        #[inline(always)]
        pub fn of<T>(it: T) -> Self
        where
            T: Make + Seal,
        {
            return Self::of_parts(<T as Make>::to_u128(it));
        }
    }
    impl Seal for u128 {}
    impl Make for u128 {
        #[inline(always)]
        fn to_u128(self) -> u128 {
            return self;
        }
    }
}

impl Thingy {
    fn of_parts(it: u128) -> Self {
        return Self(it);
    }
}

fn requires_bit<T: things::Bit>(it: T) -> u128 {
    return it.to_u128();
}

fn main() {
    let vv = Thing0(7);
    let it = Thingy::of(vv);
    assert_eq!(it.0, vv.0 / 4);

    let vv = Thing0(0b01101);
    let bit = requires_bit(vv);
    assert_eq!(bit, 0b01100);
}
