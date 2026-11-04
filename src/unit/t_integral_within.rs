//! `Within` friends: typekin's own diagnostics, the expansion-time verdicts
//! on primitive sources, and token details a consumer cannot observe.

#[cfg(test)]
mod expand {
    use syn::ItemStruct;
    use syn::parse::{
        Parse,
        ParseStream,
    };

    use crate::integral::ekran;
    use crate::integral_cfg::Cfg;

    pub(crate) fn expansion(
        config: &str,
        source: &str,
    ) -> syn::Result<String> {
        let config = syn::parse_str::<Cfg>(config)?;
        let item = syn::parse_str::<ItemStruct>(source)?;
        return ekran(Box::new(config), item).map(|it| it.to_string());
    }

    pub(crate) fn expanded(
        config: &str,
        source: &str,
    ) -> String {
        let expansion = expansion(config, source);
        assert!(expansion.is_ok(), "{config}: {:?}", expansion.err());
        return expansion.unwrap();
    }

    pub(crate) fn error(
        config: &str,
        source: &str,
    ) -> String {
        let expansion = expansion(config, source);
        assert!(expansion.is_err(), "{config}: {source}");
        return expansion.unwrap_err().to_string();
    }

    /// A float configuration, parsed for its errors.
    struct FloatCfg;

    impl Parse for FloatCfg {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            return Cfg::parse_float(input).map(|_| Self);
        }
    }

    pub(crate) fn float_error(config: &str) -> String {
        let this = syn::parse_str::<FloatCfg>(config);
        assert!(this.is_err(), "{config}");
        return this.err().unwrap().to_string();
    }

    /// The text from `start` up to the next `# [inline (always)]`.
    pub(crate) fn body_from<'a>(
        expansion: &'a str,
        start: &str,
    ) -> &'a str {
        let at = expansion
            .find(start)
            .unwrap_or_else(|| panic!("no {start}: {expansion}"));
        let rest = &expansion[at..];
        let end = rest[start.len()..]
            .find("# [inline (always)]")
            .map_or(rest.len(), |it| it + start.len());
        return &rest[..end];
    }
}

/// One test per `Within` diagnostic typekin reports during expansion.
#[cfg(test)]
mod diagnostics {
    use super::expand::{
        error,
        float_error,
    };

    #[test]
    fn explicit_conversion_is_rejected() {
        let config = "friend = percent(Percent) -> Within";
        let error = error(config, "pub struct Byte(u16);");

        assert_eq!(
            error,
            "`Within` converts through the source's raw value; write \
            `_(Percent)`"
        );
    }

    #[test]
    fn unsafe_beside_within_is_rejected() {
        let config = "friend = _(Percent) -> [Within, unsafe]";
        let error = error(config, "pub struct Byte(u16);");

        assert_eq!(error, "`Within` proves what `unsafe` trusts; use one");
    }

    #[test]
    fn self_friend_is_rejected() {
        let config = "friend = _(Self) -> Within";
        let error = error(config, "pub struct Byte(u16);");

        assert_eq!(error, "a type is always within itself; remove this friend");
    }

    #[test]
    fn own_name_friend_is_rejected() {
        let config = "friend = _(Byte) -> Within";
        let error = error(config, "pub struct Byte(u16);");

        assert_eq!(error, "a type is always within itself; remove this friend");
    }

    #[test]
    fn facet_friend_is_rejected() {
        let config = "facet = int { friend = _(Percent) -> Within }";
        let error = error(config, "pub struct Byte(u16);");

        assert_eq!(
            error,
            "facet friends only support `Math` and `Bit` capabilities"
        );
    }

    #[test]
    fn float_target_is_rejected() {
        let config = "friend = _(u8) -> Within";
        let error = float_error(config);

        assert_eq!(error, "`Within` supports integer wrappers only");
    }

    #[test]
    fn float_source_is_rejected() {
        let config = "friend = _(f32) -> Within";
        let error = error(config, "pub struct Byte(u64);");

        assert_eq!(error, "`Within` supports integer sources only");
    }

    #[test]
    fn bitflag_root_friend_is_rejected() {
        let config = "friend = _(u8) -> Within";
        let this = syn::parse_str::<crate::bitflag::Cfg>(config);
        assert!(this.is_err(), "{config}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "`Within` belongs in the nested `integral = [...]` friend list"
        );
    }

    #[test]
    fn text_friend_is_rejected() {
        let config = "friend = _(String) -> Within";
        let this = syn::parse_str::<crate::text::Cfg>(config);
        assert!(this.is_err(), "{config}");

        assert_eq!(this.unwrap_err().to_string(), "unknown text capability");
    }
}

/// Primitive sources are decided during expansion, at every pointer width.
#[cfg(test)]
mod primitives {
    use super::expand::{
        error,
        expanded,
    };

    #[test]
    fn eight_bit_source_within_a_byte_is_proven() {
        let config = "domain = ..=255, friend = _(u8) -> Within";
        let expansion = expanded(config, "pub struct Byte(u16);");

        assert!(
            !expansion.contains("__TYPEKIN_WITHIN_PROOF_"),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn domain_refutes_a_wider_source() {
        let config = "domain = ..=255, friend = _(u16) -> Within";
        let error = error(config, "pub struct Byte(u16);");

        assert_eq!(
            error,
            "`u16` is not within `Byte`: 256 is outside the domain `..=255`"
        );
    }

    #[test]
    fn raw_range_refutes_a_wider_source() {
        let config = "friend = _(i16) -> Within";
        let error = error(config, "pub struct Small(i8);");

        assert_eq!(
            error,
            "`i16` is not within `Small`: -32768 does not fit `i8`"
        );
    }

    #[test]
    fn refutation_at_one_width_names_it() {
        let config = "friend = _(usize) -> Within";
        let error = error(config, "pub struct Small(u32);");

        assert_eq!(
            error,
            "`usize` is not within `Small`: 4294967296 does not fit `u32` at \
            64-bit pointer width"
        );
    }

    #[test]
    fn callback_target_cannot_prove_a_wide_source() {
        let config = "valid = is_even, friend = _(u16) -> Within";
        let error = error(config, "pub struct Even(u16);");

        assert_eq!(
            error,
            "cannot prove `u16` within `Even`: `Even`'s `valid = is_even` is \
            opaque to subdomain proofs"
        );
    }

    #[test]
    fn callback_target_proves_an_eight_bit_source_from_its_table() {
        let config = "konst = stable, valid = is_odd, friend = _(u8) -> Within";
        let expansion = expanded(config, "pub struct Odd(u16);");

        assert!(
            expansion.contains("const __TYPEKIN_WITHIN_PROOF_0 : () ="),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn non_const_callback_target_cannot_prove_an_eight_bit_source() {
        let config = "konst = none, valid = is_odd, friend = _(u8) -> Within";
        let error = error(config, "pub struct Odd(u16);");

        assert_eq!(
            error,
            "cannot prove `u8` within `Odd`: `Odd`'s `valid = is_odd` is \
            opaque to subdomain proofs"
        );
    }

    #[test]
    fn walk_above_the_budget_cannot_prove() {
        let config =
            "domain = rem_euclid(1000) in ..200, friend = _(u16) -> Within";
        let error = error(config, "pub struct Low(u16);");

        assert_eq!(
            error,
            "cannot prove `u16` within `Low`: the check needs 201000 steps, \
            above the subdomain proof budget of 65536"
        );
    }
}

/// Generated tokens: the constructor `make` calls, the proof constants and
/// where they are forced, and the constants sources publish.
#[cfg(test)]
mod tokens {
    use super::expand::{
        body_from,
        expanded,
    };

    #[test]
    fn make_constructs_through_the_proven_constructor() {
        let config = "domain = ..=255, friend = _(Percent) -> Within";
        let expansion = expanded(config, "pub struct Byte(u16);");
        let start = expansion.find("impl Make for Percent").unwrap();
        let rest = &expansion[start + 1..];
        let make = &rest[..rest.find(" impl ").unwrap_or(rest.len())];

        assert!(make.contains("return Byte :: _within (raw) ;"), "{make}");
        assert!(!make.contains("_or_panic"), "{make}");
    }

    #[test]
    fn make_is_documented_as_proven() {
        let config = "domain = ..=255, friend = _(Percent) -> Within";
        let expansion = expanded(config, "pub struct Byte(u16);");

        assert!(
            expansion.contains(
                "\" Cannot fail validation: every value of `Percent` lies in \
                the domain of `Byte` (proven by rustc at compile time).\""
            ),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn proven_constructor_checks_in_debug_without_unsafe() {
        let config = "domain = 1.., friend = _(Port) -> Within";
        let expansion = expanded(config, "pub struct Positive(u16);");
        let body = body_from(&expansion, "fn _within (raw : u16) -> Self");

        assert!(body.contains(":: core :: debug_assert !"), "{body}");
        assert!(body.contains("NonZeroU16 :: new (raw)"), "{body}");
        assert!(!body.contains("unsafe"), "{body}");
    }

    #[test]
    fn non_generic_target_forces_its_proof() {
        let config = "domain = ..=255, friend = _(Percent) -> Within";
        let expansion = expanded(config, "pub struct Byte(u16);");

        assert!(
            expansion.contains(
                "const _ : () = < Byte > :: __TYPEKIN_WITHIN_PROOF_0 ;"
            ),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn generic_target_forces_its_proof_in_the_conversion() {
        let config = "domain = ..=255, friend = _(Tagged<D>) -> Within";
        let expansion = expanded(config, "pub struct Byte<D>(u16);");

        assert!(
            expansion.contains(
                "let () = < Byte < D > > :: __TYPEKIN_WITHIN_PROOF_0 ;"
            ),
            "{config}: {expansion}"
        );
        assert!(
            !expansion.contains(":: __TYPEKIN_WITHIN_PROOF_0 ; const"),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn source_publishes_its_form() {
        let config = "domain = ..=100";
        let expansion = expanded(config, "pub struct Percent(u8);");

        assert!(
            expansion.contains(
                "pub const __TYPEKIN_WITHIN_V1_FORM : :: core :: option :: \
                Option < (u128 , & 'static [(u128 , bool , u128 , bool , \
                u128)] ,) > = :: core :: option :: Option :: Some ((1u128 , & \
                [(0u128 , false , 0u128 , false , 100u128)])) ;"
            ),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn pointer_sized_source_publishes_a_form_per_width() {
        let config = "domain = ..=1_000 | 4_294_967_296..";
        let expansion = expanded(config, "pub struct Index(usize);");

        assert_eq!(
            expansion
                .matches("pub const __TYPEKIN_WITHIN_V1_FORM")
                .count(),
            3,
            "{config}: {expansion}"
        );
    }

    #[test]
    fn opaque_source_publishes_its_reason() {
        let config = "domain = count_ones in {3}";
        let expansion = expanded(config, "pub struct Wide(u64);");

        assert!(
            expansion.contains(
                "pub const __TYPEKIN_WITHIN_V1_WHY : & 'static str = \
                \"`Wide`'s domain uses `count_ones`, which is opaque to \
                subdomain proofs\" ;"
            ),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn visible_field_source_publishes_every_value() {
        let config = "hide_in = none, domain = ..=100";
        let expansion = expanded(config, "pub struct Percent(pub u8);");

        assert!(
            expansion.contains(
                "Some ((1u128 , & [(0u128 , false , 0u128 , false , 255u128)]))"
            ),
            "{config}: {expansion}"
        );
        assert!(
            expansion.contains(
                "__TYPEKIN_WITHIN_V1_TABLE : :: core :: option :: Option < \
                [bool ; 256] > = :: core :: option :: Option :: Some ([true ; \
                256]) ;"
            ),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn non_const_callback_source_publishes_no_table() {
        let config = "konst = none, valid = is_odd";
        let expansion = expanded(config, "pub struct Odd(u8);");

        assert!(
            expansion.contains(
                "__TYPEKIN_WITHIN_V1_TABLE : :: core :: option :: Option < \
                [bool ; 256] > = :: core :: option :: Option :: None ;"
            ),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn without_within_source_publishes_nothing() {
        let config = "domain = ..=100, without = within_source";
        let expansion = expanded(config, "pub struct Percent(u8);");

        assert!(
            !expansion.contains("__TYPEKIN_WITHIN_V1_"),
            "{config}: {expansion}"
        );
        assert!(
            !expansion.contains("__typekin_within_v1_raw"),
            "{config}: {expansion}"
        );
    }
}
