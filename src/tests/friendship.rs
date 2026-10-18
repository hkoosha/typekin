#[cfg(test)]
mod tests {
    use crate::constructor::Cfg as MakeCfg;
    use crate::friendship::cfg::Cfg;

    #[test]
    fn accepts_single_named_implicit_and_wildcard_friends_friends_crate_convert_crate_source_source_in_1()
     {
        let input = "friends = crate::convert(crate::source::Source) -> Inspect, relation = u8";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_named_implicit_and_wildcard_friends_relation_u8_friends_crate_convert_crate_sour_2()
     {
        let input = "relation = u8, friends = crate::convert(crate::source::Source) -> [Inspect, unsafe], seal = Seal";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_named_implicit_and_wildcard_friends_relation_u8_value_u16_friends_foo_crate_sour_3()
     {
        let input = "relation = u8, value = u16, friends = Foo(crate::source::Source) -> Inspect";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_named_implicit_and_wildcard_friends_relation_u8_friends_foo_crate_source_source_case_4()
     {
        let input = "relation = u8, friends = Foo(crate::source::Source) -> [Inspect, unsafe], conversion = into_value";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_named_implicit_and_wildcard_friends_friends_self_foo_inspect_relation_u8_5()
     {
        let input = "friends = Self(Foo) -> Inspect, relation = u8";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_named_implicit_and_wildcard_friends_relation_u8_friends_self_foo_inspect_trust_v_6()
     {
        let input = "relation = u8, friends = Self(Foo) -> [Inspect, unsafe], value = u16";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_named_implicit_and_wildcard_friends_relation_u8_scope_pub_crate_protocol_friends_7()
     {
        let input = "relation = u8, scope = pub(crate) protocol, friends = Self(Foo) -> Inspect";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friend_lists_relation_u8_friends_seal_seal_1()
     {
        let input = "relation = u8, friends = [], seal = Seal";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friend_lists_relation_u8_friends_into_make_source_make_in_2()
     {
        let input = "relation = u8, friends = [into_make(Source) -> Make, into_bit(Source) -> Bit]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn rejects_unknown_friendship_generation_flags() {
        let error = match syn::parse_str::<Cfg>(
            "relation = u8, friends = convert(Source) -> Make, with = [serde]",
        ) {
            Ok(_) => panic!("friendship cannot infer a serde representation"),
            Err(error) => error,
        };

        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn rejects_overlapping_capabilities_for_a_source_type_relation_u8_friends_into_make_source_make_in_1()
     {
        let input = "relation = u8, friends = [into_make(Source) -> Make, into_other(Source) -> Make]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_overlapping_capabilities_for_a_source_type_relation_u8_friends_into_source_make_bit_int_2()
     {
        let input = "relation = u8, friends = [into(Source) -> [Make, Bit], into_other(Source) -> Bit]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_convert_source_inspect_c_1()
     {
        let input = "relation = u8, friends = convert(Source) -> Inspect, convert(Other) -> Inspect";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_source_inspect_other_ins_2()
     {
        let input = "relation = u8, friends = _(Source) -> Inspect, _(Other) -> Inspect";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_foo_source_inspect_foo_o_3()
     {
        let input = "relation = u8, friends = Foo(Source) -> Inspect, Foo(Other) -> Inspect";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_inspect_extra_4()
     {
        let input = "relation = u8, friends = _ -> Inspect, _ -> Extra";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_5() {
        let input = "relation = u8, friends =";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_seal_seal_6()
     {
        let input = "relation = u8, friends =, seal = Seal";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_convert_inspect_7()
     {
        let input = "relation = u8, friends = convert() -> Inspect";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_convert_source_8()
     {
        let input = "relation = u8, friends = convert(Source) ->";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_convert_source_inspect_1_9()
     {
        let input = "relation = u8, friends = convert(Source) -> [Inspect, 1]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_foo_source_inspect_trust_10()
     {
        let input = "relation = u8, friends = Foo(Source) -> [Inspect, unsafe], unknown = true";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_foo_source_inspect_frien_11()
     {
        let input =
            "relation = u8, friends = Foo(Source) -> Inspect, friends = []";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_source_inspect_trust_unk_12()
     {
        let input = "relation = u8, friends = _(Source) -> [Inspect, unsafe], unknown = true";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_friends_source_inspect_friends_13()
     {
        let input =
            "relation = u8, friends = _(Source) -> Inspect, friends = []";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_relation_u8_mod_protocol_scope_protocol_frie_14()
     {
        let input = "relation = u8, mod = protocol, scope = protocol, friends = _ -> Inspect";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn accepts_single_constructor_type_with_neighboring_attributes_friends_crate_source_source_relationship_sel_1()
     {
        let input = "friends = crate::source::Source, relationship = Self::new, scope = self";
        syn::parse_str::<MakeCfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_constructor_type_with_neighboring_attributes_relationship_self_new_friends_crate_source_s_2()
     {
        let input = "relationship = Self::new, friends = crate::source::Source, maker = from_source, scope = _";
        syn::parse_str::<MakeCfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_constructor_type_with_neighboring_attributes_relationship_crate_construct_maker_from_sour_3()
     {
        let input = "relationship = crate::construct, maker = from_source, friends = Source, scope = pub(crate) constructors";
        syn::parse_str::<MakeCfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_constructor_types_relationship_self_new_friends_crate_source_s_1()
     {
        let input = "relationship = Self::new, friends = [crate::source::Source, Other], maker = from_source, scope = self";
        syn::parse_str::<MakeCfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_constructor_types_relationship_self_new_friends_maker_from_sou_2()
     {
        let input = "relationship = Self::new, friends = [], maker = from_source, scope = _";
        syn::parse_str::<MakeCfg>(input).expect(input);
    }

    #[test]
    fn constructor_requires_scope() {
        assert!(
            syn::parse_str::<MakeCfg>(
                "relationship = Self::new, friends = Source"
            )
            .is_err()
        );
        assert!(
            syn::parse_str::<MakeCfg>(
                "relationship = Self::new, friends = Source, scope = self",
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_sou_1()
     {
        let input =
            "relationship = Self::new, scope = self, friends = Source, Other";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_2()
     {
        let input = "relationship = Self::new, scope = self, friends =";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_mak_3()
     {
        let input = "relationship = Self::new, scope = self, friends =, maker = from_source";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_1_4()
     {
        let input = "relationship = Self::new, scope = self, friends = 1";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_con_5()
     {
        let input =
            "relationship = Self::new, scope = self, friends = convert(Source)";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_con_6()
     {
        let input = "relationship = Self::new, scope = self, friends = [convert(Source)]";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_con_7()
     {
        let input = "relationship = Self::new, scope = self, friends = convert(Source) -> Make";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_con_8()
     {
        let input = "relationship = Self::new, scope = self, friends = [convert(Source) -> Make]";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_foo_9()
     {
        let input = "relationship = Self::new, scope = self, friends = Foo(Source) -> Make";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_foo_10()
     {
        let input =
            "relationship = Self::new, scope = self, friends = Foo -> Make";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_sou_11()
     {
        let input = "relationship = Self::new, scope = self, friends = _(Source) -> Make";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_mak_12()
     {
        let input =
            "relationship = Self::new, scope = self, friends = _ -> Make";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_sou_13()
     {
        let input =
            "relationship = Self::new, scope = self, friends = Source -> Make";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_sou_14()
     {
        let input = "relationship = Self::new, scope = self, friends = Source, unknown = true";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors_relationship_self_new_scope_self_friends_sou_15()
     {
        let input = "relationship = Self::new, scope = self, friends = Source, friends = []";
        assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
    }
}
