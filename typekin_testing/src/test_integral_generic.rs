#![allow(unused, dead_code)]

use crate::*;
use core::marker::PhantomData;

struct NotTraits;

#[typekin::integral(with = [display, serde, math_xor])]
struct Tagged<T: ?Sized = NotTraits>(u32);

#[typekin::integral( with = [display, serde])]
struct Mixed<'de, T, F, H, S, D, const SPACE: u8, const FLAG: bool, Meta = str>(
    u16,
)
where
    T: ?Sized,
    Meta: ?Sized;

type CollisionTagged<'a> =
    Mixed<'a, NotTraits, NotTraits, NotTraits, NotTraits, NotTraits, 7, true>;

#[typekin::integral]
struct ConstTagged<const BYTE: u8 = 3, const SYMBOL: char = 'x'>(u32);

#[typekin::integral( domain = 1..)]
struct Positive<T>(u16);

#[typekin::integral( profile = wrapping {})]
struct Wrapped<T>(u8);

#[typekin::integral(without = repr)]
struct ReprFree<T: ?Sized>(u64);

struct CheckedFriend<T>(u32, PhantomData<T>);
struct TrustedFriend<T>(u32, PhantomData<T>);

const fn is_even(value: u32) -> bool {
    return value % 2 == 0;
}

#[typekin::integral(
    domain = 2..=100,
    valid = is_even,
    profile = wrapping {},
    friend = [
        Self::checked_raw(CheckedFriend<T>) -> Make,
        Self::trusted_raw(TrustedFriend<T>) -> [Make, unsafe],
        self(u32) -> [Math, Rel],
    ],
    with = [serde, math_xor],
)]
struct Checked<T>(u32);

impl<T> Checked<T> {
    const fn checked_raw(friend: CheckedFriend<T>) -> u32 {
        return friend.0;
    }

    const fn trusted_raw(friend: TrustedFriend<T>) -> u32 {
        return friend.0;
    }
}

#[typekin::integral(
    out_type = Self::into_tagged(_) -> Tagged<T>,
    with = math_xor,
)]
struct Producing<T>(u32);

impl<T> Producing<T> {
    fn into_tagged(self) -> Tagged<T> {
        return Tagged::make(self.raw());
    }
}

trait OwnIdentity<T> {}
struct OnlySelf;

#[typekin::integral(
    profile = circular {},
    facet = [
        circular {
            friend = Self::delta_raw(Tagged<T>) -> Math,
            out_type = Self,
        },
    ],
)]
struct WhereSelf<T>(u8)
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

mod sealed {
    pub trait Direction {}
}

trait Direction: sealed::Direction {}
struct In;
struct Out;
impl sealed::Direction for In {}
impl sealed::Direction for Out {}
impl Direction for In {}
impl Direction for Out {}

#[typekin::integral(
    profile = [circular, point],
    out_type = Self::into_offset(_) -> Offset<'a, D, SPACE>,
    facet = circular {
        with = math_add,
        friend = Self::offset_raw(Offset<'a, D, SPACE>) -> Math,
        out_type = Self,
    },
)]
struct Point<'a, D: Direction, const SPACE: u8>(u32);

#[typekin::integral( profile = circular { precedes = false })]
struct Offset<'a, D: Direction, const SPACE: u8>(i32);

impl<'a, D: Direction, const SPACE: u8> Point<'a, D, SPACE> {
    fn into_offset(self) -> Offset<'a, D, SPACE> {
        return Offset::make(self.raw() as i32);
    }

    fn offset_raw(offset: Offset<'a, D, SPACE>) -> u32 {
        return offset.raw() as u32;
    }
}

#[typekin::integral(konst = nightly)]
struct ConstValue<T>(u8);

const CONST_CLONED: ConstValue<NotTraits> = ConstValue::make(7).clone();
const CONST_ADDED: ConstValue<NotTraits> =
    ConstValue::make(2) + ConstValue::make(3);
const CONST_MAPPED: ConstValue<NotTraits> =
    ConstValue::make(2).map(const |value| value + 3);
const STABLE_CHECKED: Checked<NotTraits> = match Checked::try_make(4) {
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
struct ConstFacet<T>(u32);

const CONST_FACET_RESULT: ConstFacet<NotTraits> = ConstFacet::make(u32::MAX)
    + match Tagged::<NotTraits>::try_make(2) {
        Ok(value) => value,
        Err(_) => panic!("valid generic value rejected"),
    };

#[typekin::integral]
struct ProtocolNames<Seal, Make, Math, Bit, Rel, UnsafeTrust, T, __TypekinT>(
    u8,
);

type CollisionProtocol = ProtocolNames<
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
    NotTraits,
>;

#[cfg(feature = "test_fast")]
mod tests {
    use super::*;
    use core::mem::{
        align_of,
        align_of_val,
        size_of,
        size_of_val,
    };
    use serde_test::{
        Configure,
        Token,
        assert_de_tokens_error,
        assert_tokens,
    };
    use std::hash::{
        DefaultHasher,
        Hash,
        Hasher,
    };

    #[test]
    fn phantom_markers_do_not_require_copy_or_clone() {
        let original = Tagged::<NotTraits>::make(23);
        let copied = original;
        let cloned = original.clone();

        assert_eq!(original.raw(), copied.raw());
        assert_eq!(cloned.raw(), 23);
    }

    #[test]
    fn mixed_parameters_preserve_tuple_field_order() {
        let value = CollisionTagged::make(23);
        let raw: u16 = value.0;
        let _: PhantomData<&()> = value.1;
        let _: PhantomData<NotTraits> = value.2;
        let _: PhantomData<NotTraits> = value.3;
        let _: PhantomData<NotTraits> = value.4;
        let _: PhantomData<NotTraits> = value.5;
        let _: PhantomData<NotTraits> = value.6;
        let _: PhantomData<str> = value.9;

        assert_eq!(raw, 23);
        assert_eq!(size_of::<CollisionTagged>(), size_of::<u16>());
        assert_eq!(align_of::<CollisionTagged>(), align_of::<u16>());
    }

    #[test]
    fn distinct_const_markers_preserve_const_types_and_defaults() {
        let default: ConstTagged = ConstTagged::make(3);
        let alternate = ConstTagged::<9, '\u{1f980}'>::make(7);

        assert_eq!(default.raw(), 3);
        assert_eq!(alternate.raw(), 7);
        assert_eq!(size_of_val(&alternate), size_of::<u32>());
        assert_eq!(align_of_val(&alternate), align_of::<u32>());
    }

    #[test]
    fn unsized_markers_keep_transparent_layout() {
        let value = Tagged::<str>::make(42);
        let raw: u32 = value.0;
        let _: PhantomData<str> = value.1;

        assert_eq!(raw, 42);
        assert_eq!(size_of::<Tagged<str>>(), size_of::<u32>());
        assert_eq!(align_of::<Tagged<str>>(), align_of::<u32>());
    }

    #[test]
    fn nonzero_storage_keeps_its_niche_with_markers() {
        let result = Positive::<NotTraits>::try_make(7);
        assert!(result.is_ok());
        let value = result.unwrap();
        let storage: core::num::NonZeroU16 = value.0;

        assert_eq!(storage.get(), 7);
        assert_eq!(value.raw(), 7);
        assert_eq!(size_of::<Option<Positive<NotTraits>>>(), size_of::<u16>());
        assert_eq!(align_of::<Positive<NotTraits>>(), align_of::<u16>());
    }

    #[test]
    fn wrapping_storage_preserves_generic_arithmetic() {
        let maximum = Wrapped::<NotTraits>::make(u8::MAX);
        let one = Wrapped::<NotTraits>::make(1);
        let result = maximum + one;
        let storage: core::num::Wrapping<u8> = result.0;

        assert_eq!(result.raw(), 0);
        assert_eq!(storage.0, 0);
        assert_eq!(size_of_val(&result), size_of::<u8>());
        assert_eq!(align_of_val(&result), align_of::<u8>());
    }

    #[test]
    fn combined_storage_retains_raw_api_and_layout() {
        let value = STABLE_CHECKED;
        let storage: core::num::Wrapping<core::num::NonZeroU32> = value.0;

        assert_eq!(storage.0.get(), 4);
        assert_eq!(value.raw(), 4);
        assert_eq!(*value.as_ref(), 4);
        assert_eq!(size_of_val(&value), size_of::<u32>());
        assert_eq!(align_of_val(&value), align_of::<u32>());
    }

    #[test]
    fn layout_assertions_work_without_transparent_repr() {
        let value = ReprFree::<str>::make(9);

        assert_eq!(value.raw(), 9);
        assert_eq!(size_of_val(&value), size_of::<u64>());
        assert_eq!(align_of_val(&value), align_of::<u64>());
    }

    #[test]
    fn formatting_hashing_and_comparison_ignore_marker_traits() {
        let value = Tagged::<NotTraits>::make(42);
        let equivalent = Tagged::<NotTraits>::make(42);
        let larger = Tagged::<NotTraits>::make(43);
        let mut actual = DefaultHasher::new();
        let mut expected = DefaultHasher::new();
        value.hash(&mut actual);
        value.raw().hash(&mut expected);
        let _ = format!("{value:?}");

        assert_eq!(
            format!("{value} {value:b} {value:o} {value:x} {value:X}"),
            "42 101010 52 2a 2A"
        );
        assert_eq!(actual.finish(), expected.finish());
        assert_eq!(value, equivalent);
        assert!(value < larger);
    }

    #[test]
    fn parsing_reconstructs_markers_and_validates() {
        let source = "4";
        let parsed = source.parse::<Checked<NotTraits>>();
        assert!(parsed.is_ok(), "{source}");
        let invalid_source = "3";
        let rejected = invalid_source.parse::<Checked<NotTraits>>();

        assert_eq!(parsed.unwrap().raw(), 4);
        assert!(rejected.is_err(), "{invalid_source}");
    }

    #[test]
    fn serde_uses_only_the_raw_wire_value() {
        let value = Checked::<NotTraits>::try_make(4).unwrap();
        assert_tokens(&value.readable(), &[Token::U32(4)]);
        assert_de_tokens_error::<Checked<NotTraits>>(
            &[Token::U32(3)],
            "invalid value",
        );
    }

    #[test]
    fn common_generic_names_do_not_shadow_serde_or_map_parameters() {
        let value = CollisionTagged::make(8);
        let mapped = value.map(|raw| raw + 1);

        assert_tokens(&mapped.readable(), &[Token::U16(9)]);
    }

    #[test]
    fn checked_friend_construction_consumes_and_validates() {
        let source = CheckedFriend::<NotTraits>(4, PhantomData);
        let value = Checked::<NotTraits>::of(source);
        let invalid = CheckedFriend::<NotTraits>(3, PhantomData);

        assert_eq!(value.raw(), 4);
        assert_panics(|| {
            let _ = Checked::<NotTraits>::of(invalid);
        });
    }

    #[test]
    fn trusted_friend_bypasses_only_construction_validation() {
        let source = TrustedFriend::<NotTraits>(3, PhantomData);
        let trusted = Checked::<NotTraits>::of(source);
        let rejected = trusted.try_map(|raw| raw);
        let repaired = trusted.try_map(|raw| raw + 1);

        assert_eq!(trusted.raw(), 3);
        assert_eq!(rejected.unwrap_err(), 3);
        assert!(repaired.is_ok());
        assert_eq!(repaired.unwrap().raw(), 4);
    }

    #[test]
    fn mapping_validates_the_final_value_once() {
        let source = Checked::<NotTraits>::try_make(4).unwrap();
        let calls = core::cell::Cell::new(0);
        let result = source.try_map(|raw| {
            calls.set(calls.get() + 1);
            return raw + 2;
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().raw(), 6);
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn generic_operations_and_assignments_preserve_validation() {
        let two = Checked::<NotTraits>::try_make(2).unwrap();
        let four = Checked::<NotTraits>::try_make(4).unwrap();
        let mut value = two + four;
        value += two;
        value -= 2u32;
        value *= 2u32;
        value /= 2u32;
        value %= 4u32;
        value |= 4u32;
        value &= 6u32;
        value ^= 2u32;
        value <<= 1usize;
        value >>= 1usize;

        assert_eq!(value.raw(), 4);
        assert_panics(|| {
            let _ = two + 1u32;
        });
        assert_panics(|| {
            let _ = two - two;
        });
    }

    #[test]
    fn core_integer_methods_reconstruct_the_generic_target() {
        let source = Tagged::<NotTraits>::make(0x1234_5678);
        let rotated = source.rotate_left(8);
        let bytes = source.to_be_bytes();
        let reconstructed = Tagged::<NotTraits>::from_be_bytes(bytes);
        let checked = source.checked_add(Tagged::make(1));

        assert_eq!(rotated.raw(), 0x3456_7812);
        assert_eq!(reconstructed.raw(), source.raw());
        assert_eq!(checked.unwrap().raw(), 0x1234_5679);
    }

    #[test]
    fn output_conversions_keep_the_source_marker_identity() {
        let source = Producing::<NotTraits>::make(6);
        let output: Tagged<NotTraits> = source + Producing::make(2);
        let bit_output: Tagged<NotTraits> = source ^ 3u32;
        let shift_output: Tagged<NotTraits> = source << 1usize;

        assert_eq!(output.raw(), 8);
        assert_eq!(bit_output.raw(), 5);
        assert_eq!(shift_output.raw(), 12);
    }

    #[test]
    fn self_predicates_keep_wrapper_meaning_in_foreign_impls() {
        let point = WhereSelf::<OnlySelf>::make(2);
        let delta = Tagged::<OnlySelf>::make(3);
        let result = point + delta;

        assert_eq!(result.raw(), 5);
        assert_eq!(WhereSelf::<OnlySelf>::of(7u8).raw(), 7);
    }

    #[test]
    fn outgoing_facets_preserve_lifetime_type_and_const_arguments() {
        let maximum = Point::<'static, Out, 7>::make(u32::MAX);
        let zero = Point::<'static, Out, 7>::make(0);
        let offset: Offset<'static, Out, 7> = zero - maximum;
        let wrapped: Point<'static, Out, 7> = maximum + offset;
        let restored = wrapped - offset;

        assert_eq!(offset.raw(), 1);
        assert_eq!(wrapped.raw(), 0);
        assert_eq!(restored.raw(), u32::MAX);
    }

    #[test]
    fn incoming_facets_preserve_direction_identity() {
        let source = Point::<'static, In, 9>::make(10);
        let offset = Offset::<'static, In, 9>::make(-3);
        let moved: Point<'static, In, 9> = source + offset;
        let difference: Offset<'static, In, 9> = moved - source;

        assert_eq!(moved.raw(), 7);
        assert_eq!(difference.raw(), -3);
    }

    #[test]
    fn generic_nightly_const_paths_keep_marker_bounds_absent() {
        assert_eq!(CONST_CLONED.raw(), 7);
        assert_eq!(CONST_ADDED.raw(), 5);
        assert_eq!(CONST_MAPPED.raw(), 5);
        assert_eq!(CONST_FACET_RESULT.raw(), 1);
    }

    #[test]
    fn protocol_names_do_not_collide_with_user_parameters() {
        let source = CollisionProtocol::make(8);
        let result = source + CollisionProtocol::of(2u8);
        let mapped = result.map(|raw| raw + 1);

        assert_eq!(mapped.raw(), 11);
        assert!(mapped > source);
    }
}
