#![allow(unused, dead_code)]

use crate::fx_integral_generic::Checked;
use crate::fx_integral_generic::*;
use crate::*;
use core::marker::PhantomData;

#[cfg(test)]
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
    fn nongeneric_declaration_keeps_its_single_value_field() {
        let value = Untagged::make(23);
        let raw = untagged_field(value);

        assert_eq!(raw, 23, "{value:?}");
    }

    #[test]
    fn mixed_parameters_preserve_tuple_field_order() {
        let value = CollisionTagged::make(23);
        let (raw, one, two, three, four, five, six, nine) =
            collision_tagged_fields(value);
        let raw: u16 = raw;
        let _: PhantomData<&()> = one;
        let _: PhantomData<NotTraits> = two;
        let _: PhantomData<NotTraits> = three;
        let _: PhantomData<NotTraits> = four;
        let _: PhantomData<NotTraits> = five;
        let _: PhantomData<NotTraits> = six;
        let _: PhantomData<str> = nine;

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
        let (raw, marker) = tagged_fields(value);
        let raw: u32 = raw;
        let _: PhantomData<str> = marker;

        assert_eq!(raw, 42);
        assert_eq!(size_of::<Tagged<str>>(), size_of::<u32>());
        assert_eq!(align_of::<Tagged<str>>(), align_of::<u32>());
    }

    #[test]
    fn nonzero_storage_keeps_its_niche_with_markers() {
        let result = Positive::<NotTraits>::try_make(7);
        assert!(result.is_ok());
        let value = result.unwrap();
        let storage: core::num::NonZeroU16 = positive_storage(value);

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
        let storage: core::num::Wrapping<u8> = wrapped_storage(result);

        assert_eq!(result.raw(), 0);
        assert_eq!(storage.0, 0);
        assert_eq!(size_of_val(&result), size_of::<u8>());
        assert_eq!(align_of_val(&result), align_of::<u8>());
    }

    #[test]
    fn combined_storage_retains_raw_api_and_layout() {
        let value = STABLE_CHECKED;
        let storage: core::num::Wrapping<core::num::NonZeroU32> =
            checked_storage(value);

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
    fn blind_trust_friend_bypasses_only_construction_validation() {
        let source = BlindTrustFriend::<NotTraits>(3, PhantomData);
        let blind_trust = Checked::<NotTraits>::of(source);
        let rejected = blind_trust.try_map(|raw| raw);
        let repaired = blind_trust.try_map(|raw| raw + 1);

        assert_eq!(blind_trust.raw(), 3);
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

    #[test]
    fn marker_tree_leaves_implement_nested_branch_bounds() {
        fn accepts_root<T: TreeSeal>() {}
        fn accepts_transport<T: Transport>() {}
        fn accepts_direction<T: DirectionBranch>() {}

        accepts_root::<Incoming>();
        accepts_transport::<Incoming>();
        accepts_direction::<Incoming>();
        accepts_root::<Outgoing>();
        accepts_transport::<Outgoing>();
        accepts_direction::<Outgoing>();
    }

    #[test]
    fn marks_and_gaps_combine_within_one_lane() {
        let start = Mark::<Outbound>::make(10);
        let step = Gap::<Outbound>::make(5);
        let moved = start + step;
        let gap = moved - start;
        let doubled = step + step;

        assert_eq!(moved.raw(), 15);
        assert_eq!(gap.raw(), 5);
        assert_eq!(doubled.raw(), 10);
    }

    #[test]
    fn outbound_mark_cannot_add_inbound_gap() {
        assert_not_impl!(Mark<Outbound>: core::ops::Add<Gap<Inbound>>);
    }

    #[test]
    fn inbound_mark_cannot_add_outbound_gap() {
        assert_not_impl!(Mark<Inbound>: core::ops::Add<Gap<Outbound>>);
    }

    #[test]
    fn outbound_mark_cannot_subtract_inbound_gap() {
        assert_not_impl!(Mark<Outbound>: core::ops::Sub<Gap<Inbound>>);
    }

    #[test]
    fn inbound_mark_cannot_subtract_outbound_gap() {
        assert_not_impl!(Mark<Inbound>: core::ops::Sub<Gap<Outbound>>);
    }

    #[test]
    fn outbound_mark_cannot_subtract_inbound_mark() {
        assert_not_impl!(Mark<Outbound>: core::ops::Sub<Mark<Inbound>>);
    }

    #[test]
    fn inbound_mark_cannot_subtract_outbound_mark() {
        assert_not_impl!(Mark<Inbound>: core::ops::Sub<Mark<Outbound>>);
    }

    #[test]
    fn outbound_gap_cannot_add_inbound_gap() {
        assert_not_impl!(Gap<Outbound>: core::ops::Add<Gap<Inbound>>);
    }

    #[test]
    fn inbound_gap_cannot_add_outbound_gap() {
        assert_not_impl!(Gap<Inbound>: core::ops::Add<Gap<Outbound>>);
    }

    #[test]
    fn outbound_gap_cannot_subtract_inbound_gap() {
        assert_not_impl!(Gap<Outbound>: core::ops::Sub<Gap<Inbound>>);
    }

    #[test]
    fn inbound_gap_cannot_subtract_outbound_gap() {
        assert_not_impl!(Gap<Inbound>: core::ops::Sub<Gap<Outbound>>);
    }

    #[test]
    fn marks_cannot_be_added() {
        assert_not_impl!(Mark<Outbound>: core::ops::Add<Mark<Outbound>>);
    }

    #[test]
    fn gaps_cannot_add_marks() {
        assert_not_impl!(Gap<Outbound>: core::ops::Add<Mark<Outbound>>);
    }

    #[test]
    fn gaps_cannot_subtract_marks() {
        assert_not_impl!(Gap<Outbound>: core::ops::Sub<Mark<Outbound>>);
    }

    #[test]
    fn const_arguments_are_part_of_operator_identity() {
        let sum = SpaceTagged::<7>::make(1) + SpaceTagged::<7>::make(2);

        assert_eq!(sum.raw(), 3);
        assert_not_impl!(SpaceTagged<7>: core::ops::Add<SpaceTagged<9>>);
    }

    #[test]
    fn owning_markers_propagate_send_constraints() {
        fn accepts_send<T: Send>() {}

        accepts_send::<OwningMarker<u8>>();
        assert_not_impl!(OwningMarker<std::rc::Rc<()>>: Send);
    }

    #[test]
    fn owning_markers_propagate_sync_constraints() {
        fn accepts_sync<T: Sync>() {}

        accepts_sync::<OwningMarker<u8>>();
        assert_not_impl!(OwningMarker<std::cell::Cell<u8>>: Sync);
    }

    #[test]
    fn enum_const_parameters_preserve_their_declared_type() {
        let raw = OUTGOING_MODE.raw();

        assert_eq!(raw, 7);
    }
}
