#![allow(unused, dead_code)]

use crate::*;
use core::marker::PhantomData;

pub(crate) struct NotTraits;

#[typekin::integral(hide_in = none, with = [display, serde, math_xor])]
pub(crate) struct Tagged<T: ?Sized = NotTraits>(u32);

pub(crate) fn tagged_fields<T: ?Sized>(
    value: Tagged<T>
) -> (u32, PhantomData<T>) {
    return (value.0, value.1);
}

#[typekin::integral(hide_in = none)]
pub(crate) struct Untagged(u32);

pub(crate) fn untagged_field(value: Untagged) -> u32 {
    let Untagged(raw) = value;
    return raw;
}

#[typekin::integral(hide_in = none,  with = [display, serde])]
pub(crate) struct Mixed<
    'de,
    T,
    F,
    H,
    S,
    D,
    const SPACE: u8,
    const FLAG: bool,
    Meta = str,
>(u16)
where
    T: ?Sized,
    Meta: ?Sized;

pub(crate) type CollisionTagged<'a> =
    Mixed<'a, NotTraits, NotTraits, NotTraits, NotTraits, NotTraits, 7, true>;

pub(crate) fn collision_tagged_fields<'de>(
    value: CollisionTagged<'de>
) -> (
    u16,
    PhantomData<&'de ()>,
    PhantomData<NotTraits>,
    PhantomData<NotTraits>,
    PhantomData<NotTraits>,
    PhantomData<NotTraits>,
    PhantomData<NotTraits>,
    PhantomData<str>,
) {
    return (
        value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.9,
    );
}

#[typekin::integral]
pub(crate) struct ConstTagged<const BYTE: u8 = 3, const SYMBOL: char = 'x'>(
    u32,
);

#[typekin::integral(hide_in = none,  domain = 1..)]
pub(crate) struct Positive<T>(u16);

pub(crate) fn positive_storage<T>(value: Positive<T>) -> core::num::NonZeroU16 {
    return value.0;
}

#[typekin::integral(hide_in = none,  profile = wrapping {})]
pub(crate) struct Wrapped<T>(u8);

pub(crate) fn wrapped_storage<T>(value: Wrapped<T>) -> core::num::Wrapping<u8> {
    return value.0;
}

#[typekin::integral(without = repr)]
pub(crate) struct ReprFree<T: ?Sized>(u64);

pub(crate) struct CheckedFriend<T>(pub(crate) u32, pub(crate) PhantomData<T>);
pub(crate) struct BlindTrustFriend<T>(
    pub(crate) u32,
    pub(crate) PhantomData<T>,
);

pub(crate) const fn is_even(value: u32) -> bool {
    return value % 2 == 0;
}

#[typekin::integral(
    hide_in = none,
    domain = 2..=100,
    valid = is_even,
    profile = wrapping {},
    friend = [
        Self::checked_raw(CheckedFriend<T>) -> Make,
        Self::blind_trust_raw(BlindTrustFriend<T>) -> [Make, unsafe],
        self(u32) -> [Math, Rel],
    ],
    with = [serde, math_xor],
)]
pub(crate) struct Checked<T>(u32);

pub(crate) fn checked_storage<T>(
    value: Checked<T>
) -> core::num::Wrapping<core::num::NonZeroU32> {
    return value.0;
}

impl<T> Checked<T> {
    const fn checked_raw(friend: CheckedFriend<T>) -> u32 {
        return friend.0;
    }

    const fn blind_trust_raw(friend: BlindTrustFriend<T>) -> u32 {
        return friend.0;
    }
}

#[typekin::integral(
    produce = Self::into_tagged(_) -> Tagged<T>,
    with = math_xor,
)]
pub(crate) struct Producing<T>(u32);

impl<T> Producing<T> {
    fn into_tagged(self) -> Tagged<T> {
        return Tagged::make(self.raw());
    }
}

pub(crate) trait OwnIdentity<T> {}
pub(crate) struct OnlySelf;

#[typekin::integral(
    profile = circular {},
    facet = [
        circular {
            friend = Self::delta_raw(Tagged<T>) -> Math,
            produce = Self,
        },
    ],
)]
pub(crate) struct WhereSelf<T>(u8)
where
    T: OwnIdentity<Self>;

impl OwnIdentity<WhereSelf<OnlySelf>> for OnlySelf {}

impl<T> WhereSelf<T>
where
    T: OwnIdentity<Self>,
{
    fn delta_raw(delta: Tagged<T>) -> u8 {
        return delta.raw() as u8;
    }
}

#[typekin::tree(
    Direction {
        In,
        Out,
    },
)]
pub(crate) trait MakerSeal {}

#[typekin::integral(
    profile = [circular, point],
    produce = Self::into_offset(_) -> Offset<'a, D, SPACE>,
    facet = circular {
        with = math_add,
        friend = Self::offset_raw(Offset<'a, D, SPACE>) -> Math,
        produce = Self,
    },
)]
pub(crate) struct Point<'a, D: Direction, const SPACE: u8>(u32);

#[typekin::delta]
pub(crate) struct Offset<'a, D: Direction, const SPACE: u8>(i32);

impl<'a, D: Direction, const SPACE: u8> Point<'a, D, SPACE> {
    fn into_offset(self) -> Offset<'a, D, SPACE> {
        return Offset::make(self.raw() as i32);
    }

    fn offset_raw(offset: Offset<'a, D, SPACE>) -> u32 {
        return offset.raw() as u32;
    }
}

#[typekin::integral(konst = nightly)]
pub(crate) struct ConstValue<T>(u8);

pub(crate) const CONST_CLONED: ConstValue<NotTraits> =
    ConstValue::make(7).clone();
pub(crate) const CONST_ADDED: ConstValue<NotTraits> =
    ConstValue::make(2) + ConstValue::make(3);
pub(crate) const CONST_MAPPED: ConstValue<NotTraits> =
    ConstValue::make(2).map(const |value| value + 3);
pub(crate) const STABLE_CHECKED: Checked<NotTraits> = match Checked::try_make(4)
{
    Ok(value) => value,
    Err(_) => panic!("valid generic value rejected"),
};

#[typekin::integral(
    konst = nightly,
    profile = circular {},
    facet = [
        circular {
            friend = Tagged::raw(Tagged<T>) -> Math,
        },
    ],
)]
pub(crate) struct ConstFacet<T>(u32);

pub(crate) const CONST_FACET_RESULT: ConstFacet<NotTraits> =
    ConstFacet::make(u32::MAX)
        + match Tagged::<NotTraits>::try_make(2) {
            Ok(value) => value,
            Err(_) => panic!("valid generic value rejected"),
        };

#[typekin::integral]
pub(crate) struct ProtocolNames<
    Seal,
    Make,
    Math,
    Bit,
    Rel,
    UnsafeTrust,
    T,
    __TypekinT,
>(u8);

pub(crate) type CollisionProtocol = ProtocolNames<
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
>;

#[typekin::tree(
    pub Transport {
        pub DirectionBranch {
            pub Incoming,
            pub Outgoing,
        },
    },
)]
pub(crate) trait TreeSeal {}

#[typekin::circular(
    tree = Lane { Inbound, Outbound },
    without = math_add,
    produce = Self::into_gap(_) -> Gap<L>,
    facet = circular {
        with = math_add,
        produce = Self,
        friend = Self::gap_raw(Gap<L>) -> Math,
    },
)]
pub(crate) struct Mark<L: Lane>(u32);

#[typekin::delta]
pub(crate) struct Gap<L: Lane>(i32);

impl<L: Lane> Mark<L> {
    const fn into_gap(self) -> Gap<L> {
        return match Gap::try_make(self.raw() as i32) {
            Ok(gap) => gap,
            Err(_) => panic!("gap rejected"),
        };
    }

    const fn gap_raw(gap: Gap<L>) -> u32 {
        return gap.raw() as u32;
    }
}

#[typekin::integral]
pub(crate) struct SpaceTagged<const SPACE: u8>(u32);

#[typekin::integral]
pub(crate) struct OwningMarker<T>(u32);

#[derive(PartialEq, Eq, core::marker::ConstParamTy)]
pub(crate) enum Mode {
    Incoming,
    Outgoing,
}

#[typekin::integral]
pub(crate) struct ModeTagged<const MODE: Mode>(u32);

pub(crate) const OUTGOING_MODE: ModeTagged<{ Mode::Outgoing }> =
    match ModeTagged::try_make(7) {
        Ok(value) => value,
        Err(_) => panic!("valid generic value rejected"),
    };
