#[cfg(test)]
mod tests {
    use quote::{
        format_ident,
        quote,
    };
    use syn::parse::{
        Parse,
        ParseStream,
    };

    use crate::integral::{
        StoragePlan,
        ekran,
        ekran_with,
    };
    use crate::integral_cfg::{
        Cfg,
        IntegralFlag,
    };
    use crate::zz_domain_float::MathProvider;
    use crate::zz_lib::Named as _;
    use crate::zz_profile_integral::ProfileKind;
    use crate::zz_values::{
        F,
        FnName,
        N,
        Num,
    };

    struct FloatCfg(Cfg);

    impl Parse for FloatCfg {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            return Cfg::parse_float(input).map(Self);
        }
    }

    fn config(input: &str) -> Cfg {
        let this = syn::parse_str::<FloatCfg>(input);
        assert!(this.is_ok(), "{input}: {:?}", this.as_ref().err());
        return this.unwrap().0;
    }

    fn config_error(input: &str) -> String {
        let this = syn::parse_str::<FloatCfg>(input);
        assert!(this.is_err(), "{input}");
        return this.err().unwrap().to_string();
    }

    fn resolved(
        input: &str,
        subject: F,
    ) -> Cfg {
        let mut config = config(input);
        let math = config.math;
        let this = config.validation.resolve(Num::Float(subject), math);
        assert!(this.is_ok(), "{input}: {:?}", this.err());
        return config;
    }

    fn resolve_error(
        input: &str,
        subject: F,
    ) -> String {
        let mut config = config(input);
        let math = config.math;
        let this = config.validation.resolve(Num::Float(subject), math);
        assert!(this.is_err(), "{input}");
        return this.unwrap_err().to_string();
    }

    fn emitted(
        input: &str,
        subject: F,
    ) -> String {
        let validation =
            resolved(input, subject).validation.emit(&quote! { it });
        return quote! { #validation }.to_string();
    }

    fn expansion_of(
        input: &str,
        subject: F,
    ) -> String {
        let config = config(input);
        let expansion = ekran_with(
            format_ident!("Value"),
            format_ident!("{}", subject.name()),
            Box::new(config),
            StoragePlan::primitive(subject),
        );
        assert!(expansion.is_ok(), "{input}: {:?}", expansion.as_ref().err());
        return expansion.unwrap().to_string();
    }

    fn expansion_error(
        input: &str,
        subject: F,
    ) -> String {
        let config = config(input);
        let expansion = ekran_with(
            format_ident!("Value"),
            format_ident!("{}", subject.name()),
            Box::new(config),
            StoragePlan::primitive(subject),
        );
        assert!(expansion.is_err(), "{input}");
        return expansion.unwrap_err().to_string();
    }

    fn item_error(
        input: &str,
        source: &str,
    ) -> String {
        let config = config(input);
        let item = syn::parse_str::<syn::ItemStruct>(source);
        assert!(item.is_ok(), "{source}");
        let expansion = ekran(Box::new(config), item.unwrap());
        assert!(expansion.is_err(), "{input}: {source}");
        return expansion.unwrap_err().to_string();
    }

    fn query(
        input: &str,
        subject: F,
        function: FnName,
    ) -> Option<bool> {
        let config = resolved(input, subject);
        let domain = config.validation.float_domain();
        assert!(domain.is_some(), "{input}");
        return domain.unwrap().query(function);
    }

    fn integral_expansion(
        input: &str,
        subject: N,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        return ekran_with(
            format_ident!("Value"),
            format_ident!("{}", subject.name()),
            Box::new(this.unwrap()),
            StoragePlan::primitive(subject),
        );
    }

    // -------------------------------------------------------------------------
    // Domain forms

    #[test]
    fn closed_range_emits_both_bounds() {
        let input = "domain = 0.0..=1.0";
        let emitted = emitted(input, F::F064);

        assert_eq!(emitted, "((it >= (0.0)) && (it <= (1.0)))", "{input}");
    }

    #[test]
    fn type_bound_constant_emits_through_core() {
        let input = "domain = ..f64::MAX";
        let emitted = emitted(input, F::F064);

        assert_eq!(
            emitted, "(it < (:: core :: primitive :: f64 :: MAX))",
            "{input}"
        );
    }

    #[test]
    fn infinity_bound_is_folded_away() {
        let input = "domain = 0.0..=f64::INFINITY";
        let emitted = emitted(input, F::F064);

        assert_eq!(emitted, "(it >= (0.0))", "{input}");
    }

    #[test]
    fn std_consts_path_emits_through_core() {
        let input = "domain = 0.0..std::f64::consts::TAU";
        let emitted = emitted(input, F::F064);

        assert_eq!(
            emitted,
            "((it >= (0.0)) && (it < (:: core :: f64 :: consts :: TAU)))",
            "{input}"
        );
    }

    #[test]
    fn not_nan_predicate_is_emitted() {
        let input = "domain = !is_nan";
        let emitted = emitted(input, F::F064);

        assert_eq!(emitted, "(! ((it) . is_nan ()))", "{input}");
    }

    #[test]
    fn finite_range_drops_redundant_is_finite() {
        let input = "domain = 0.0..=1.0 & is_finite";
        let emitted = emitted(input, F::F064);

        assert_eq!(emitted, "((it >= (0.0)) && (it <= (1.0)))", "{input}");
    }

    // -------------------------------------------------------------------------
    // Domain rejections

    #[test]
    fn integer_literal_is_rejected() {
        let input = "domain = 1..2";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("float literals; write 1.0"), "{error}");
    }

    #[test]
    fn mismatched_suffix_is_rejected() {
        let input = "domain = 1.0f32..";
        let error = resolve_error(input, F::F064);

        assert!(
            error.contains("f32 suffix does not match the f64"),
            "{error}"
        );
    }

    #[test]
    fn mismatched_type_constant_is_rejected() {
        let input = "domain = ..f32::MAX";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("does not match the f64 operand"), "{error}");
    }

    #[test]
    fn nan_constant_is_rejected() {
        let input = "domain = ..f64::NAN";
        let error = config_error(input);

        assert!(error.contains("NaN never compares equal"), "{error}");
    }

    #[test]
    fn overflowing_constant_is_rejected() {
        let input = "domain = ..1e40";
        let error = resolve_error(input, F::F032);

        assert!(error.contains("1e40 overflows f32 to infinity"), "{error}");
    }

    #[test]
    fn inexact_divisor_is_rejected() {
        let input = "domain = mul(0.1)";
        let error = resolve_error(input, F::F064);

        assert!(
            error.contains("0.1 is not exactly representable as f64"),
            "{error}"
        );
    }

    #[test]
    fn zero_divisor_is_rejected() {
        let input = "domain = mul(0.0)";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("division by zero"), "{error}");
    }

    #[test]
    fn empty_range_is_rejected() {
        let input = "domain = 1.0..1.0";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("this range is empty"), "{error}");
    }

    #[test]
    fn unsatisfiable_domain_is_rejected() {
        let input = "domain = ..0.0 & 1.0.. & !{1.0}";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("domain admits no f64 value"), "{error}");
    }

    #[test]
    fn unknown_function_lists_available_ones() {
        let input = "domain = foo in ..=1.0";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("unknown domain function foo"), "{error}");
        assert!(error.contains("abs"), "{error}");
    }

    #[test]
    fn provider_function_needs_a_math_provider() {
        let input = "domain = sqrt in ..=2.0";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("sqrt needs math = std"), "{error}");
    }

    #[test]
    fn std_provider_function_calls_the_inherent_method() {
        let input = "math = std, domain = sqrt in ..=2.0";
        let emitted = emitted(input, F::F064);

        assert_eq!(emitted, "((it) . sqrt () <= (2.0))", "{input}");
    }

    #[test]
    fn provider_function_resolves_with_nightly_core_math() {
        let input = "math = nightly, domain = sqrt in ..=2.0";
        let emitted = emitted(input, F::F032);

        assert_eq!(
            emitted, "(:: core :: f32 :: math :: sqrt (it) <= (2.0))",
            "{input}"
        );
    }

    #[test]
    fn powi_is_rejected() {
        let input = "math = std, domain = powi in ..=2.0";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("powi is not supported"), "{error}");
    }

    #[test]
    fn predicate_used_as_a_lens_is_rejected() {
        let input = "domain = is_nan in {1.0}";
        let error = resolve_error(input, F::F064);

        assert!(error.contains("is_nan is a predicate"), "{error}");
    }

    // -------------------------------------------------------------------------
    // NaN-freeness

    #[test]
    fn nan_freeness_matches_the_resolved_probe() {
        let input = "domain = (0.0..1.0 | 10.0..) & !{0.5}";
        let config = resolved(input, F::F064);
        let domain = config.validation.float_domain().unwrap();

        assert!(!config.validation.admits_nan(), "{input}");
        assert!(!domain.admits_nan(), "{input}");
    }

    // -------------------------------------------------------------------------
    // Query answers

    #[test]
    fn bounded_range_decides_nan_and_finiteness() {
        let input = "domain = 0.0..=1.0";

        assert_eq!(
            query(input, F::F064, FnName::IsNan),
            Some(false),
            "{input}"
        );
        assert_eq!(
            query(input, F::F064, FnName::IsInfinite),
            Some(false),
            "{input}"
        );
        assert_eq!(
            query(input, F::F064, FnName::IsFinite),
            Some(true),
            "{input}"
        );
    }

    #[test]
    fn range_from_zero_leaves_sign_open() {
        let input = "domain = 0.0..=1.0";

        assert_eq!(
            query(input, F::F064, FnName::IsSignPositive),
            None,
            "{input}"
        );
        assert_eq!(
            query(input, F::F064, FnName::IsSignNegative),
            None,
            "{input}"
        );
    }

    #[test]
    fn range_from_zero_leaves_normality_open() {
        let input = "domain = 0.0..=1.0";

        assert_eq!(query(input, F::F064, FnName::IsNormal), None, "{input}");
        assert_eq!(query(input, F::F064, FnName::IsSubnormal), None, "{input}");
    }

    #[test]
    fn positive_range_decides_sign() {
        let input = "domain = 1.0..=2.0";

        assert_eq!(
            query(input, F::F064, FnName::IsSignPositive),
            Some(true),
            "{input}"
        );
        assert_eq!(
            query(input, F::F064, FnName::IsSignNegative),
            Some(false),
            "{input}"
        );
    }

    #[test]
    fn normal_range_decides_normality() {
        let input = "domain = 1.0..=2.0";

        assert_eq!(
            query(input, F::F064, FnName::IsNormal),
            Some(true),
            "{input}"
        );
        assert_eq!(
            query(input, F::F064, FnName::IsSubnormal),
            Some(false),
            "{input}"
        );
    }

    #[test]
    fn excluded_zero_decides_sign_but_not_normality() {
        let input = "domain = 0.0.. & !{0.0}";

        assert_eq!(
            query(input, F::F064, FnName::IsSignPositive),
            Some(true),
            "{input}"
        );
        assert_eq!(query(input, F::F064, FnName::IsNormal), None, "{input}");
    }

    #[test]
    fn subnormal_range_decides_subnormality() {
        let input = "domain = 0.0.. & !{0.0} & ..f64::MIN_POSITIVE";

        assert_eq!(
            query(input, F::F064, FnName::IsSubnormal),
            Some(true),
            "{input}"
        );
        assert_eq!(
            query(input, F::F064, FnName::IsNormal),
            Some(false),
            "{input}"
        );
    }

    #[test]
    fn min_positive_bound_excludes_f32_subnormals() {
        let input = "domain = f32::MIN_POSITIVE..=1.0";

        assert_eq!(
            query(input, F::F032, FnName::IsSubnormal),
            Some(false),
            "{input}"
        );
    }

    #[test]
    fn f32_subnormal_bound_leaves_subnormality_open() {
        let input = "domain = 1.0e-39..=1.0";

        assert_eq!(query(input, F::F032, FnName::IsSubnormal), None, "{input}");
    }

    #[test]
    fn f64_normal_bound_decides_subnormality() {
        let input = "domain = 1.0e-39..=1.0";

        assert_eq!(
            query(input, F::F064, FnName::IsSubnormal),
            Some(false),
            "{input}"
        );
    }

    #[test]
    fn open_range_leaves_finiteness_open() {
        let input = "domain = 1.0..";

        assert_eq!(
            query(input, F::F064, FnName::IsNan),
            Some(false),
            "{input}"
        );
        assert_eq!(query(input, F::F064, FnName::IsFinite), None, "{input}");
        assert_eq!(query(input, F::F064, FnName::IsInfinite), None, "{input}");
    }

    #[test]
    fn not_equal_leaves_nan_open() {
        let input = "domain = !{0.5}";

        assert_eq!(query(input, F::F064, FnName::IsNan), None, "{input}");
    }

    #[test]
    fn negated_predicates_decide_finiteness() {
        let input = "domain = !is_nan & !is_infinite";

        assert_eq!(
            query(input, F::F064, FnName::IsFinite),
            Some(true),
            "{input}"
        );
    }

    #[test]
    fn predicate_decides_its_own_query() {
        let input = "domain = is_normal";

        assert_eq!(
            query(input, F::F064, FnName::IsNormal),
            Some(true),
            "{input}"
        );
        assert_eq!(
            query(input, F::F064, FnName::IsSubnormal),
            Some(false),
            "{input}"
        );
    }

    #[test]
    fn divisibility_leaves_normality_open() {
        let input = "domain = mul(0.25)";

        assert_eq!(
            query(input, F::F064, FnName::IsFinite),
            Some(true),
            "{input}"
        );
        assert_eq!(query(input, F::F064, FnName::IsNormal), None, "{input}");
    }

    #[test]
    fn function_lens_leaves_sign_open() {
        let input = "domain = abs in ..=1.0";

        assert_eq!(
            query(input, F::F064, FnName::IsFinite),
            Some(true),
            "{input}"
        );
        assert_eq!(
            query(input, F::F064, FnName::IsSignNegative),
            None,
            "{input}"
        );
    }

    // -------------------------------------------------------------------------
    // Query soundness

    /// Strictly increasing range endpoints around every class boundary.
    fn query_endpoints(subject: F) -> [&'static str; 9] {
        return match subject {
            F::F032 => [
                "-f32::MAX",
                "-1.0",
                "-f32::MIN_POSITIVE",
                "-1.0e-40",
                "0.0",
                "1.0e-40",
                "f32::MIN_POSITIVE",
                "1.0",
                "f32::MAX",
            ],
            F::F064 => [
                "-f64::MAX",
                "-1.0",
                "-f64::MIN_POSITIVE",
                "-1.0e-310",
                "0.0",
                "1.0e-310",
                "f64::MIN_POSITIVE",
                "1.0",
                "f64::MAX",
            ],
        };
    }

    /// Ranges over the endpoints, their combinations, and other atoms.
    /// `contains` evaluates in `f64`, which matches `f32` for all of them
    /// except `is_normal` and `is_subnormal`, so only `f64` uses those.
    fn query_sources(subject: F) -> Vec<String> {
        let endpoints = query_endpoints(subject);
        let mut ranges = Vec::new();
        for (index, lo) in endpoints.iter().enumerate() {
            ranges.extend([
                format!("{lo}.."),
                format!("..{lo}"),
                format!("..={lo}"),
                format!("{lo}.. & !{{{lo}}}"),
            ]);
            for hi in &endpoints[index + 1..] {
                ranges.extend([
                    format!("{lo}..={hi}"),
                    format!("{lo}..{hi}"),
                    format!("{lo}.. & !{{{lo}}} & ..{hi}"),
                ]);
            }
        }
        let mut sources = Vec::new();
        for range in ranges {
            sources.extend([
                format!("!({range})"),
                format!("({range}) | is_nan"),
                format!("({range}) | is_infinite"),
                format!("({range}) & !{{1.0}}"),
                range,
            ]);
        }
        sources.extend(
            [
                "is_finite",
                "!is_finite",
                "is_infinite",
                "is_sign_negative",
                "!is_sign_negative & !is_nan",
                "is_nan | is_infinite",
                "abs in ..=1.0",
                "abs in (1.0.. & !{1.0})",
                "mul(0.25)",
                "!mul(0.25)",
            ]
            .map(String::from),
        );
        if subject == F::F064 {
            sources.extend(
                [
                    "is_normal",
                    "is_subnormal",
                    "!is_subnormal",
                    "is_normal | is_nan",
                    "is_subnormal & is_sign_negative",
                ]
                .map(String::from),
            );
        }
        return sources;
    }

    /// Class boundaries and their neighbors in `subject`, of both signs.
    fn query_samples(subject: F) -> Vec<f64> {
        let mut samples = vec![
            f64::NAN,
            -f64::NAN,
        ];
        let mut push = |value: f64| {
            samples.extend([
                value,
                -value,
            ])
        };
        match subject {
            F::F032 => {
                for it in [
                    0.0,
                    1.0e-40,
                    f32::MIN_POSITIVE,
                    0.25,
                    1.0,
                    f32::MAX,
                    f32::INFINITY,
                ] {
                    for value in [
                        it,
                        it.next_up(),
                        it.next_down(),
                    ] {
                        push(f64::from(value));
                    }
                }
            }
            F::F064 => {
                for it in [
                    0.0,
                    1.0e-310,
                    f64::MIN_POSITIVE,
                    0.25,
                    1.0,
                    f64::MAX,
                    f64::INFINITY,
                ] {
                    for value in [
                        it,
                        it.next_up(),
                        it.next_down(),
                    ] {
                        push(value);
                    }
                }
            }
        }
        return samples;
    }

    /// Every sampled value a domain admits gives a decided query's answer.
    fn assert_query_sound(
        subject: F,
        function: FnName,
        answer: impl Fn(f64) -> bool,
    ) {
        for source in query_sources(subject) {
            let input = format!("domain = {source}");
            let config = resolved(&input, subject);
            let domain = config.validation.float_domain().unwrap();
            let Some(decided) = domain.query(function)
            else {
                continue;
            };
            for value in query_samples(subject) {
                if domain.contains(value) {
                    assert_eq!(answer(value), decided, "{input}: {value:e}");
                }
            }
        }
    }

    #[test]
    fn f64_is_nan_answer_is_sound() {
        assert_query_sound(F::F064, FnName::IsNan, f64::is_nan);
    }

    #[test]
    fn f64_is_infinite_answer_is_sound() {
        assert_query_sound(F::F064, FnName::IsInfinite, f64::is_infinite);
    }

    #[test]
    fn f64_is_finite_answer_is_sound() {
        assert_query_sound(F::F064, FnName::IsFinite, f64::is_finite);
    }

    #[test]
    fn f64_is_subnormal_answer_is_sound() {
        assert_query_sound(F::F064, FnName::IsSubnormal, f64::is_subnormal);
    }

    #[test]
    fn f64_is_normal_answer_is_sound() {
        assert_query_sound(F::F064, FnName::IsNormal, f64::is_normal);
    }

    #[test]
    fn f64_is_sign_positive_answer_is_sound() {
        assert_query_sound(
            F::F064,
            FnName::IsSignPositive,
            f64::is_sign_positive,
        );
    }

    #[test]
    fn f64_is_sign_negative_answer_is_sound() {
        assert_query_sound(
            F::F064,
            FnName::IsSignNegative,
            f64::is_sign_negative,
        );
    }

    // Samples are exact `f32` values; widening keeps NaN, infinity, finiteness,
    // and sign, so only normality needs the `f32` method.

    #[test]
    fn f32_is_nan_answer_is_sound() {
        assert_query_sound(F::F032, FnName::IsNan, f64::is_nan);
    }

    #[test]
    fn f32_is_infinite_answer_is_sound() {
        assert_query_sound(F::F032, FnName::IsInfinite, f64::is_infinite);
    }

    #[test]
    fn f32_is_finite_answer_is_sound() {
        assert_query_sound(F::F032, FnName::IsFinite, f64::is_finite);
    }

    #[test]
    fn f32_is_subnormal_answer_is_sound() {
        assert_query_sound(F::F032, FnName::IsSubnormal, |it| {
            (it as f32).is_subnormal()
        });
    }

    #[test]
    fn f32_is_normal_answer_is_sound() {
        assert_query_sound(F::F032, FnName::IsNormal, |it| {
            (it as f32).is_normal()
        });
    }

    #[test]
    fn f32_is_sign_positive_answer_is_sound() {
        assert_query_sound(
            F::F032,
            FnName::IsSignPositive,
            f64::is_sign_positive,
        );
    }

    #[test]
    fn f32_is_sign_negative_answer_is_sound() {
        assert_query_sound(
            F::F032,
            FnName::IsSignNegative,
            f64::is_sign_negative,
        );
    }

    // -------------------------------------------------------------------------
    // Totality flags on NaN-able types

    #[test]
    fn eq_on_nan_able_type_is_rejected() {
        let input = "domain = !{0.5}, with = eq";
        let error = config_error(input);

        assert!(
            error.contains("eq requires a NaN-free float type"),
            "{error}"
        );
        assert!(error.contains("write (domain) & !is_nan"), "{error}");
    }

    #[test]
    fn ord_on_nan_able_type_is_rejected() {
        let input = "with = ord";
        let error = config_error(input);

        assert!(
            error.contains("ord requires a NaN-free float type"),
            "{error}"
        );
        assert!(error.contains("such as !is_nan"), "{error}");
    }

    #[test]
    fn hash_on_nan_able_type_is_rejected() {
        let input = "with = hash";
        let error = config_error(input);

        assert!(error.contains("hash requires a NaN-free"), "{error}");
    }

    #[test]
    fn fn_op_cmp_on_nan_able_type_is_rejected() {
        let input = "with = fn_op_cmp";
        let error = config_error(input);

        assert!(error.contains("fn_op_cmp requires a NaN-free"), "{error}");
    }

    // -------------------------------------------------------------------------
    // Integer-only configuration

    #[test]
    fn non_zero_is_rejected_with_a_domain_hint() {
        let input = "with = non_zero";
        let error = config_error(input);

        assert!(
            error.contains("non_zero requires an integer type"),
            "{error}"
        );
        assert!(error.contains("domain = !{0.0}"), "{error}");
    }

    #[test]
    fn bit_access_is_rejected() {
        let input = "with = bit_access";
        let error = config_error(input);

        assert!(error.contains("bit_access requires an integer"), "{error}");
    }

    #[test]
    fn core_int_is_rejected() {
        let input = "with = core_int";
        let error = config_error(input);

        assert!(error.contains("core_int requires an integer"), "{error}");
    }

    #[test]
    fn proven_is_rejected() {
        let input = "with = proven";
        let error = config_error(input);

        assert!(error.contains("proven requires an integer"), "{error}");
    }

    #[test]
    fn fmt_binary_is_rejected() {
        let input = "with = fmt_binary";
        let error = config_error(input);

        assert!(error.contains("fmt_binary requires an integer"), "{error}");
    }

    #[test]
    fn fmt_hex_upper_is_rejected() {
        let input = "with = fmt_hex_upper";
        let error = config_error(input);

        assert!(
            error.contains("fmt_hex_upper requires an integer"),
            "{error}"
        );
    }

    #[test]
    fn range_is_rejected() {
        let input = "with = range";
        let error = config_error(input);

        assert!(error.contains("range requires an integer"), "{error}");
    }

    #[test]
    fn math_and_is_rejected() {
        let input = "with = math_and";
        let error = config_error(input);

        assert!(error.contains("math_and requires an integer"), "{error}");
    }

    #[test]
    fn math_shr_is_rejected() {
        let input = "with = math_shr";
        let error = config_error(input);

        assert!(error.contains("math_shr requires an integer"), "{error}");
    }

    #[test]
    fn assign_shl_is_rejected() {
        let input = "with = assign_shl";
        let error = config_error(input);

        assert!(error.contains("assign_shl requires an integer"), "{error}");
    }

    #[test]
    fn assign_xor_is_rejected() {
        let input = "with = assign_xor";
        let error = config_error(input);

        assert!(error.contains("assign_xor requires an integer"), "{error}");
    }

    #[test]
    fn fn_math_not_is_rejected() {
        let input = "with = fn_math_not";
        let error = config_error(input);

        assert!(error.contains("fn_math_not requires an integer"), "{error}");
    }

    #[test]
    fn self_bit_is_rejected() {
        let input = "with = self_bit";
        let error = config_error(input);

        assert!(error.contains("self_bit requires an integer"), "{error}");
    }

    #[test]
    fn bit_capability_is_rejected() {
        let input = "friend = convert(Other) -> Bit";
        let error = config_error(input);

        assert!(
            error.contains("the Bit capability requires an integer type"),
            "{error}"
        );
    }

    #[test]
    fn lossy_rel_friend_is_rejected() {
        let input = "friend = _(u64) -> Rel";
        let error = expansion_error(input, F::F064);

        assert_eq!(
            error,
            "u64 -> f64 is lossy; float Rel and Math friends must \
            convert exactly",
        );
    }

    #[test]
    fn facet_bit_capability_is_rejected() {
        let input = "facet = int { friend = convert(Other) -> Bit }";
        let error = config_error(input);

        assert!(
            error.contains("the Bit capability requires an integer type"),
            "{error}"
        );
    }

    #[test]
    fn circular_facet_is_rejected() {
        let input = "facet = circular { friend = convert(Other) -> Math }";
        let error = config_error(input);

        assert!(
            error.contains("float types support only the int and point"),
            "{error}"
        );
    }

    // -------------------------------------------------------------------------
    // Profiles

    #[test]
    fn int_entry_point_rejects_a_float_field() {
        let item = quote! { struct Meters(f64); };
        let kind = crate::profile_kind(ProfileKind::Int, &item);
        let error = kind.unwrap_err();

        assert_eq!(
            error.to_string(),
            "use #[typekin::float] for f32/f64 value fields",
        );
    }

    #[test]
    fn circular_entry_point_rejects_a_float_field() {
        let item = quote! { struct Angle(f64); };
        let kind = crate::profile_kind(ProfileKind::Circular, &item);
        let error = kind.unwrap_err();

        assert_eq!(
            error.to_string(),
            "the circular profile requires an integer value field",
        );
    }

    #[test]
    fn circular_profile_is_rejected() {
        let input = "profile = circular";
        let error = config_error(input);

        assert!(
            error.contains("float types support only the int and point"),
            "{error}"
        );
    }

    #[test]
    fn delta_profile_is_rejected() {
        let input = "profile = delta";
        let error = config_error(input);

        assert!(error.contains("only the int and point"), "{error}");
    }

    #[test]
    fn sequential_profile_is_rejected() {
        let input = "profile = sequential";
        let error = config_error(input);

        assert!(error.contains("only the int and point"), "{error}");
    }

    #[test]
    fn wrapping_profile_is_rejected() {
        let input = "profile = wrapping";
        let error = config_error(input);

        assert!(error.contains("only the int and point"), "{error}");
    }

    #[test]
    fn precedes_is_rejected() {
        let input = "precedes = check";
        let this = syn::parse_str::<FloatCfg>(input);

        assert!(this.is_err(), "{input}");
    }

    // -------------------------------------------------------------------------
    // Math provider

    #[test]
    fn nightly_math_enables_float_math() {
        let input = "math = nightly";
        let config = config(input);

        assert_eq!(config.math, MathProvider::Nightly, "{input}");
        assert!(config.flags.has(IntegralFlag::FloatMath), "{input}");
    }

    #[test]
    fn unknown_math_provider_is_rejected() {
        let input = "math = libm";
        let error = config_error(input);

        assert!(
            error.contains("expected `core`, `std`, or `nightly`"),
            "{error}"
        );
    }

    #[test]
    fn duplicate_math_is_rejected() {
        let input = "math = std, math = core";
        let this = syn::parse_str::<FloatCfg>(input);

        assert!(this.is_err(), "{input}");
    }

    // -------------------------------------------------------------------------
    // Integer-side rejections

    fn integral_error(input: &str) -> String {
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
        return this.err().unwrap().to_string();
    }

    #[test]
    fn integral_math_key_is_rejected() {
        let input = "math = std";
        let error = integral_error(input);

        assert!(
            error.contains("math selects the float math provider"),
            "{error}"
        );
    }

    #[test]
    fn integral_core_float_is_rejected() {
        let input = "with = core_float";
        let error = integral_error(input);

        assert!(
            error.contains("core_float requires a float type"),
            "{error}"
        );
    }

    #[test]
    fn integral_float_math_is_rejected() {
        let input = "with = float_math";
        let error = integral_error(input);

        assert!(
            error.contains("float_math requires a float type"),
            "{error}"
        );
    }

    #[test]
    fn rejects_float_math_when_operations_report_failure() {
        let input = "math = std, with = float_math, panic_at = nothing";
        let error = config_error(input);

        assert_eq!(
            error,
            "float_math returns `Self` from results it would have to panic on; panic_at = nothing leaves it out",
        );
    }

    #[test]
    fn unsigned_integral_math_neg_is_rejected() {
        let input = "with = math_neg";
        let error = integral_expansion(input, N::U032).unwrap_err().to_string();

        assert!(
            error.contains("math_neg requires a signed integer or float type"),
            "{error}"
        );
    }

    #[test]
    fn integral_sum_is_rejected() {
        let input = "with = sum";
        let error = integral_error(input);

        assert!(error.contains("sum requires a float type"), "{error}");
    }

    // -------------------------------------------------------------------------
    // Emission

    #[test]
    fn decided_query_returns_its_answer() {
        let input = "domain = 0.0..=1.0";
        let expansion = expansion_of(input, F::F064);

        assert!(
            expansion.contains(
                "\" Always `false`: every value in the domain `0.0..=1.0` \
                answers alike (decided by typekin at expansion).\""
            ),
            "{expansion}"
        );
        assert!(
            expansion.contains(
                "pub const fn is_nan (self) -> bool { return false ; }"
            ),
            "{expansion}"
        );
    }

    #[test]
    fn undecided_query_reads_the_value() {
        let input = "domain = 0.0..=1.0";
        let expansion = expansion_of(input, F::F064);

        assert!(
            expansion.contains(
                "pub const fn is_sign_negative (self) -> bool { return Self \
                :: raw (self) . is_sign_negative () ; }"
            ),
            "{expansion}"
        );
    }

    #[test]
    fn integer_value_field_is_rejected() {
        let input = "";
        let source = "struct Count(u32);";
        let error = item_error(input, source);

        assert_eq!(error, "float value field must be f32 or f64");
    }

    #[test]
    fn half_precision_value_field_is_rejected() {
        let input = "";
        let source = "struct Half(f16);";
        let error = item_error(input, source);

        assert_eq!(error, "f16 and f128 are not supported");
    }

    #[test]
    fn ord_requires_a_private_value_field() {
        let input = "domain = !is_nan, with = ord";
        let source = "pub struct Ordered(pub f64);";
        let error = item_error(input, source);

        assert_eq!(error, "ord on a float type requires a private value field",);
    }

    #[test]
    fn ord_compares_without_unwrapping() {
        let input = "domain = !is_nan";
        let expansion = expansion_of(input, F::F064);

        assert!(
            expansion.contains("impl :: core :: cmp :: Ord"),
            "{expansion}"
        );
        assert!(!expansion.contains("unwrap"), "{expansion}");
    }

    #[test]
    fn exact_integer_conversion_uses_the_round_trip_only() {
        let input = "";
        let expansion = expansion_of(input, F::F064);
        let start = expansion.find("fn try_into_i32").unwrap();
        let body = &expansion[start..start + 400];

        assert!(body.contains("t as f64 == r"), "{body}");
        assert!(!body.contains("r <"), "{body}");
    }

    #[test]
    fn f32_usize_conversion_bounds_narrow_pointer_widths() {
        let input = "";
        let expansion = expansion_of(input, F::F032);

        assert_eq!(expansion.matches("fn try_into_usize").count(), 3);
        assert!(expansion.contains("r < 4294967296.0"), "{expansion}");
    }

    #[test]
    fn pointer_sized_conversion_varies_by_width() {
        let input = "";
        let expansion = expansion_of(input, F::F064);

        assert_eq!(expansion.matches("fn try_into_usize").count(), 2);
        assert!(
            expansion.contains(
                "# [cfg (any (target_pointer_width = \"16\" , target_pointer_width = \"32\"))]"
            ),
            "{expansion}"
        );
    }
}
