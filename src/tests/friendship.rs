use crate::{
    constructor::Cfg as MakeCfg,
    friendship::cfg::Cfg,
};

#[test]
fn accepts_single_named_implicit_and_wildcard_friends() {
    for input in [
        "friends = crate::convert(crate::source::Source) -> Inspect, relation = u8",
        "relation = u8, friends = crate::convert(crate::source::Source) -> [Inspect, Trust], seal = Seal",
        "relation = u8, value = u16, friends = _(crate::source::Source) -> Inspect",
        "relation = u8, friends = _(crate::source::Source) -> [Inspect, Trust], conversion = into_value",
        "friends = _ -> Inspect, relation = u8",
        "relation = u8, friends = _ -> [Inspect, Trust], value = u16",
    ] {
        syn::parse_str::<Cfg>(input).expect(input);
    }
}

#[test]
fn accepts_bracketed_multiple_and_empty_friend_lists() {
    for input in [
        "relation = u8, friends = [convert(Source) -> Inspect, _(Other) -> [Inspect, Trust], _ -> Extra], seal = Seal",
        "relation = u8, friends = [], seal = Seal",
    ] {
        syn::parse_str::<Cfg>(input).expect(input);
    }
}

#[test]
fn rejects_invalid_single_friend_boundaries() {
    for input in [
        "relation = u8, friends = convert(Source) -> Inspect, convert(Other) -> Inspect",
        "relation = u8, friends = _(Source) -> Inspect, _(Other) -> Inspect",
        "relation = u8, friends = _ -> Inspect, _ -> Extra",
        "relation = u8, friends =",
        "relation = u8, friends =, seal = Seal",
        "relation = u8, friends = convert(Source)",
        "relation = u8, friends = convert() -> Inspect",
        "relation = u8, friends = convert(Source) ->",
        "relation = u8, friends = convert(Source) -> [Inspect, 1]",
        "relation = u8, friends = _(Source) -> [Inspect, Trust], unknown = true",
        "relation = u8, friends = _(Source) -> Inspect, friends = []",
    ] {
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}

#[test]
fn accepts_single_constructor_type_with_neighboring_attributes() {
    for input in [
        "friends = crate::source::Source, of_relation = Self::new, mod = self",
        "of_relation = Self::new, friends = crate::source::Source, of_friend = from_source, mod = _",
        "of_relation = crate::construct, of_friend = from_source, friends = Source, mod = pub(crate) constructors",
    ] {
        syn::parse_str::<MakeCfg>(input).expect(input);
    }
}

#[test]
fn accepts_bracketed_multiple_and_empty_constructor_types() {
    for input in [
        "of_relation = Self::new, friends = [crate::source::Source, Other], of_friend = from_source, mod = self",
        "of_relation = Self::new, friends = [], of_friend = from_source, mod = _",
    ] {
        syn::parse_str::<MakeCfg>(input).expect(input);
    }
}

#[test]
fn friendship_controls_constructor_scope() {
    assert!(
        syn::parse::Parser::parse_str(
            MakeCfg::parse_without_scope,
            "of_relation = Self::new, friends = Source",
        )
        .is_ok()
    );
    assert!(
        syn::parse::Parser::parse_str(
            MakeCfg::parse_without_scope,
            "of_relation = Self::new, friends = Source, mod = self",
        )
        .is_err()
    );
}

#[test]
fn rejects_non_type_or_unbracketed_multiple_constructor_selectors() {
    for input in [
        "of_relation = Self::new, friends = Source, Other",
        "of_relation = Self::new, friends = Source",
        "of_relation = Self::new, friends =",
        "of_relation = Self::new, friends =, of_friend = from_source",
        "of_relation = Self::new, friends = 1",
        "of_relation = Self::new, friends = convert(Source)",
        "of_relation = Self::new, friends = [convert(Source)]",
        "of_relation = Self::new, friends = convert(Source) -> Make",
        "of_relation = Self::new, friends = [convert(Source) -> Make]",
        "of_relation = Self::new, friends = _(Source) -> Make",
        "of_relation = Self::new, friends = _ -> Make",
        "of_relation = Self::new, friends = Source -> Make",
        "of_relation = Self::new, friends = Source, unknown = true",
        "of_relation = Self::new, friends = Source, friends = []",
    ] {
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }
}
