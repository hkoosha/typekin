//! `hide_in`: types live in a private module and are re-exported; the
//! defining module keeps the generated API but not the tuple constructor.

use std::marker::PhantomData;

use crate::fx_hide::*;
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validated_construction_uses_private_parent_callback() {
        let even = Even::try_make(4);
        let odd = Even::try_make(3);

        assert_eq!(even.map(Even::raw), Ok(4));
        assert_eq!(odd.map(Even::raw), Err(3));
    }

    #[test]
    fn friend_conversion_resolves_private_parent_items() {
        let raw = 6;
        let even = even_of_source(raw);

        assert_eq!(even.raw(), raw);
    }

    #[test]
    fn self_produce_callback_resolves_parent_impl() {
        let lhs = Narrow::make(2);
        let rhs = Narrow::make(3);
        let wide: Wide = lhs + rhs;

        assert_eq!(wide.raw(), 5);
    }

    #[test]
    fn generic_tree_type_is_re_exported_with_its_nodes() {
        let meters = Length::<Meters>::make(3);
        let feet = Length::<Feet>::make(3);

        assert_eq!(meters.raw(), feet.raw());
        assert_eq!(size_of::<Length<Meters>>(), size_of::<u32>());
        let _: PhantomData<Feet> = PhantomData;
    }

    #[test]
    fn named_module_hides_and_re_exports() {
        let value = Renamed::make(7);

        assert_eq!(value.raw(), 7);
        assert_eq!(renamed_in_custom_module(1).raw(), 1);
    }

    #[test]
    fn bitflag_enum_and_value_are_re_exported() {
        let both = Access::Read.inserted(Access::Write);

        assert_eq!(both.raw(), 0b11);
        assert_eq!(AccessValue::try_make(0b01).map(AccessValue::raw), Ok(1));
    }

    #[test]
    fn text_type_is_re_exported() {
        let stage = Stage::try_from_str("draft");
        let other = Stage::try_from_str("other");

        assert_eq!(stage.map(|it| it.into_inner()), Ok("draft".to_owned()));
        assert!(other.is_err());
    }

    #[test]
    fn super_qualified_callback_is_re_anchored() {
        let small = nested::try_make_small(3);
        let large = nested::try_make_small(30);

        assert_eq!(small, Ok(3));
        assert_eq!(large, Err(30));
    }

    #[test]
    fn self_qualified_callback_is_re_anchored() {
        let even = SelfChecked::try_make(4);
        let odd = SelfChecked::try_make(3);

        assert_eq!(even.map(SelfChecked::raw), Ok(4));
        assert_eq!(odd.map(SelfChecked::raw), Err(3));
    }

    #[test]
    fn crate_qualified_callback_is_kept() {
        let even = CrateChecked::try_make(4);
        let odd = CrateChecked::try_make(3);

        assert_eq!(even.map(CrateChecked::raw), Ok(4));
        assert_eq!(odd.map(CrateChecked::raw), Err(3));
    }

    #[test]
    fn super_qualified_friend_type_is_re_anchored() {
        let raw = 7;
        let relayed = nested::relayed_of_source(raw);

        assert_eq!(relayed, raw);
    }

    #[test]
    fn tree_node_restricted_by_an_ancestor_path_keeps_its_reach() {
        nested::far_has_the_reach_visibility();
    }

    #[test]
    fn field_restricted_by_an_ancestor_path_keeps_its_reach() {
        let raw = 9;
        let item = nested::shelf_item(raw);

        assert_eq!(item, raw);
    }

    #[test]
    fn default_module_is_dunder_snake_case() {
        let header = page_header_in_default_module(1);
        let code = http_code_in_default_module(200);

        assert_eq!(header.raw(), 1);
        assert_eq!(code.raw(), 200);
    }

    #[test]
    fn default_module_strips_raw_identifier_prefix() {
        let raw = "typed";
        let value = type_in_default_module(raw);

        assert_eq!(value.map(|it| it.into_inner()), Ok(raw.to_owned()));
    }

    #[test]
    fn raw_none_names_a_module() {
        let value = raw_none_in_none_module(4);

        assert_eq!(value.raw(), 4);
    }

    #[test]
    fn hide_in_none_keeps_the_item_in_the_defining_module() {
        let raw = 5;
        let exposed = exposed_from_constructor(raw);

        assert_eq!(exposed_field(&exposed), raw);
        assert_eq!(exposed.raw(), raw);
    }
}
