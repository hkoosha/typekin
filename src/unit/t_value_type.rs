#[cfg(test)]
mod tests {
    use crate::zz_values::{
        F,
        N,
        Num,
        Width,
        pow2_literal,
    };

    #[test]
    fn usize_converts_to_f32_exactly_only_at_16_bits() {
        let source = Num::Int(N::USIZ);
        let target = Num::Float(F::F032);
        let narrow = source.can_safe_cast_to(target, Width::W16);
        let wide = source.can_safe_cast_to(target, Width::W32);

        assert!(narrow, "{source:?} -> {target:?} on 16 bits");
        assert!(!wide, "{source:?} -> {target:?} on 32 bits");
    }

    #[test]
    fn usize_converts_to_f64_exactly_at_32_bits() {
        let source = Num::Int(N::USIZ);
        let target = Num::Float(F::F064);
        let narrow = source.can_safe_cast_to(target, Width::W32);

        assert!(narrow, "{source:?} -> {target:?} on 32 bits");
    }

    #[test]
    fn num_resolves_integer_and_float_names() {
        let int = "u8";
        let float = "f64";
        let other = "f16";

        assert_eq!(Num::of(int), Some(Num::Int(N::U008)), "{int}");
        assert_eq!(Num::of(float), Some(Num::Float(F::F064)), "{float}");
        assert_eq!(Num::of(other), None, "{other}");
    }

    #[test]
    fn pow2_literal_is_exact_beyond_u64() {
        let exponent = 64;
        let literal = pow2_literal(exponent);

        assert_eq!(literal, "18446744073709551616.0", "2^{exponent}");
    }

    #[test]
    fn pow2_literal_covers_the_u128_bound() {
        let exponent = 128;
        let literal = pow2_literal(exponent);

        assert_eq!(
            literal, "340282366920938463463374607431768211456.0",
            "2^{exponent}"
        );
    }

    #[test]
    fn pow2_literal_parses_to_the_exact_float() {
        let exponent = 63;
        let literal = pow2_literal(exponent);
        let parsed = literal.parse::<f64>();

        assert!(parsed.is_ok(), "{literal}");
        assert_eq!(parsed.unwrap(), 2f64.powi(63), "{literal}");
    }
}
