#[cfg(test)]
mod tests {
    use crate::constructor::Cfg as MakeCfg;
    use crate::friendship::cfg::Cfg;

    #[test]
    fn accepts_friend_path() {
        let it = "
            friend = crate::convert(crate::source::Source) -> Inspect,
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_friend_and_rel_and_seal() {
        let it = "
            relation = u8,
            friend = crate::convert(crate::source::Source) -> [Inspect, unsafe],
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
            friend = Foo(crate::source::Source) -> Inspect
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_rel_and_friend_and_conv() {
        let it = "
            relation = u8,
            friend = Foo(crate::source::Source) -> [Inspect, unsafe],
            conversion = into_value
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_friend_self() {
        let it = "
            friend = _(Foo) -> Inspect,
            relation = u8
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_friends_empty() {
        let it = "
            friend = []
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_friends_multi() {
        let it = "
            friend = [
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
            friend = convert(Source) -> Make,
            with = [serde]
        ";

        let error = syn::parse_str::<Cfg>(it).unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn rejects_duplicate_capability_for_source() {
        let it = "
            relation = u8,
            friend = [
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
            friend = [
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
            friend = convert(Source) -> Inspect,
            convert(Other) -> Inspect
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_multiple_single_implicit_friends() {
        let it = "
            relation = u8,
            friend = _(Source) -> Inspect,
            _(Other) -> Inspect
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_multiple_default_friends() {
        let it = "
            relation = u8,
            friend = _ -> Inspect,
            _ -> Extra
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_missing_single_friend() {
        let it = "
            relation = u8,
            friend =
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_missing_single_friend_before_attribute() {
        let it = "
            relation = u8,
            friend =,
            seal = Seal
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_conversion_without_source() {
        let it = "
            relation = u8,
            friend = convert() -> Inspect
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_conversion_without_capabilities() {
        let it = "
            relation = u8,
            friend = convert(Source) ->
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_non_identifier_friend_capability() {
        let it = "
            relation = u8,
            friend = convert(Source) -> [Inspect, 1]
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attribute_after_custom_friend() {
        let it = "
            relation = u8,
            friend = Foo(Source) -> [Inspect, unsafe],
            unknown = true
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_duplicate_friends_after_custom_friend() {
        let it = "
            relation = u8,
            friend = Foo(Source) -> Inspect,
            friend = []
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn rejects_conflicting_mod_and_scope() {
        let it = "
            relation = u8,
            mod = protocol,
            scope = protocol,
            friend = _ -> Inspect
        ";
        assert!(syn::parse_str::<Cfg>(it).is_err(), "{it}");
    }

    #[test]
    fn accepts_constructor_friend_with_neighboring_attributes() {
        let it = "
            friend = crate::source::Source,
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
            friend = crate::source::Source,
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
            friend = Source,
            scope = pub(crate) constructors
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_multiple_constructor_friends() {
        let it = "
            relationship = Self::new,
            friend = [crate::source::Source, Other],
            maker = from_source,
            scope = self
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn constructor_friends_are_ordered_and_deduplicated_by_path() {
        let input = "
            relationship = Self::new,
            scope = self,
            friend = [
                crate::z::Source,
                crate::a::Source,
                crate::z::Source,
            ]
        ";
        let parsed = syn::parse_str::<MakeCfg>(input);
        assert!(parsed.is_ok(), "{input}");
        let config = parsed.unwrap();
        let mut friends = config.friends.iter();
        let first = friends.next().unwrap();
        let second = friends.next().unwrap();

        assert_eq!(first.ty.path().segments[1].ident, "a", "{input}");
        assert_eq!(second.ty.path().segments[1].ident, "z", "{input}");
        assert!(friends.next().is_none(), "{input}");
    }

    #[test]
    fn accepts_empty_constructor_friends() {
        let it = "
            relationship = Self::new,
            friend = [],
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
            friend = Source
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");

        let it = "
            relationship = Self::new,
            friend = Source,
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
            friend = Source, Other
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_missing_constructor_friends() {
        let it = "
            relationship = Self::new,
            scope = self,
            friend =
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_missing_constructor_friends_before_maker() {
        let it = "
            relationship = Self::new,
            scope = self,
            friend =,
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
            friend = 1
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_conversion_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friend = convert(Source)
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_bracketed_conversion_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friend = [convert(Source)]
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_implicit_conversion_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friend = _(Source) -> Make
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_constructor_friend_target() {
        let it = "
            relationship = Self::new,
            scope = self,
            friend = Source -> Make
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attribute_after_constructor_friend() {
        let it = "
            relationship = Self::new,
            scope = self,
            friend = Source,
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
            friend = Source,
            friend = []
        ";
        let this = syn::parse_str::<MakeCfg>(it);
        assert!(this.is_err(), "{it}");
    }
}
