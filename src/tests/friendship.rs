#[cfg(test)]
mod tests {
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
            "relation = u8, scope = pub(crate) protocol, friends = _ -> Inspect",
        ] {
            syn::parse_str::<Cfg>(input).expect(input);
        }
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friend_lists() {
        for input in [
            "relation = u8, friends = [convert(Source) -> Inspect, _(Other) -> [Inspect, Trust], _ -> Extra], seal = Seal",
            "relation = u8, friends = [], seal = Seal",
            "relation = u8, friends = [into_make(Source) -> Make, into_bit(Source) -> Bit]",
        ] {
            syn::parse_str::<Cfg>(input).expect(input);
        }
    }

    #[test]
    fn rejects_overlapping_capabilities_for_a_source_type() {
        for input in [
            "relation = u8, friends = [into_make(Source) -> Make, into_other(Source) -> Make]",
            "relation = u8, friends = [into(Source) -> [Make, Bit], into_other(Source) -> Bit]",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
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
            "relation = u8, mod = protocol, scope = protocol, friends = _ -> Inspect",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn accepts_single_constructor_type_with_neighboring_attributes() {
        for input in [
            "friends = crate::source::Source, relationship = Self::new, scope = self",
            "relationship = Self::new, friends = crate::source::Source, maker = from_source, scope = _",
            "relationship = crate::construct, maker = from_source, friends = Source, scope = pub(crate) constructors",
        ] {
            syn::parse_str::<MakeCfg>(input).expect(input);
        }
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_constructor_types() {
        for input in [
            "relationship = Self::new, friends = [crate::source::Source, Other], maker = from_source, scope = self",
            "relationship = Self::new, friends = [], maker = from_source, scope = _",
        ] {
            syn::parse_str::<MakeCfg>(input).expect(input);
        }
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
    fn rejects_non_type_or_unbracketed_multiple_constructor_selectors() {
        for input in [
            "relationship = Self::new, scope = self, friends = Source, Other",
            "relationship = Self::new, scope = self, friends =",
            "relationship = Self::new, scope = self, friends =, maker = from_source",
            "relationship = Self::new, scope = self, friends = 1",
            "relationship = Self::new, scope = self, friends = convert(Source)",
            "relationship = Self::new, scope = self, friends = [convert(Source)]",
            "relationship = Self::new, scope = self, friends = convert(Source) -> Make",
            "relationship = Self::new, scope = self, friends = [convert(Source) -> Make]",
            "relationship = Self::new, scope = self, friends = _(Source) -> Make",
            "relationship = Self::new, scope = self, friends = _ -> Make",
            "relationship = Self::new, scope = self, friends = Source -> Make",
            "relationship = Self::new, scope = self, friends = Source, unknown = true",
            "relationship = Self::new, scope = self, friends = Source, friends = []",
        ] {
            assert!(syn::parse_str::<MakeCfg>(input).is_err(), "{input}");
        }
    }
}
