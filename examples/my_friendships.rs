mod muti_fn_friendship {
    struct Thing0(u32);

    fn convert_t0(it: Thing0) -> u128 {
        return it.0 as u128;
    }

    #[typekin::friends(
        relation = ::core::primitive::u128,
        friends = [
            _ -> Haha,
            convert_t0(Thing0) -> [Make],
            convert_t1(Thing0) -> [Bit],
        ],
        scope = things,
    )]
    #[derive(Debug, Clone, Copy)]
    pub struct Thingy(u128);

    impl Thingy {
        fn of_parts(it: u128) -> Self {
            return Self(it);
        }
    }

    fn requires_bit<T: things::Bit>(_: T) {}

    pub fn ekran() {
        requires_bit(Thing0(1));
        assert_eq!(Thingy::of(Thing0(7)).0, 7);
    }
}

fn main() {
    muti_fn_friendship::ekran();
}
