#[cfg(test)]
mod test {
    use crate::bitflag::BitflagCfg;

    #[test]
    fn accepts_top_level_konst_values() {
        let config = syn::parse_str::<BitflagCfg>("konst = true")
            .expect("top-level true konst should parse");
        assert!(config.int.konst);

        let config = syn::parse_str::<BitflagCfg>("konst = false")
            .expect("top-level false konst should parse");
        assert!(!config.int.konst);
    }

    #[test]
    fn rejects_missing_konst() {
        let error =
            match syn::parse_str::<BitflagCfg>("friends = [u8(conv = self)]") {
                Ok(_) => panic!("konst must be explicit"),
                Err(error) => error,
            };
        assert_eq!(error.to_string(), "missing required `konst` argument",);
    }

    #[test]
    fn rejects_friendship_trait_configuration() {
        let error = match syn::parse_str::<BitflagCfg>(
            "with = [trait_seal], integral = [konst = false]",
        ) {
            Ok(_) => panic!("bitflag must delegate friendship trait names"),
            Err(error) => error,
        };

        assert_eq!(error.to_string(), "unknown flag");
    }

    #[test]
    fn accepts_friend_capabilities_and_rejects_levels() {
        let config = syn::parse_str::<BitflagCfg>(
            "friends = [u8(conv = self, cap = [Bit])], integral = [konst = false]",
        )
        .expect("bitflag capabilities should parse");
        let friend = config.friends.iter().next().unwrap();
        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));

        let level = match syn::parse_str::<BitflagCfg>(
            "friends = [u8(conv = self, level = [Bit])], integral = [konst = false]",
        ) {
            Ok(_) => panic!("levels were replaced by capabilities"),
            Err(error) => error,
        };
        assert_eq!(level.to_string(), "unknown attribute");
    }
}
