#[cfg(test)]
mod tests {
    use crate::constructor::Cfg as MakeCfg;
    use crate::friendship::cfg::Cfg;

    #[test]
    fn accepts_friend_path() {
        let it = "
            friends = crate::convert(crate::source::Source) -> Inspect,
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_friend_and_rel_and_seal() {
        let it = "
            relation = u8,
            friends = crate::convert(crate::source::Source) -> [Inspect, unsafe],
            seal = Seal
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_rel_and_value_and_friend() {
        let it = "
            relation = u8,
            value = u16,
            friends = Foo(crate::source::Source) -> Inspect
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_rel_and_friend_and_conv() {
        let it = "
            relation = u8,
            friends = Foo(crate::source::Source) -> [Inspect, unsafe],
            conversion = into_value
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_friend_self() {
        let it = "
            friends = _(Foo) -> Inspect,
            relation = u8
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_friends_empty() {
        let it = "
            friends = []
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_friends_multi() {
        let it = "
            friends = [
                into_make(Source) -> Make,
                into_bit(Source) -> Bit
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn rejects_unknown_friendship_generation_flags() {
        let it = "
            relation = u8,
            friends = convert(Source) -> Make,
            with = [serde]
        ";

        let error = syn::parse_str::<Cfg>(it).unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn rejects_legacy_self_implicit_conversion_syntax() {
        let it = "
            relation = u8,
            friends = Self(Foo) -> Inspect
        ";

        let error = syn::parse_str::<Cfg>(it).unwrap_err();

        assert_eq!(
            error.to_string(),
            "friend implicit conversion is `_(Type)`, not `Self(Type)`",
        );
    }

    #[test]
    fn rejects_duplicate_capability_for_source() {
        let it = "
            relation = u8,
            friends = [
                into_make(Source) -> Make,
                into_other(Source) -> Make
            ]
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_duplicate_capability_in_multi_capability_friend() {
        let it = "
            relation = u8,
            friends = [
                into(Source) -> [Make, Bit],
                into_other(Source) -> Bit
            ]
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_multiple_single_explicit_friends() {
        let it = "
            relation = u8,
            friends = convert(Source) -> Inspect,
            convert(Other) -> Inspect
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_multiple_single_implicit_friends() {
        let it = "
            relation = u8,
            friends = _(Source) -> Inspect,
            _(Other) -> Inspect
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_multiple_default_friends() {
        let it = "
            relation = u8,
            friends = _ -> Inspect,
            _ -> Extra
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_missing_single_friend() {
        let it = "
            relation = u8,
            friends =
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_missing_single_friend_before_attribute() {
        let it = "
            relation = u8,
            friends =,
            seal = Seal
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_conversion_without_source() {
        let it = "
            relation = u8,
            friends = convert() -> Inspect
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_conversion_without_capabilities() {
        let it = "
            relation = u8,
            friends = convert(Source) ->
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_non_identifier_friend_capability() {
        let it = "
            relation = u8,
            friends = convert(Source) -> [Inspect, 1]
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attribute_after_custom_friend() {
        let it = "
            relation = u8,
            friends = Foo(Source) -> [Inspect, unsafe],
            unknown = true
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_duplicate_friends_after_custom_friend() {
        let it = "
            relation = u8,
            friends = Foo(Source) -> Inspect,
            friends = []
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_conflicting_mod_and_scope() {
        let it = "
            relation = u8,
            mod = protocol,
            scope = protocol,
            friends = _ -> Inspect
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn accepts_constructor_friend_with_neighboring_attributes() {
        let it = "
            friends = crate::source::Source,
            relationship = Self::new,
            scope = self
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_constructor_friend_with_maker() {
        let it = "
            relationship = Self::new,
            friends = crate::source::Source,
            maker = from_source,
            scope = _
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_constructor_friend_with_custom_relationship() {
        let it = "
            relationship = crate::construct,
            maker = from_source,
            friends = Source,
            scope = pub(crate) constructors
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_multiple_constructor_friends() {
        let it = "
            relationship = Self::new,
            friends = [crate::source::Source, Other],
            maker = from_source,
            scope = self
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_empty_constructor_friends() {
        let it = "
            relationship = Self::new,
            friends = [],
            maker = from_source,
            scope = _
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn constructor_requires_scope() {
        let it = "
            relationship = Self::new,
            friends = Source
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");

        let it = "
            relationship = Self::new,
            friends = Source,
            scope = self
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn rejects_unbracketed_multiple_constructor_friends() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends = Source, Other
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_missing_constructor_friends() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends =
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_missing_constructor_friends_before_maker() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends =,
            maker = from_source
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_non_type_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends = 1
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_conversion_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends = convert(Source)
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_bracketed_conversion_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends = [convert(Source)]
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_implicit_conversion_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends = _(Source) -> Make
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_constructor_friend_target() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends = Source -> Make
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attribute_after_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends = Source,
            unknown = true
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_duplicate_constructor_friends() {
        let it = "
            relationship = Self::new,
            scope = self,
            friends = Source,
            friends = []
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }
}
