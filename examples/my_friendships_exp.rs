struct Thing0(u32);

fn convert_t0(it: Thing0) -> u128 {
    return (it.0 as u128) / 4;
}

fn convert_t1(it: Thing0) -> u128 {
    return (it.0 as u128) / 2;
}

#[derive(Debug, Clone, Copy)]
pub struct Thingy(u128);

mod things {
    pub(super) trait Seal {
        fn to_u128(self) -> ::core::primitive::u128;
    }
    #[allow(unused, dead_code)]
    pub(super) trait Bit: Seal {}
    #[allow(unused, dead_code)]
    pub(super) trait Make: Seal {}
    impl Seal for super::Thing0 {
        #[inline(always)]
        fn to_u128(self) -> ::core::primitive::u128 {
            return super::convert_t1(self);
        }
    }
    impl Bit for super::Thing0 {}
    impl Make for super::Thing0 {}
    impl super::Thingy {
        #[allow(private_bounds)]
        #[inline(always)]
        pub fn of<T>(it: T) -> Self
        where
            T: Make + Seal,
        {
            return Self::of_parts(<T as Seal>::to_u128(it));
        }
    }
    impl Seal for ::core::primitive::u128 {
        #[inline(always)]
        fn to_u128(self) -> ::core::primitive::u128 {
            return self;
        }
    }
    impl Make for ::core::primitive::u128 {}
}

impl Thingy {
    fn of_parts(it: u128) -> Self {
        return Self(it);
    }
}

fn requires_bit<T: things::Bit>(_: T) {}

fn main() {
    requires_bit(Thing0(1));
    assert_eq!(Thingy::of(Thing0(7)).0, 7);

    assert_eq!(Thingy::of(Thing0(2)).0, 2);
}
