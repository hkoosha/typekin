#[cfg(test)]
mod tests {
    use crate::zz_domain::Domain;
    use crate::zz_domain_closure::{
        Closure,
        ClosureOp,
        Policy,
        Rhs,
        eval,
    };
    use crate::zz_domain_int::ResolvedDomain;
    use crate::zz_values::{
        Int,
        N,
        Width,
    };

    fn resolved(
        source: &str,
        subject: N,
    ) -> ResolvedDomain {
        let this = syn::parse_str::<Domain>(source);
        assert!(
            this.is_ok(),
            "{source}: {:?}",
            this.err().map(|it| it.to_string())
        );
        let resolved = this.unwrap().resolve(subject, false);
        assert!(
            resolved.is_ok(),
            "{source}: {:?}",
            resolved.err().map(|it| it.to_string())
        );
        return resolved.unwrap();
    }

    fn closure(
        source: &str,
        subject: N,
        op: ClosureOp,
        rhs: Rhs,
        policy: Policy,
    ) -> Closure {
        return resolved(source, subject).closure(op, rhs, policy);
    }

    fn int(value: i128) -> Int {
        return Int::from_sign_magnitude(value < 0, value.unsigned_abs());
    }

    // -------------------------------------------------------------------------
    // Eligibility

    #[test]
    fn interval_domain_is_eligible() {
        let source = "..=100";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn union_domain_is_eligible() {
        let source = "..=1 | 254..";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn ne_atom_is_ineligible() {
        let source = "..=100 & !{10}";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Ineligible(String::from("!{10}")),
            "{source}"
        );
    }

    #[test]
    fn divisible_atom_is_ineligible() {
        let source = "mul(3)";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Ineligible(String::from("mul(3)")),
            "{source}"
        );
    }

    #[test]
    fn negated_divisible_atom_is_ineligible() {
        let source = "..=100 & !mul(3)";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Ineligible(String::from("!mul(3)")),
            "{source}"
        );
    }

    #[test]
    fn predicate_is_ineligible() {
        let source = "is_power_of_two";
        let closure = closure(
            source,
            N::U032,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Ineligible(String::from("is_power_of_two")),
            "{source}"
        );
    }

    #[test]
    fn lens_is_ineligible() {
        let source = "abs in ..=1000";
        let closure = closure(
            source,
            N::I032,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Ineligible(String::from("unsigned_abs in ..=1000")),
            "{source}"
        );
    }

    #[test]
    fn lens_with_argument_renders_its_call() {
        let source = "rem_euclid(16) in {3}";
        let closure = closure(
            source,
            N::U032,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Ineligible(String::from("rem_euclid(16) in {3}")),
            "{source}"
        );
    }

    #[test]
    fn negated_group_is_ineligible() {
        let source = "!(3..6)";
        let closure = closure(
            source,
            N::U032,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Ineligible(String::from("!(3..6)")),
            "{source}"
        );
    }

    #[test]
    fn non_zero_storage_requirement_is_ineligible() {
        let source = "..=100";
        let domain = syn::parse_str::<Domain>(source);
        assert!(domain.is_ok(), "{source}");
        let resolved = domain.unwrap().resolve(N::U008, true);
        assert!(resolved.is_ok(), "{source}");
        let closure = resolved.unwrap().closure(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Ineligible(String::from("!{0}")),
            "{source}"
        );
    }

    // -------------------------------------------------------------------------
    // Proofs and refutations

    #[test]
    fn div_on_zero_based_interval_is_closed() {
        let source = "..=100";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn div_by_primitive_on_zero_based_interval_is_closed() {
        let source = "..=100";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Div,
            Rhs::Full,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn div_on_one_based_interval_is_not_closed() {
        let source = "1..=100";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(1),
                rhs: Some(int(2)),
                result: int(0),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn signed_div_on_symmetric_interval_is_closed() {
        let source = "-100..=100";
        let closure = closure(
            source,
            N::I008,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn rem_on_zero_based_interval_is_closed() {
        let source = "..=100";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn checked_add_on_positive_is_closed() {
        let source = "1..";
        let closure = closure(
            source,
            N::U032,
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn wrapping_add_on_positive_is_not_closed() {
        let source = "1..";
        let closure = closure(
            source,
            N::U032,
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Wrapping,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(1),
                rhs: Some(int(u32::MAX as i128)),
                result: int(0),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn saturating_add_on_lower_bounded_is_closed() {
        let source = "10..";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Saturating,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn checked_mul_on_positive_is_closed() {
        let source = "1..";
        let closure = closure(
            source,
            N::U016,
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn bitor_on_power_of_two_mask_is_closed() {
        let source = "..=15";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn bitor_beyond_power_of_two_mask_is_not_closed() {
        let source = "..=16";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(1),
                rhs: Some(int(16)),
                result: int(17),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn bitand_by_primitive_on_zero_based_interval_is_closed() {
        let source = "..=100";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn bitand_on_union_without_corner_counterexample_is_unknown() {
        let source = "..=10 | 20..=30";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Unknown {
                lo: int(0),
                hi: int(30),
                width: Width::W16,
            },
            "{source}"
        );
    }

    #[test]
    fn shr_on_zero_based_interval_is_closed() {
        let source = "..=100";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Shr,
            Rhs::Shift,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn unbounded_shr_on_positive_is_not_closed() {
        let source = "1..";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::UnboundedShr,
            Rhs::Shift,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(1),
                rhs: Some(int(1)),
                result: int(0),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn neg_on_symmetric_interval_is_closed() {
        let source = "-100..=100";
        let closure = closure(
            source,
            N::I008,
            ClosureOp::Neg,
            Rhs::Unary,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn wrapping_neg_keeps_min_valid() {
        let source = "..=-100 | 100..";
        let closure = closure(
            source,
            N::I008,
            ClosureOp::Neg,
            Rhs::Unary,
            Policy::Wrapping,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn wrapping_neg_on_unsigned_is_not_closed() {
        let source = "..=100";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Neg,
            Rhs::Unary,
            Policy::Wrapping,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(1),
                rhs: None,
                result: int(255),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn not_on_symmetric_signed_interval_is_closed() {
        let source = "-100..=99";
        let closure = closure(
            source,
            N::I008,
            ClosureOp::Not,
            Rhs::Unary,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn abs_on_symmetric_interval_is_closed() {
        let source = "-100..=100";
        let closure = closure(
            source,
            N::I008,
            ClosureOp::Abs,
            Rhs::Unary,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn isqrt_on_zero_based_interval_is_closed() {
        let source = "..=1000";
        let closure = closure(
            source,
            N::U016,
            ClosureOp::Isqrt,
            Rhs::Unary,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn midpoint_on_interval_is_closed() {
        let source = "10..=20";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn midpoint_on_union_with_gap_is_not_closed() {
        let source = "..=10 | 20..=30";
        let closure = closure(
            source,
            N::U008,
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(0),
                rhs: Some(int(29)),
                result: int(14),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn domain_beyond_interval_cap_is_unknown() {
        let source = "-1..=0 | 2..=2 | -3..=-3 | 4..=4 | -5..=-5 | 6..=6 | -7..=-7 | 8..=8 | -9..=-9 | 10..=10 | -11..=-11 | 12..=12 | -13..=-13 | 14..=14 | -15..=-15 | 16..=16 | -17..=-17";
        let closure = closure(
            source,
            N::I008,
            ClosureOp::Not,
            Rhs::Unary,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Unknown {
                lo: int(-128),
                hi: int(127),
                width: Width::W16,
            },
            "{source}"
        );
    }

    #[test]
    fn refutation_recomputes_outside_domain() {
        let source = "..=100";
        let domain = resolved(source, N::U008);
        let closure =
            domain.closure(ClosureOp::Add, Rhs::SameDomain, Policy::Checked);
        let Closure::Refuted {
            lhs,
            rhs: Some(rhs),
            result,
            width,
            ..
        } = closure
        else {
            panic!("{source}: {closure:?}");
        };
        let recomputed =
            (lhs.magnitude() as u8).checked_add(rhs.magnitude() as u8);
        let contains = domain.contains(result, width);

        assert_eq!(
            recomputed.map(|it| int(it as i128)),
            Some(result),
            "{source}"
        );
        assert!(!contains, "{source}");
    }

    // -------------------------------------------------------------------------
    // Pointer widths and 128-bit extremes

    #[test]
    fn usize_domain_closed_only_at_64_bits_is_not_closed() {
        let source = "..=10 | 18446744073709551605..";
        let closure = closure(
            source,
            N::USIZ,
            ClosureOp::Not,
            Rhs::Unary,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(0),
                rhs: None,
                result: int(65535),
                width: Width::W16,
                all_widths: false,
            },
            "{source}"
        );
    }

    #[test]
    fn u64_domain_closed_under_not_is_closed() {
        let source = "..=10 | 18446744073709551605..";
        let closure = closure(
            source,
            N::U064,
            ClosureOp::Not,
            Rhs::Unary,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn u128_checked_mul_on_positive_is_closed() {
        let source = "1..";
        let closure = closure(
            source,
            N::U128,
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn u128_wrapping_add_on_positive_is_not_closed() {
        let source = "1..";
        let closure = closure(
            source,
            N::U128,
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Wrapping,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(1),
                rhs: Some(Int::NonNegative(u128::MAX)),
                result: int(0),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn u128_saturating_mul_on_upper_half_is_closed() {
        let source = "170141183460469231731687303715884105728..";
        let closure = closure(
            source,
            N::U128,
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Saturating,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn u128_unbounded_shr_by_width_is_not_closed() {
        let source = "170141183460469231731687303715884105728..";
        let closure = closure(
            source,
            N::U128,
            ClosureOp::UnboundedShr,
            Rhs::Shift,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: Int::NonNegative(1 << 127),
                rhs: Some(int(1)),
                result: Int::NonNegative(1 << 126),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn i128_saturating_add_below_int_range_is_closed() {
        let source = "..=0";
        let closure = closure(
            source,
            N::I128,
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Saturating,
        );

        assert_eq!(closure, Closure::Proven, "{source}");
    }

    #[test]
    fn i128_checked_div_on_negative_is_not_closed() {
        let source = "..=-1";
        let closure = closure(
            source,
            N::I128,
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(i128::MIN),
                rhs: Some(int(i128::MIN)),
                result: int(1),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn i128_wrapping_neg_on_negative_is_not_closed() {
        let source = "..=-1";
        let closure = closure(
            source,
            N::I128,
            ClosureOp::Neg,
            Rhs::Unary,
            Policy::Wrapping,
        );

        assert_eq!(
            closure,
            Closure::Refuted {
                lhs: int(i128::MIN + 1),
                rhs: None,
                result: int(i128::MAX),
                width: Width::W16,
                all_widths: true,
            },
            "{source}"
        );
    }

    #[test]
    fn i128_checked_mul_without_counterexample_is_unknown() {
        let source = "-170141183460469231731687303715884105727..";
        let closure = closure(
            source,
            N::I128,
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Checked,
        );

        assert_eq!(
            closure,
            Closure::Unknown {
                lo: int(i128::MIN),
                hi: int(i128::MAX),
                width: Width::W16,
            },
            "{source}"
        );
    }

    // -------------------------------------------------------------------------
    // Soundness against Rust's own operations: exhaustive over u8 and i8,
    // boundary samples over u128 and i128

    /// A primitive whose values are enumerated: every value for 8-bit types,
    /// boundary samples for 128-bit ones.
    trait Prim: Copy + std::fmt::Debug {
        const SUBJECT: N;
        /// Endpoints combined into every `lo..=hi` domain.
        const ENDPOINTS: &[&str];
        const UNIONS: &[&str];

        fn all() -> Vec<Self>;
        fn int(self) -> Int;
        fn of(value: Int) -> Self;
    }

    impl Prim for u8 {
        const SUBJECT: N = N::U008;
        const ENDPOINTS: &[&str] = &[
            "0",
            "1",
            "2",
            "3",
            "7",
            "8",
            "15",
            "16",
            "31",
            "32",
            "63",
            "64",
            "100",
            "127",
            "128",
            "200",
            "254",
            "255",
        ];
        const UNIONS: &[&str] = &[
            "..=10 | 20..=30",
            "..=1 | 254..",
            "1..=3 | 100..=200",
            "0..=0 | 8..=8 | 255..=255",
            "..=15 | 240..",
        ];

        fn all() -> Vec<Self> {
            return (Self::MIN..=Self::MAX).collect();
        }

        fn int(self) -> Int {
            return Int::NonNegative(self as u128);
        }

        fn of(value: Int) -> Self {
            assert!(!value.is_negative(), "{value}");
            return value.magnitude() as Self;
        }
    }

    impl Prim for i8 {
        const SUBJECT: N = N::I008;
        const ENDPOINTS: &[&str] = &[
            "-128",
            "-127",
            "-100",
            "-65",
            "-64",
            "-8",
            "-2",
            "-1",
            "0",
            "1",
            "2",
            "7",
            "8",
            "63",
            "64",
            "100",
            "126",
            "127",
        ];
        const UNIONS: &[&str] = &[
            "..=-100 | 100..",
            "-10..=-5 | 5..=10",
            "-1..=1 | 50..=60",
            "..=-127 | 0..=0 | 127..",
            "-128..=-120 | -8..=7",
        ];

        fn all() -> Vec<Self> {
            return (Self::MIN..=Self::MAX).collect();
        }

        fn int(self) -> Int {
            return Int::from_sign_magnitude(
                self < 0,
                self.unsigned_abs() as u128,
            );
        }

        fn of(value: Int) -> Self {
            let magnitude = value.magnitude() as u8;
            return match value.is_negative() {
                true => magnitude.wrapping_neg() as Self,
                false => magnitude as Self,
            };
        }
    }

    impl Prim for u128 {
        const SUBJECT: N = N::U128;
        const ENDPOINTS: &[&str] = &[
            "0",
            "1",
            "2",
            "255",
            "18446744073709551615",
            "18446744073709551616",
            "170141183460469231731687303715884105727",
            "170141183460469231731687303715884105728",
            "340282366920938463463374607431768211454",
            "340282366920938463463374607431768211455",
        ];
        const UNIONS: &[&str] = &[
            "..=1 | 340282366920938463463374607431768211454..",
            "..=255 | 18446744073709551616..=170141183460469231731687303715884105727",
        ];

        fn all() -> Vec<Self> {
            let mut values = Vec::new();
            for base in [
                0,
                1 << 8,
                1 << 64,
                1 << 126,
                1 << 127,
                Self::MAX / 3,
            ] {
                for offset in 0..=3 {
                    values.push(base.wrapping_add(offset));
                    values.push(base.wrapping_sub(offset));
                }
            }
            values.sort();
            values.dedup();
            return values;
        }

        fn int(self) -> Int {
            return Int::NonNegative(self);
        }

        fn of(value: Int) -> Self {
            assert!(!value.is_negative(), "{value}");
            return value.magnitude();
        }
    }

    impl Prim for i128 {
        const SUBJECT: N = N::I128;
        const ENDPOINTS: &[&str] = &[
            "-170141183460469231731687303715884105728",
            "-170141183460469231731687303715884105727",
            "-18446744073709551616",
            "-2",
            "-1",
            "0",
            "1",
            "2",
            "18446744073709551616",
            "170141183460469231731687303715884105726",
            "170141183460469231731687303715884105727",
        ];
        const UNIONS: &[&str] = &[
            "..=-18446744073709551616 | 18446744073709551616..",
            "-1..=1 | 170141183460469231731687303715884105727..",
        ];

        fn all() -> Vec<Self> {
            let mut values = Vec::new();
            for base in [
                0,
                1 << 8,
                1 << 64,
                1 << 125,
                Self::MAX,
                Self::MAX / 3,
            ] {
                for offset in 0..=3 {
                    for value in [
                        base.wrapping_add(offset),
                        base.wrapping_sub(offset),
                    ] {
                        values.push(value);
                        values.push(value.wrapping_neg());
                    }
                }
            }
            values.sort();
            values.dedup();
            return values;
        }

        fn int(self) -> Int {
            return Int::from_sign_magnitude(self < 0, self.unsigned_abs());
        }

        fn of(value: Int) -> Self {
            let magnitude = value.magnitude() as Self;
            return match value.is_negative() {
                true => magnitude.wrapping_neg(),
                false => magnitude,
            };
        }
    }

    struct Case {
        source: String,
        domain: ResolvedDomain,
    }

    impl Case {
        fn valid(
            &self,
            value: Int,
        ) -> bool {
            return self.domain.contains(value, Width::W64);
        }
    }

    /// Every `lo..=hi` over the endpoints except the full range, and unions.
    fn cases<T: Prim>() -> Vec<Case> {
        let endpoints = T::ENDPOINTS;
        let mut sources = Vec::new();
        for (low, lo) in endpoints.iter().enumerate() {
            for (high, hi) in endpoints.iter().enumerate() {
                let full = low == 0 && high == endpoints.len() - 1;
                if low <= high && !full {
                    sources.push(format!("{lo}..={hi}"));
                }
            }
        }
        sources.extend(T::UNIONS.iter().map(|it| it.to_string()));

        return sources
            .into_iter()
            .map(|source| {
                let domain = resolved(&source, T::SUBJECT);
                return Case { source, domain };
            })
            .collect();
    }

    fn shift_amounts() -> Vec<u32> {
        return (0..=300).chain([u32::MAX]).collect();
    }

    /// Whenever `closure` proves the operation, every result of valid
    /// operands is valid; a refutation is a real invalid result.
    fn assert_sound<T: Prim>(
        op: ClosureOp,
        rhs: Rhs,
        policy: Policy,
        rhs_values: Vec<Option<Int>>,
        real: impl Fn(T, Option<Int>) -> Option<T>,
    ) {
        for case in cases::<T>() {
            let source = &case.source;
            let lhs_values = T::all()
                .into_iter()
                .filter(|it| case.valid(it.int()))
                .collect::<Vec<_>>();
            let rhs_values = rhs_values
                .iter()
                .filter(|it| rhs != Rhs::SameDomain || case.valid(it.unwrap()))
                .collect::<Vec<_>>();
            let closure = case.domain.closure(op, rhs, policy);
            match closure {
                Closure::Proven => {
                    for a in &lhs_values {
                        for b in &rhs_values {
                            let Some(result) = real(*a, **b)
                            else {
                                continue;
                            };
                            let valid = case.valid(result.int());

                            assert!(
                                valid,
                                "{source}: {a:?} {op:?} {b:?} = {result:?} is proven valid"
                            );
                        }
                    }
                }
                Closure::Refuted {
                    lhs,
                    rhs: amount,
                    result,
                    all_widths,
                    ..
                } => {
                    let produced = real(T::of(lhs), amount);
                    let rhs_valid = match (rhs, amount) {
                        (Rhs::SameDomain, Some(it)) => case.valid(it),
                        (Rhs::Full, Some(_)) => true,
                        (Rhs::Shift, Some(it)) => !it.is_negative(),
                        (Rhs::Unary, None) => true,
                        _ => false,
                    };

                    assert!(case.valid(lhs), "{source}: {lhs} is valid");
                    assert!(
                        rhs_valid,
                        "{source}: {amount:?} is a valid operand"
                    );
                    assert_eq!(
                        produced.map(T::int),
                        Some(result),
                        "{source}: {lhs} {op:?} {amount:?}"
                    );
                    assert!(
                        !case.valid(result),
                        "{source}: {result} is invalid"
                    );
                    assert!(
                        all_widths,
                        "{source}: a fixed-width counterexample applies at every width"
                    );
                }
                Closure::Unknown { .. } => {}
                Closure::Ineligible(atom) => {
                    panic!("{source}: ineligible at {atom}")
                }
            }
        }
    }

    fn assert_binary_sound<T: Prim>(
        op: ClosureOp,
        rhs: Rhs,
        policy: Policy,
        real: impl Fn(T, T) -> Option<T>,
    ) {
        let rhs_values =
            T::all().into_iter().map(|it| Some(it.int())).collect();
        assert_sound(op, rhs, policy, rhs_values, |a, b| {
            real(a, T::of(b.unwrap()))
        });
    }

    fn assert_shift_sound<T: Prim>(
        op: ClosureOp,
        policy: Policy,
        real: impl Fn(T, u32) -> Option<T>,
    ) {
        let rhs_values = shift_amounts()
            .into_iter()
            .map(|it| Some(Int::NonNegative(it as u128)))
            .collect();
        assert_sound(op, Rhs::Shift, policy, rhs_values, |a, b| {
            return real(a, b.unwrap().magnitude() as u32);
        });
    }

    fn assert_unary_sound<T: Prim>(
        op: ClosureOp,
        policy: Policy,
        real: impl Fn(T) -> Option<T>,
    ) {
        assert_sound(op, Rhs::Unary, policy, vec![None], |a, _| real(a));
    }

    /// `eval` agrees with Rust on every enumerated operand pair.
    fn assert_eval<T: Prim>(
        op: ClosureOp,
        policy: Policy,
        rhs_values: Vec<Option<Int>>,
        real: impl Fn(T, Option<Int>) -> Option<T>,
    ) {
        for a in T::all() {
            for b in &rhs_values {
                let expected = real(a, *b).map(T::int);
                let ty = T::SUBJECT.at(Width::W64);
                let actual = eval(op, policy, ty, a.int(), *b);

                assert_eq!(
                    actual, expected,
                    "{a:?} {op:?} {b:?} under {policy:?}"
                );
            }
        }
    }

    fn assert_binary_eval<T: Prim>(
        op: ClosureOp,
        policy: Policy,
        real: impl Fn(T, T) -> Option<T>,
    ) {
        let rhs_values =
            T::all().into_iter().map(|it| Some(it.int())).collect();
        assert_eval(op, policy, rhs_values, |a, b| real(a, T::of(b.unwrap())));
    }

    fn assert_shift_eval<T: Prim>(
        op: ClosureOp,
        policy: Policy,
        real: impl Fn(T, u32) -> Option<T>,
    ) {
        let rhs_values = shift_amounts()
            .into_iter()
            .map(|it| Some(Int::NonNegative(it as u128)))
            .collect();
        assert_eval(op, policy, rhs_values, |a, b| {
            real(a, b.unwrap().magnitude() as u32)
        });
    }

    fn assert_unary_eval<T: Prim>(
        op: ClosureOp,
        policy: Policy,
        real: impl Fn(T) -> Option<T>,
    ) {
        assert_eval(op, policy, vec![None], |a, _| real(a));
    }

    // -------------------------------------------------------------------------
    // u8: soundness

    #[test]
    fn u8_checked_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| a.checked_add(b),
        );
    }

    #[test]
    fn u8_checked_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| a.checked_add(b),
        );
    }

    #[test]
    fn u8_wrapping_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| Some(a.wrapping_add(b)),
        );
    }

    #[test]
    fn u8_wrapping_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| Some(a.wrapping_add(b)),
        );
    }

    #[test]
    fn u8_saturating_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: u8, b| Some(a.saturating_add(b)),
        );
    }

    #[test]
    fn u8_saturating_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Saturating,
            |a: u8, b| Some(a.saturating_add(b)),
        );
    }

    #[test]
    fn u8_checked_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| a.checked_sub(b),
        );
    }

    #[test]
    fn u8_checked_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| a.checked_sub(b),
        );
    }

    #[test]
    fn u8_wrapping_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| Some(a.wrapping_sub(b)),
        );
    }

    #[test]
    fn u8_wrapping_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| Some(a.wrapping_sub(b)),
        );
    }

    #[test]
    fn u8_saturating_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: u8, b| Some(a.saturating_sub(b)),
        );
    }

    #[test]
    fn u8_saturating_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Saturating,
            |a: u8, b| Some(a.saturating_sub(b)),
        );
    }

    #[test]
    fn u8_checked_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| a.checked_mul(b),
        );
    }

    #[test]
    fn u8_checked_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| a.checked_mul(b),
        );
    }

    #[test]
    fn u8_wrapping_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| Some(a.wrapping_mul(b)),
        );
    }

    #[test]
    fn u8_wrapping_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| Some(a.wrapping_mul(b)),
        );
    }

    #[test]
    fn u8_saturating_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: u8, b| Some(a.saturating_mul(b)),
        );
    }

    #[test]
    fn u8_saturating_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Saturating,
            |a: u8, b| Some(a.saturating_mul(b)),
        );
    }

    #[test]
    fn u8_checked_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| a.checked_div(b),
        );
    }

    #[test]
    fn u8_checked_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| a.checked_div(b),
        );
    }

    #[test]
    fn u8_wrapping_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_div(b)),
        );
    }

    #[test]
    fn u8_wrapping_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_div(b)),
        );
    }

    #[test]
    fn u8_saturating_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: u8, b| (b != 0).then(|| a.saturating_div(b)),
        );
    }

    #[test]
    fn u8_saturating_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Saturating,
            |a: u8, b| (b != 0).then(|| a.saturating_div(b)),
        );
    }

    #[test]
    fn u8_checked_rem_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| a.checked_rem(b),
        );
    }

    #[test]
    fn u8_checked_rem_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| a.checked_rem(b),
        );
    }

    #[test]
    fn u8_wrapping_rem_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_rem(b)),
        );
    }

    #[test]
    fn u8_wrapping_rem_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_rem(b)),
        );
    }

    #[test]
    fn u8_checked_div_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn u8_checked_div_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn u8_wrapping_div_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn u8_wrapping_div_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn u8_checked_rem_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn u8_checked_rem_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn u8_wrapping_rem_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn u8_wrapping_rem_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn u8_checked_bit_and_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| Some(a & b),
        );
    }

    #[test]
    fn u8_checked_bit_and_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| Some(a & b),
        );
    }

    #[test]
    fn u8_wrapping_bit_and_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| Some(a & b),
        );
    }

    #[test]
    fn u8_wrapping_bit_and_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| Some(a & b),
        );
    }

    #[test]
    fn u8_checked_bit_or_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| Some(a | b),
        );
    }

    #[test]
    fn u8_checked_bit_or_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| Some(a | b),
        );
    }

    #[test]
    fn u8_wrapping_bit_or_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| Some(a | b),
        );
    }

    #[test]
    fn u8_wrapping_bit_or_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| Some(a | b),
        );
    }

    #[test]
    fn u8_checked_bit_xor_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| Some(a ^ b),
        );
    }

    #[test]
    fn u8_checked_bit_xor_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| Some(a ^ b),
        );
    }

    #[test]
    fn u8_wrapping_bit_xor_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| Some(a ^ b),
        );
    }

    #[test]
    fn u8_wrapping_bit_xor_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| Some(a ^ b),
        );
    }

    #[test]
    fn u8_checked_midpoint_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u8_checked_midpoint_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::Full,
            Policy::Checked,
            |a: u8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u8_wrapping_midpoint_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u8_wrapping_midpoint_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::Full,
            Policy::Wrapping,
            |a: u8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u8_checked_shr_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Shr,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u8, b| a.checked_shr(u32::from(b)),
        );
    }

    #[test]
    fn u8_wrapping_shr_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Shr,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u8, b| Some(a.wrapping_shr(b as u32)),
        );
    }

    #[test]
    fn u8_checked_shr_amount_is_sound() {
        assert_shift_sound(ClosureOp::Shr, Policy::Checked, |a: u8, s| {
            a.checked_shr(s)
        });
    }

    #[test]
    fn u8_wrapping_shr_amount_is_sound() {
        assert_shift_sound(ClosureOp::Shr, Policy::Wrapping, |a: u8, s| {
            Some(a.wrapping_shr(s))
        });
    }

    #[test]
    fn u8_checked_unbounded_shr_amount_is_sound() {
        assert_shift_sound(
            ClosureOp::UnboundedShr,
            Policy::Checked,
            |a: u8, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn u8_wrapping_unbounded_shr_amount_is_sound() {
        assert_shift_sound(
            ClosureOp::UnboundedShr,
            Policy::Wrapping,
            |a: u8, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn u8_checked_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Checked, |a: u8| {
            a.checked_neg()
        });
    }

    #[test]
    fn u8_wrapping_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Wrapping, |a: u8| {
            Some(a.wrapping_neg())
        });
    }

    #[test]
    fn u8_checked_not_is_sound() {
        assert_unary_sound(ClosureOp::Not, Policy::Checked, |a: u8| Some(!a));
    }

    #[test]
    fn u8_wrapping_not_is_sound() {
        assert_unary_sound(ClosureOp::Not, Policy::Wrapping, |a: u8| Some(!a));
    }

    #[test]
    fn u8_checked_isqrt_is_sound() {
        assert_unary_sound(ClosureOp::Isqrt, Policy::Checked, |a: u8| {
            Some(a.isqrt())
        });
    }

    // -------------------------------------------------------------------------
    // u8: eval

    #[test]
    fn u8_eval_matches_checked_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Checked, |a: u8, b| {
            a.checked_add(b)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Wrapping, |a: u8, b| {
            Some(a.wrapping_add(b))
        });
    }

    #[test]
    fn u8_eval_matches_saturating_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Saturating, |a: u8, b| {
            Some(a.saturating_add(b))
        });
    }

    #[test]
    fn u8_eval_matches_checked_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Checked, |a: u8, b| {
            a.checked_sub(b)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Wrapping, |a: u8, b| {
            Some(a.wrapping_sub(b))
        });
    }

    #[test]
    fn u8_eval_matches_saturating_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Saturating, |a: u8, b| {
            Some(a.saturating_sub(b))
        });
    }

    #[test]
    fn u8_eval_matches_checked_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Checked, |a: u8, b| {
            a.checked_mul(b)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Wrapping, |a: u8, b| {
            Some(a.wrapping_mul(b))
        });
    }

    #[test]
    fn u8_eval_matches_saturating_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Saturating, |a: u8, b| {
            Some(a.saturating_mul(b))
        });
    }

    #[test]
    fn u8_eval_matches_checked_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Checked, |a: u8, b| {
            a.checked_div(b)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Wrapping, |a: u8, b| {
            (b != 0).then(|| a.wrapping_div(b))
        });
    }

    #[test]
    fn u8_eval_matches_saturating_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Saturating, |a: u8, b| {
            (b != 0).then(|| a.saturating_div(b))
        });
    }

    #[test]
    fn u8_eval_matches_checked_rem() {
        assert_binary_eval(ClosureOp::Rem, Policy::Checked, |a: u8, b| {
            a.checked_rem(b)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_rem() {
        assert_binary_eval(ClosureOp::Rem, Policy::Wrapping, |a: u8, b| {
            (b != 0).then(|| a.wrapping_rem(b))
        });
    }

    #[test]
    fn u8_eval_matches_checked_div_euclid() {
        assert_binary_eval(
            ClosureOp::DivEuclid,
            Policy::Checked,
            |a: u8, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn u8_eval_matches_wrapping_div_euclid() {
        assert_binary_eval(
            ClosureOp::DivEuclid,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn u8_eval_matches_checked_rem_euclid() {
        assert_binary_eval(
            ClosureOp::RemEuclid,
            Policy::Checked,
            |a: u8, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn u8_eval_matches_wrapping_rem_euclid() {
        assert_binary_eval(
            ClosureOp::RemEuclid,
            Policy::Wrapping,
            |a: u8, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn u8_eval_matches_checked_bit_and() {
        assert_binary_eval(ClosureOp::BitAnd, Policy::Checked, |a: u8, b| {
            Some(a & b)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_bit_and() {
        assert_binary_eval(ClosureOp::BitAnd, Policy::Wrapping, |a: u8, b| {
            Some(a & b)
        });
    }

    #[test]
    fn u8_eval_matches_checked_bit_or() {
        assert_binary_eval(ClosureOp::BitOr, Policy::Checked, |a: u8, b| {
            Some(a | b)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_bit_or() {
        assert_binary_eval(ClosureOp::BitOr, Policy::Wrapping, |a: u8, b| {
            Some(a | b)
        });
    }

    #[test]
    fn u8_eval_matches_checked_bit_xor() {
        assert_binary_eval(ClosureOp::BitXor, Policy::Checked, |a: u8, b| {
            Some(a ^ b)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_bit_xor() {
        assert_binary_eval(ClosureOp::BitXor, Policy::Wrapping, |a: u8, b| {
            Some(a ^ b)
        });
    }

    #[test]
    fn u8_eval_matches_checked_midpoint() {
        assert_binary_eval(ClosureOp::Midpoint, Policy::Checked, |a: u8, b| {
            Some(a.midpoint(b))
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_midpoint() {
        assert_binary_eval(
            ClosureOp::Midpoint,
            Policy::Wrapping,
            |a: u8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u8_eval_matches_checked_shr() {
        assert_shift_eval(ClosureOp::Shr, Policy::Checked, |a: u8, s| {
            a.checked_shr(s)
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_shr() {
        assert_shift_eval(ClosureOp::Shr, Policy::Wrapping, |a: u8, s| {
            Some(a.wrapping_shr(s))
        });
    }

    #[test]
    fn u8_eval_matches_checked_unbounded_shr() {
        assert_shift_eval(
            ClosureOp::UnboundedShr,
            Policy::Checked,
            |a: u8, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn u8_eval_matches_wrapping_unbounded_shr() {
        assert_shift_eval(
            ClosureOp::UnboundedShr,
            Policy::Wrapping,
            |a: u8, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn u8_eval_matches_checked_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Checked, |a: u8| {
            a.checked_neg()
        });
    }

    #[test]
    fn u8_eval_matches_wrapping_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Wrapping, |a: u8| {
            Some(a.wrapping_neg())
        });
    }

    #[test]
    fn u8_eval_matches_checked_not() {
        assert_unary_eval(ClosureOp::Not, Policy::Checked, |a: u8| Some(!a));
    }

    #[test]
    fn u8_eval_matches_wrapping_not() {
        assert_unary_eval(ClosureOp::Not, Policy::Wrapping, |a: u8| Some(!a));
    }

    #[test]
    fn u8_eval_matches_checked_isqrt() {
        assert_unary_eval(ClosureOp::Isqrt, Policy::Checked, |a: u8| {
            Some(a.isqrt())
        });
    }

    // -------------------------------------------------------------------------
    // i8: soundness

    #[test]
    fn i8_checked_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| a.checked_add(b),
        );
    }

    #[test]
    fn i8_checked_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| a.checked_add(b),
        );
    }

    #[test]
    fn i8_wrapping_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| Some(a.wrapping_add(b)),
        );
    }

    #[test]
    fn i8_wrapping_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| Some(a.wrapping_add(b)),
        );
    }

    #[test]
    fn i8_saturating_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: i8, b| Some(a.saturating_add(b)),
        );
    }

    #[test]
    fn i8_saturating_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Saturating,
            |a: i8, b| Some(a.saturating_add(b)),
        );
    }

    #[test]
    fn i8_checked_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| a.checked_sub(b),
        );
    }

    #[test]
    fn i8_checked_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| a.checked_sub(b),
        );
    }

    #[test]
    fn i8_wrapping_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| Some(a.wrapping_sub(b)),
        );
    }

    #[test]
    fn i8_wrapping_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| Some(a.wrapping_sub(b)),
        );
    }

    #[test]
    fn i8_saturating_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: i8, b| Some(a.saturating_sub(b)),
        );
    }

    #[test]
    fn i8_saturating_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Saturating,
            |a: i8, b| Some(a.saturating_sub(b)),
        );
    }

    #[test]
    fn i8_checked_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| a.checked_mul(b),
        );
    }

    #[test]
    fn i8_checked_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| a.checked_mul(b),
        );
    }

    #[test]
    fn i8_wrapping_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| Some(a.wrapping_mul(b)),
        );
    }

    #[test]
    fn i8_wrapping_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| Some(a.wrapping_mul(b)),
        );
    }

    #[test]
    fn i8_saturating_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: i8, b| Some(a.saturating_mul(b)),
        );
    }

    #[test]
    fn i8_saturating_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Saturating,
            |a: i8, b| Some(a.saturating_mul(b)),
        );
    }

    #[test]
    fn i8_checked_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| a.checked_div(b),
        );
    }

    #[test]
    fn i8_checked_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| a.checked_div(b),
        );
    }

    #[test]
    fn i8_wrapping_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_div(b)),
        );
    }

    #[test]
    fn i8_wrapping_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_div(b)),
        );
    }

    #[test]
    fn i8_saturating_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: i8, b| (b != 0).then(|| a.saturating_div(b)),
        );
    }

    #[test]
    fn i8_saturating_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Saturating,
            |a: i8, b| (b != 0).then(|| a.saturating_div(b)),
        );
    }

    #[test]
    fn i8_checked_rem_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| a.checked_rem(b),
        );
    }

    #[test]
    fn i8_checked_rem_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| a.checked_rem(b),
        );
    }

    #[test]
    fn i8_wrapping_rem_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_rem(b)),
        );
    }

    #[test]
    fn i8_wrapping_rem_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_rem(b)),
        );
    }

    #[test]
    fn i8_checked_div_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn i8_checked_div_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn i8_wrapping_div_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn i8_wrapping_div_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn i8_checked_rem_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn i8_checked_rem_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn i8_wrapping_rem_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn i8_wrapping_rem_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn i8_checked_bit_and_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| Some(a & b),
        );
    }

    #[test]
    fn i8_checked_bit_and_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| Some(a & b),
        );
    }

    #[test]
    fn i8_wrapping_bit_and_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| Some(a & b),
        );
    }

    #[test]
    fn i8_wrapping_bit_and_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| Some(a & b),
        );
    }

    #[test]
    fn i8_checked_bit_or_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| Some(a | b),
        );
    }

    #[test]
    fn i8_checked_bit_or_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| Some(a | b),
        );
    }

    #[test]
    fn i8_wrapping_bit_or_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| Some(a | b),
        );
    }

    #[test]
    fn i8_wrapping_bit_or_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| Some(a | b),
        );
    }

    #[test]
    fn i8_checked_bit_xor_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| Some(a ^ b),
        );
    }

    #[test]
    fn i8_checked_bit_xor_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| Some(a ^ b),
        );
    }

    #[test]
    fn i8_wrapping_bit_xor_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| Some(a ^ b),
        );
    }

    #[test]
    fn i8_wrapping_bit_xor_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| Some(a ^ b),
        );
    }

    #[test]
    fn i8_checked_midpoint_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i8_checked_midpoint_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::Full,
            Policy::Checked,
            |a: i8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i8_wrapping_midpoint_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i8_wrapping_midpoint_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::Full,
            Policy::Wrapping,
            |a: i8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i8_checked_shr_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Shr,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i8, b| u32::try_from(b).ok().and_then(|s| a.checked_shr(s)),
        );
    }

    #[test]
    fn i8_wrapping_shr_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Shr,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i8, b| Some(a.wrapping_shr(b as u32)),
        );
    }

    #[test]
    fn i8_checked_shr_amount_is_sound() {
        assert_shift_sound(ClosureOp::Shr, Policy::Checked, |a: i8, s| {
            a.checked_shr(s)
        });
    }

    #[test]
    fn i8_wrapping_shr_amount_is_sound() {
        assert_shift_sound(ClosureOp::Shr, Policy::Wrapping, |a: i8, s| {
            Some(a.wrapping_shr(s))
        });
    }

    #[test]
    fn i8_checked_unbounded_shr_amount_is_sound() {
        assert_shift_sound(
            ClosureOp::UnboundedShr,
            Policy::Checked,
            |a: i8, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn i8_wrapping_unbounded_shr_amount_is_sound() {
        assert_shift_sound(
            ClosureOp::UnboundedShr,
            Policy::Wrapping,
            |a: i8, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn i8_checked_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Checked, |a: i8| {
            a.checked_neg()
        });
    }

    #[test]
    fn i8_wrapping_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Wrapping, |a: i8| {
            Some(a.wrapping_neg())
        });
    }

    #[test]
    fn i8_checked_not_is_sound() {
        assert_unary_sound(ClosureOp::Not, Policy::Checked, |a: i8| Some(!a));
    }

    #[test]
    fn i8_wrapping_not_is_sound() {
        assert_unary_sound(ClosureOp::Not, Policy::Wrapping, |a: i8| Some(!a));
    }

    #[test]
    fn i8_saturating_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Saturating, |a: i8| {
            Some(a.saturating_neg())
        });
    }

    #[test]
    fn i8_checked_abs_is_sound() {
        assert_unary_sound(ClosureOp::Abs, Policy::Checked, |a: i8| {
            a.checked_abs()
        });
    }

    #[test]
    fn i8_wrapping_abs_is_sound() {
        assert_unary_sound(ClosureOp::Abs, Policy::Wrapping, |a: i8| {
            Some(a.wrapping_abs())
        });
    }

    #[test]
    fn i8_saturating_abs_is_sound() {
        assert_unary_sound(ClosureOp::Abs, Policy::Saturating, |a: i8| {
            Some(a.saturating_abs())
        });
    }

    #[test]
    fn i8_checked_isqrt_is_sound() {
        assert_unary_sound(ClosureOp::Isqrt, Policy::Checked, |a: i8| {
            a.checked_isqrt()
        });
    }

    // -------------------------------------------------------------------------
    // i8: eval

    #[test]
    fn i8_eval_matches_checked_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Checked, |a: i8, b| {
            a.checked_add(b)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Wrapping, |a: i8, b| {
            Some(a.wrapping_add(b))
        });
    }

    #[test]
    fn i8_eval_matches_saturating_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Saturating, |a: i8, b| {
            Some(a.saturating_add(b))
        });
    }

    #[test]
    fn i8_eval_matches_checked_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Checked, |a: i8, b| {
            a.checked_sub(b)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Wrapping, |a: i8, b| {
            Some(a.wrapping_sub(b))
        });
    }

    #[test]
    fn i8_eval_matches_saturating_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Saturating, |a: i8, b| {
            Some(a.saturating_sub(b))
        });
    }

    #[test]
    fn i8_eval_matches_checked_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Checked, |a: i8, b| {
            a.checked_mul(b)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Wrapping, |a: i8, b| {
            Some(a.wrapping_mul(b))
        });
    }

    #[test]
    fn i8_eval_matches_saturating_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Saturating, |a: i8, b| {
            Some(a.saturating_mul(b))
        });
    }

    #[test]
    fn i8_eval_matches_checked_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Checked, |a: i8, b| {
            a.checked_div(b)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Wrapping, |a: i8, b| {
            (b != 0).then(|| a.wrapping_div(b))
        });
    }

    #[test]
    fn i8_eval_matches_saturating_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Saturating, |a: i8, b| {
            (b != 0).then(|| a.saturating_div(b))
        });
    }

    #[test]
    fn i8_eval_matches_checked_rem() {
        assert_binary_eval(ClosureOp::Rem, Policy::Checked, |a: i8, b| {
            a.checked_rem(b)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_rem() {
        assert_binary_eval(ClosureOp::Rem, Policy::Wrapping, |a: i8, b| {
            (b != 0).then(|| a.wrapping_rem(b))
        });
    }

    #[test]
    fn i8_eval_matches_checked_div_euclid() {
        assert_binary_eval(
            ClosureOp::DivEuclid,
            Policy::Checked,
            |a: i8, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn i8_eval_matches_wrapping_div_euclid() {
        assert_binary_eval(
            ClosureOp::DivEuclid,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn i8_eval_matches_checked_rem_euclid() {
        assert_binary_eval(
            ClosureOp::RemEuclid,
            Policy::Checked,
            |a: i8, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn i8_eval_matches_wrapping_rem_euclid() {
        assert_binary_eval(
            ClosureOp::RemEuclid,
            Policy::Wrapping,
            |a: i8, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn i8_eval_matches_checked_bit_and() {
        assert_binary_eval(ClosureOp::BitAnd, Policy::Checked, |a: i8, b| {
            Some(a & b)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_bit_and() {
        assert_binary_eval(ClosureOp::BitAnd, Policy::Wrapping, |a: i8, b| {
            Some(a & b)
        });
    }

    #[test]
    fn i8_eval_matches_checked_bit_or() {
        assert_binary_eval(ClosureOp::BitOr, Policy::Checked, |a: i8, b| {
            Some(a | b)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_bit_or() {
        assert_binary_eval(ClosureOp::BitOr, Policy::Wrapping, |a: i8, b| {
            Some(a | b)
        });
    }

    #[test]
    fn i8_eval_matches_checked_bit_xor() {
        assert_binary_eval(ClosureOp::BitXor, Policy::Checked, |a: i8, b| {
            Some(a ^ b)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_bit_xor() {
        assert_binary_eval(ClosureOp::BitXor, Policy::Wrapping, |a: i8, b| {
            Some(a ^ b)
        });
    }

    #[test]
    fn i8_eval_matches_checked_midpoint() {
        assert_binary_eval(ClosureOp::Midpoint, Policy::Checked, |a: i8, b| {
            Some(a.midpoint(b))
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_midpoint() {
        assert_binary_eval(
            ClosureOp::Midpoint,
            Policy::Wrapping,
            |a: i8, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i8_eval_matches_checked_shr() {
        assert_shift_eval(ClosureOp::Shr, Policy::Checked, |a: i8, s| {
            a.checked_shr(s)
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_shr() {
        assert_shift_eval(ClosureOp::Shr, Policy::Wrapping, |a: i8, s| {
            Some(a.wrapping_shr(s))
        });
    }

    #[test]
    fn i8_eval_matches_checked_unbounded_shr() {
        assert_shift_eval(
            ClosureOp::UnboundedShr,
            Policy::Checked,
            |a: i8, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn i8_eval_matches_wrapping_unbounded_shr() {
        assert_shift_eval(
            ClosureOp::UnboundedShr,
            Policy::Wrapping,
            |a: i8, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn i8_eval_matches_checked_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Checked, |a: i8| {
            a.checked_neg()
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Wrapping, |a: i8| {
            Some(a.wrapping_neg())
        });
    }

    #[test]
    fn i8_eval_matches_checked_not() {
        assert_unary_eval(ClosureOp::Not, Policy::Checked, |a: i8| Some(!a));
    }

    #[test]
    fn i8_eval_matches_wrapping_not() {
        assert_unary_eval(ClosureOp::Not, Policy::Wrapping, |a: i8| Some(!a));
    }

    #[test]
    fn i8_eval_matches_saturating_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Saturating, |a: i8| {
            Some(a.saturating_neg())
        });
    }

    #[test]
    fn i8_eval_matches_checked_abs() {
        assert_unary_eval(ClosureOp::Abs, Policy::Checked, |a: i8| {
            a.checked_abs()
        });
    }

    #[test]
    fn i8_eval_matches_wrapping_abs() {
        assert_unary_eval(ClosureOp::Abs, Policy::Wrapping, |a: i8| {
            Some(a.wrapping_abs())
        });
    }

    #[test]
    fn i8_eval_matches_saturating_abs() {
        assert_unary_eval(ClosureOp::Abs, Policy::Saturating, |a: i8| {
            Some(a.saturating_abs())
        });
    }

    #[test]
    fn i8_eval_matches_checked_isqrt() {
        assert_unary_eval(ClosureOp::Isqrt, Policy::Checked, |a: i8| {
            a.checked_isqrt()
        });
    }

    // -------------------------------------------------------------------------
    // u128: soundness on boundary samples

    #[test]
    fn u128_checked_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| a.checked_add(b),
        );
    }

    #[test]
    fn u128_checked_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| a.checked_add(b),
        );
    }

    #[test]
    fn u128_wrapping_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| Some(a.wrapping_add(b)),
        );
    }

    #[test]
    fn u128_wrapping_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| Some(a.wrapping_add(b)),
        );
    }

    #[test]
    fn u128_saturating_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: u128, b| Some(a.saturating_add(b)),
        );
    }

    #[test]
    fn u128_saturating_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Saturating,
            |a: u128, b| Some(a.saturating_add(b)),
        );
    }

    #[test]
    fn u128_checked_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| a.checked_sub(b),
        );
    }

    #[test]
    fn u128_checked_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| a.checked_sub(b),
        );
    }

    #[test]
    fn u128_wrapping_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| Some(a.wrapping_sub(b)),
        );
    }

    #[test]
    fn u128_wrapping_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| Some(a.wrapping_sub(b)),
        );
    }

    #[test]
    fn u128_saturating_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: u128, b| Some(a.saturating_sub(b)),
        );
    }

    #[test]
    fn u128_saturating_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Saturating,
            |a: u128, b| Some(a.saturating_sub(b)),
        );
    }

    #[test]
    fn u128_checked_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| a.checked_mul(b),
        );
    }

    #[test]
    fn u128_checked_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| a.checked_mul(b),
        );
    }

    #[test]
    fn u128_wrapping_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| Some(a.wrapping_mul(b)),
        );
    }

    #[test]
    fn u128_wrapping_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| Some(a.wrapping_mul(b)),
        );
    }

    #[test]
    fn u128_saturating_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: u128, b| Some(a.saturating_mul(b)),
        );
    }

    #[test]
    fn u128_saturating_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Saturating,
            |a: u128, b| Some(a.saturating_mul(b)),
        );
    }

    #[test]
    fn u128_checked_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| a.checked_div(b),
        );
    }

    #[test]
    fn u128_checked_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| a.checked_div(b),
        );
    }

    #[test]
    fn u128_wrapping_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_div(b)),
        );
    }

    #[test]
    fn u128_wrapping_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_div(b)),
        );
    }

    #[test]
    fn u128_saturating_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: u128, b| (b != 0).then(|| a.saturating_div(b)),
        );
    }

    #[test]
    fn u128_saturating_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Saturating,
            |a: u128, b| (b != 0).then(|| a.saturating_div(b)),
        );
    }

    #[test]
    fn u128_checked_rem_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| a.checked_rem(b),
        );
    }

    #[test]
    fn u128_checked_rem_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| a.checked_rem(b),
        );
    }

    #[test]
    fn u128_wrapping_rem_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_rem(b)),
        );
    }

    #[test]
    fn u128_wrapping_rem_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_rem(b)),
        );
    }

    #[test]
    fn u128_checked_div_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn u128_checked_div_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn u128_wrapping_div_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn u128_wrapping_div_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn u128_checked_rem_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn u128_checked_rem_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn u128_wrapping_rem_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn u128_wrapping_rem_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn u128_checked_bit_and_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| Some(a & b),
        );
    }

    #[test]
    fn u128_checked_bit_and_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| Some(a & b),
        );
    }

    #[test]
    fn u128_wrapping_bit_and_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| Some(a & b),
        );
    }

    #[test]
    fn u128_wrapping_bit_and_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| Some(a & b),
        );
    }

    #[test]
    fn u128_checked_bit_or_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| Some(a | b),
        );
    }

    #[test]
    fn u128_checked_bit_or_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| Some(a | b),
        );
    }

    #[test]
    fn u128_wrapping_bit_or_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| Some(a | b),
        );
    }

    #[test]
    fn u128_wrapping_bit_or_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| Some(a | b),
        );
    }

    #[test]
    fn u128_checked_bit_xor_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| Some(a ^ b),
        );
    }

    #[test]
    fn u128_checked_bit_xor_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| Some(a ^ b),
        );
    }

    #[test]
    fn u128_wrapping_bit_xor_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| Some(a ^ b),
        );
    }

    #[test]
    fn u128_wrapping_bit_xor_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| Some(a ^ b),
        );
    }

    #[test]
    fn u128_checked_midpoint_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u128_checked_midpoint_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::Full,
            Policy::Checked,
            |a: u128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u128_wrapping_midpoint_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u128_wrapping_midpoint_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::Full,
            Policy::Wrapping,
            |a: u128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u128_checked_shr_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Shr,
            Rhs::SameDomain,
            Policy::Checked,
            |a: u128, b| u32::try_from(b).ok().and_then(|s| a.checked_shr(s)),
        );
    }

    #[test]
    fn u128_wrapping_shr_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Shr,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: u128, b| Some(a.wrapping_shr(b as u32)),
        );
    }

    #[test]
    fn u128_checked_shr_amount_is_sound() {
        assert_shift_sound(ClosureOp::Shr, Policy::Checked, |a: u128, s| {
            a.checked_shr(s)
        });
    }

    #[test]
    fn u128_wrapping_shr_amount_is_sound() {
        assert_shift_sound(ClosureOp::Shr, Policy::Wrapping, |a: u128, s| {
            Some(a.wrapping_shr(s))
        });
    }

    #[test]
    fn u128_checked_unbounded_shr_amount_is_sound() {
        assert_shift_sound(
            ClosureOp::UnboundedShr,
            Policy::Checked,
            |a: u128, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn u128_wrapping_unbounded_shr_amount_is_sound() {
        assert_shift_sound(
            ClosureOp::UnboundedShr,
            Policy::Wrapping,
            |a: u128, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn u128_checked_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Checked, |a: u128| {
            a.checked_neg()
        });
    }

    #[test]
    fn u128_wrapping_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Wrapping, |a: u128| {
            Some(a.wrapping_neg())
        });
    }

    #[test]
    fn u128_checked_not_is_sound() {
        assert_unary_sound(ClosureOp::Not, Policy::Checked, |a: u128| Some(!a));
    }

    #[test]
    fn u128_wrapping_not_is_sound() {
        assert_unary_sound(ClosureOp::Not, Policy::Wrapping, |a: u128| {
            Some(!a)
        });
    }

    #[test]
    fn u128_checked_isqrt_is_sound() {
        assert_unary_sound(ClosureOp::Isqrt, Policy::Checked, |a: u128| {
            Some(a.isqrt())
        });
    }

    // -------------------------------------------------------------------------
    // u128: eval

    #[test]
    fn u128_eval_matches_checked_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Checked, |a: u128, b| {
            a.checked_add(b)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Wrapping, |a: u128, b| {
            Some(a.wrapping_add(b))
        });
    }

    #[test]
    fn u128_eval_matches_saturating_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Saturating, |a: u128, b| {
            Some(a.saturating_add(b))
        });
    }

    #[test]
    fn u128_eval_matches_checked_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Checked, |a: u128, b| {
            a.checked_sub(b)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Wrapping, |a: u128, b| {
            Some(a.wrapping_sub(b))
        });
    }

    #[test]
    fn u128_eval_matches_saturating_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Saturating, |a: u128, b| {
            Some(a.saturating_sub(b))
        });
    }

    #[test]
    fn u128_eval_matches_checked_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Checked, |a: u128, b| {
            a.checked_mul(b)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Wrapping, |a: u128, b| {
            Some(a.wrapping_mul(b))
        });
    }

    #[test]
    fn u128_eval_matches_saturating_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Saturating, |a: u128, b| {
            Some(a.saturating_mul(b))
        });
    }

    #[test]
    fn u128_eval_matches_checked_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Checked, |a: u128, b| {
            a.checked_div(b)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Wrapping, |a: u128, b| {
            (b != 0).then(|| a.wrapping_div(b))
        });
    }

    #[test]
    fn u128_eval_matches_saturating_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Saturating, |a: u128, b| {
            (b != 0).then(|| a.saturating_div(b))
        });
    }

    #[test]
    fn u128_eval_matches_checked_rem() {
        assert_binary_eval(ClosureOp::Rem, Policy::Checked, |a: u128, b| {
            a.checked_rem(b)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_rem() {
        assert_binary_eval(ClosureOp::Rem, Policy::Wrapping, |a: u128, b| {
            (b != 0).then(|| a.wrapping_rem(b))
        });
    }

    #[test]
    fn u128_eval_matches_checked_div_euclid() {
        assert_binary_eval(
            ClosureOp::DivEuclid,
            Policy::Checked,
            |a: u128, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn u128_eval_matches_wrapping_div_euclid() {
        assert_binary_eval(
            ClosureOp::DivEuclid,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn u128_eval_matches_checked_rem_euclid() {
        assert_binary_eval(
            ClosureOp::RemEuclid,
            Policy::Checked,
            |a: u128, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn u128_eval_matches_wrapping_rem_euclid() {
        assert_binary_eval(
            ClosureOp::RemEuclid,
            Policy::Wrapping,
            |a: u128, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn u128_eval_matches_checked_bit_and() {
        assert_binary_eval(ClosureOp::BitAnd, Policy::Checked, |a: u128, b| {
            Some(a & b)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_bit_and() {
        assert_binary_eval(
            ClosureOp::BitAnd,
            Policy::Wrapping,
            |a: u128, b| Some(a & b),
        );
    }

    #[test]
    fn u128_eval_matches_checked_bit_or() {
        assert_binary_eval(ClosureOp::BitOr, Policy::Checked, |a: u128, b| {
            Some(a | b)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_bit_or() {
        assert_binary_eval(ClosureOp::BitOr, Policy::Wrapping, |a: u128, b| {
            Some(a | b)
        });
    }

    #[test]
    fn u128_eval_matches_checked_bit_xor() {
        assert_binary_eval(ClosureOp::BitXor, Policy::Checked, |a: u128, b| {
            Some(a ^ b)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_bit_xor() {
        assert_binary_eval(
            ClosureOp::BitXor,
            Policy::Wrapping,
            |a: u128, b| Some(a ^ b),
        );
    }

    #[test]
    fn u128_eval_matches_checked_midpoint() {
        assert_binary_eval(
            ClosureOp::Midpoint,
            Policy::Checked,
            |a: u128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u128_eval_matches_wrapping_midpoint() {
        assert_binary_eval(
            ClosureOp::Midpoint,
            Policy::Wrapping,
            |a: u128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn u128_eval_matches_checked_shr() {
        assert_shift_eval(ClosureOp::Shr, Policy::Checked, |a: u128, s| {
            a.checked_shr(s)
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_shr() {
        assert_shift_eval(ClosureOp::Shr, Policy::Wrapping, |a: u128, s| {
            Some(a.wrapping_shr(s))
        });
    }

    #[test]
    fn u128_eval_matches_checked_unbounded_shr() {
        assert_shift_eval(
            ClosureOp::UnboundedShr,
            Policy::Checked,
            |a: u128, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn u128_eval_matches_wrapping_unbounded_shr() {
        assert_shift_eval(
            ClosureOp::UnboundedShr,
            Policy::Wrapping,
            |a: u128, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn u128_eval_matches_checked_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Checked, |a: u128| {
            a.checked_neg()
        });
    }

    #[test]
    fn u128_eval_matches_wrapping_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Wrapping, |a: u128| {
            Some(a.wrapping_neg())
        });
    }

    #[test]
    fn u128_eval_matches_checked_not() {
        assert_unary_eval(ClosureOp::Not, Policy::Checked, |a: u128| Some(!a));
    }

    #[test]
    fn u128_eval_matches_wrapping_not() {
        assert_unary_eval(ClosureOp::Not, Policy::Wrapping, |a: u128| Some(!a));
    }

    #[test]
    fn u128_eval_matches_checked_isqrt() {
        assert_unary_eval(ClosureOp::Isqrt, Policy::Checked, |a: u128| {
            Some(a.isqrt())
        });
    }

    // -------------------------------------------------------------------------
    // i128: soundness on boundary samples

    #[test]
    fn i128_checked_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| a.checked_add(b),
        );
    }

    #[test]
    fn i128_checked_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| a.checked_add(b),
        );
    }

    #[test]
    fn i128_wrapping_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| Some(a.wrapping_add(b)),
        );
    }

    #[test]
    fn i128_wrapping_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| Some(a.wrapping_add(b)),
        );
    }

    #[test]
    fn i128_saturating_add_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: i128, b| Some(a.saturating_add(b)),
        );
    }

    #[test]
    fn i128_saturating_add_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Add,
            Rhs::Full,
            Policy::Saturating,
            |a: i128, b| Some(a.saturating_add(b)),
        );
    }

    #[test]
    fn i128_checked_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| a.checked_sub(b),
        );
    }

    #[test]
    fn i128_checked_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| a.checked_sub(b),
        );
    }

    #[test]
    fn i128_wrapping_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| Some(a.wrapping_sub(b)),
        );
    }

    #[test]
    fn i128_wrapping_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| Some(a.wrapping_sub(b)),
        );
    }

    #[test]
    fn i128_saturating_sub_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: i128, b| Some(a.saturating_sub(b)),
        );
    }

    #[test]
    fn i128_saturating_sub_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Sub,
            Rhs::Full,
            Policy::Saturating,
            |a: i128, b| Some(a.saturating_sub(b)),
        );
    }

    #[test]
    fn i128_checked_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| a.checked_mul(b),
        );
    }

    #[test]
    fn i128_checked_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| a.checked_mul(b),
        );
    }

    #[test]
    fn i128_wrapping_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| Some(a.wrapping_mul(b)),
        );
    }

    #[test]
    fn i128_wrapping_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| Some(a.wrapping_mul(b)),
        );
    }

    #[test]
    fn i128_saturating_mul_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: i128, b| Some(a.saturating_mul(b)),
        );
    }

    #[test]
    fn i128_saturating_mul_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Mul,
            Rhs::Full,
            Policy::Saturating,
            |a: i128, b| Some(a.saturating_mul(b)),
        );
    }

    #[test]
    fn i128_checked_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| a.checked_div(b),
        );
    }

    #[test]
    fn i128_checked_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| a.checked_div(b),
        );
    }

    #[test]
    fn i128_wrapping_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_div(b)),
        );
    }

    #[test]
    fn i128_wrapping_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_div(b)),
        );
    }

    #[test]
    fn i128_saturating_div_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::SameDomain,
            Policy::Saturating,
            |a: i128, b| (b != 0).then(|| a.saturating_div(b)),
        );
    }

    #[test]
    fn i128_saturating_div_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Div,
            Rhs::Full,
            Policy::Saturating,
            |a: i128, b| (b != 0).then(|| a.saturating_div(b)),
        );
    }

    #[test]
    fn i128_checked_rem_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| a.checked_rem(b),
        );
    }

    #[test]
    fn i128_checked_rem_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| a.checked_rem(b),
        );
    }

    #[test]
    fn i128_wrapping_rem_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_rem(b)),
        );
    }

    #[test]
    fn i128_wrapping_rem_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Rem,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_rem(b)),
        );
    }

    #[test]
    fn i128_checked_div_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn i128_checked_div_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn i128_wrapping_div_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn i128_wrapping_div_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::DivEuclid,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn i128_checked_rem_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn i128_checked_rem_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn i128_wrapping_rem_euclid_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn i128_wrapping_rem_euclid_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::RemEuclid,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn i128_checked_bit_and_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| Some(a & b),
        );
    }

    #[test]
    fn i128_checked_bit_and_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| Some(a & b),
        );
    }

    #[test]
    fn i128_wrapping_bit_and_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| Some(a & b),
        );
    }

    #[test]
    fn i128_wrapping_bit_and_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitAnd,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| Some(a & b),
        );
    }

    #[test]
    fn i128_checked_bit_or_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| Some(a | b),
        );
    }

    #[test]
    fn i128_checked_bit_or_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| Some(a | b),
        );
    }

    #[test]
    fn i128_wrapping_bit_or_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| Some(a | b),
        );
    }

    #[test]
    fn i128_wrapping_bit_or_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitOr,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| Some(a | b),
        );
    }

    #[test]
    fn i128_checked_bit_xor_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| Some(a ^ b),
        );
    }

    #[test]
    fn i128_checked_bit_xor_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| Some(a ^ b),
        );
    }

    #[test]
    fn i128_wrapping_bit_xor_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| Some(a ^ b),
        );
    }

    #[test]
    fn i128_wrapping_bit_xor_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::BitXor,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| Some(a ^ b),
        );
    }

    #[test]
    fn i128_checked_midpoint_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i128_checked_midpoint_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::Full,
            Policy::Checked,
            |a: i128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i128_wrapping_midpoint_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i128_wrapping_midpoint_primitive_is_sound() {
        assert_binary_sound(
            ClosureOp::Midpoint,
            Rhs::Full,
            Policy::Wrapping,
            |a: i128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i128_checked_shr_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Shr,
            Rhs::SameDomain,
            Policy::Checked,
            |a: i128, b| u32::try_from(b).ok().and_then(|s| a.checked_shr(s)),
        );
    }

    #[test]
    fn i128_wrapping_shr_same_domain_is_sound() {
        assert_binary_sound(
            ClosureOp::Shr,
            Rhs::SameDomain,
            Policy::Wrapping,
            |a: i128, b| Some(a.wrapping_shr(b as u32)),
        );
    }

    #[test]
    fn i128_checked_shr_amount_is_sound() {
        assert_shift_sound(ClosureOp::Shr, Policy::Checked, |a: i128, s| {
            a.checked_shr(s)
        });
    }

    #[test]
    fn i128_wrapping_shr_amount_is_sound() {
        assert_shift_sound(ClosureOp::Shr, Policy::Wrapping, |a: i128, s| {
            Some(a.wrapping_shr(s))
        });
    }

    #[test]
    fn i128_checked_unbounded_shr_amount_is_sound() {
        assert_shift_sound(
            ClosureOp::UnboundedShr,
            Policy::Checked,
            |a: i128, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn i128_wrapping_unbounded_shr_amount_is_sound() {
        assert_shift_sound(
            ClosureOp::UnboundedShr,
            Policy::Wrapping,
            |a: i128, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn i128_checked_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Checked, |a: i128| {
            a.checked_neg()
        });
    }

    #[test]
    fn i128_wrapping_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Wrapping, |a: i128| {
            Some(a.wrapping_neg())
        });
    }

    #[test]
    fn i128_checked_not_is_sound() {
        assert_unary_sound(ClosureOp::Not, Policy::Checked, |a: i128| Some(!a));
    }

    #[test]
    fn i128_wrapping_not_is_sound() {
        assert_unary_sound(ClosureOp::Not, Policy::Wrapping, |a: i128| {
            Some(!a)
        });
    }

    #[test]
    fn i128_saturating_neg_is_sound() {
        assert_unary_sound(ClosureOp::Neg, Policy::Saturating, |a: i128| {
            Some(a.saturating_neg())
        });
    }

    #[test]
    fn i128_checked_abs_is_sound() {
        assert_unary_sound(ClosureOp::Abs, Policy::Checked, |a: i128| {
            a.checked_abs()
        });
    }

    #[test]
    fn i128_wrapping_abs_is_sound() {
        assert_unary_sound(ClosureOp::Abs, Policy::Wrapping, |a: i128| {
            Some(a.wrapping_abs())
        });
    }

    #[test]
    fn i128_saturating_abs_is_sound() {
        assert_unary_sound(ClosureOp::Abs, Policy::Saturating, |a: i128| {
            Some(a.saturating_abs())
        });
    }

    #[test]
    fn i128_checked_isqrt_is_sound() {
        assert_unary_sound(ClosureOp::Isqrt, Policy::Checked, |a: i128| {
            a.checked_isqrt()
        });
    }

    // -------------------------------------------------------------------------
    // i128: eval

    #[test]
    fn i128_eval_matches_checked_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Checked, |a: i128, b| {
            a.checked_add(b)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Wrapping, |a: i128, b| {
            Some(a.wrapping_add(b))
        });
    }

    #[test]
    fn i128_eval_matches_saturating_add() {
        assert_binary_eval(ClosureOp::Add, Policy::Saturating, |a: i128, b| {
            Some(a.saturating_add(b))
        });
    }

    #[test]
    fn i128_eval_matches_checked_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Checked, |a: i128, b| {
            a.checked_sub(b)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Wrapping, |a: i128, b| {
            Some(a.wrapping_sub(b))
        });
    }

    #[test]
    fn i128_eval_matches_saturating_sub() {
        assert_binary_eval(ClosureOp::Sub, Policy::Saturating, |a: i128, b| {
            Some(a.saturating_sub(b))
        });
    }

    #[test]
    fn i128_eval_matches_checked_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Checked, |a: i128, b| {
            a.checked_mul(b)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Wrapping, |a: i128, b| {
            Some(a.wrapping_mul(b))
        });
    }

    #[test]
    fn i128_eval_matches_saturating_mul() {
        assert_binary_eval(ClosureOp::Mul, Policy::Saturating, |a: i128, b| {
            Some(a.saturating_mul(b))
        });
    }

    #[test]
    fn i128_eval_matches_checked_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Checked, |a: i128, b| {
            a.checked_div(b)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Wrapping, |a: i128, b| {
            (b != 0).then(|| a.wrapping_div(b))
        });
    }

    #[test]
    fn i128_eval_matches_saturating_div() {
        assert_binary_eval(ClosureOp::Div, Policy::Saturating, |a: i128, b| {
            (b != 0).then(|| a.saturating_div(b))
        });
    }

    #[test]
    fn i128_eval_matches_checked_rem() {
        assert_binary_eval(ClosureOp::Rem, Policy::Checked, |a: i128, b| {
            a.checked_rem(b)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_rem() {
        assert_binary_eval(ClosureOp::Rem, Policy::Wrapping, |a: i128, b| {
            (b != 0).then(|| a.wrapping_rem(b))
        });
    }

    #[test]
    fn i128_eval_matches_checked_div_euclid() {
        assert_binary_eval(
            ClosureOp::DivEuclid,
            Policy::Checked,
            |a: i128, b| a.checked_div_euclid(b),
        );
    }

    #[test]
    fn i128_eval_matches_wrapping_div_euclid() {
        assert_binary_eval(
            ClosureOp::DivEuclid,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_div_euclid(b)),
        );
    }

    #[test]
    fn i128_eval_matches_checked_rem_euclid() {
        assert_binary_eval(
            ClosureOp::RemEuclid,
            Policy::Checked,
            |a: i128, b| a.checked_rem_euclid(b),
        );
    }

    #[test]
    fn i128_eval_matches_wrapping_rem_euclid() {
        assert_binary_eval(
            ClosureOp::RemEuclid,
            Policy::Wrapping,
            |a: i128, b| (b != 0).then(|| a.wrapping_rem_euclid(b)),
        );
    }

    #[test]
    fn i128_eval_matches_checked_bit_and() {
        assert_binary_eval(ClosureOp::BitAnd, Policy::Checked, |a: i128, b| {
            Some(a & b)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_bit_and() {
        assert_binary_eval(
            ClosureOp::BitAnd,
            Policy::Wrapping,
            |a: i128, b| Some(a & b),
        );
    }

    #[test]
    fn i128_eval_matches_checked_bit_or() {
        assert_binary_eval(ClosureOp::BitOr, Policy::Checked, |a: i128, b| {
            Some(a | b)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_bit_or() {
        assert_binary_eval(ClosureOp::BitOr, Policy::Wrapping, |a: i128, b| {
            Some(a | b)
        });
    }

    #[test]
    fn i128_eval_matches_checked_bit_xor() {
        assert_binary_eval(ClosureOp::BitXor, Policy::Checked, |a: i128, b| {
            Some(a ^ b)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_bit_xor() {
        assert_binary_eval(
            ClosureOp::BitXor,
            Policy::Wrapping,
            |a: i128, b| Some(a ^ b),
        );
    }

    #[test]
    fn i128_eval_matches_checked_midpoint() {
        assert_binary_eval(
            ClosureOp::Midpoint,
            Policy::Checked,
            |a: i128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i128_eval_matches_wrapping_midpoint() {
        assert_binary_eval(
            ClosureOp::Midpoint,
            Policy::Wrapping,
            |a: i128, b| Some(a.midpoint(b)),
        );
    }

    #[test]
    fn i128_eval_matches_checked_shr() {
        assert_shift_eval(ClosureOp::Shr, Policy::Checked, |a: i128, s| {
            a.checked_shr(s)
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_shr() {
        assert_shift_eval(ClosureOp::Shr, Policy::Wrapping, |a: i128, s| {
            Some(a.wrapping_shr(s))
        });
    }

    #[test]
    fn i128_eval_matches_checked_unbounded_shr() {
        assert_shift_eval(
            ClosureOp::UnboundedShr,
            Policy::Checked,
            |a: i128, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn i128_eval_matches_wrapping_unbounded_shr() {
        assert_shift_eval(
            ClosureOp::UnboundedShr,
            Policy::Wrapping,
            |a: i128, s| Some(a.unbounded_shr(s)),
        );
    }

    #[test]
    fn i128_eval_matches_checked_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Checked, |a: i128| {
            a.checked_neg()
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Wrapping, |a: i128| {
            Some(a.wrapping_neg())
        });
    }

    #[test]
    fn i128_eval_matches_checked_not() {
        assert_unary_eval(ClosureOp::Not, Policy::Checked, |a: i128| Some(!a));
    }

    #[test]
    fn i128_eval_matches_wrapping_not() {
        assert_unary_eval(ClosureOp::Not, Policy::Wrapping, |a: i128| Some(!a));
    }

    #[test]
    fn i128_eval_matches_saturating_neg() {
        assert_unary_eval(ClosureOp::Neg, Policy::Saturating, |a: i128| {
            Some(a.saturating_neg())
        });
    }

    #[test]
    fn i128_eval_matches_checked_abs() {
        assert_unary_eval(ClosureOp::Abs, Policy::Checked, |a: i128| {
            a.checked_abs()
        });
    }

    #[test]
    fn i128_eval_matches_wrapping_abs() {
        assert_unary_eval(ClosureOp::Abs, Policy::Wrapping, |a: i128| {
            Some(a.wrapping_abs())
        });
    }

    #[test]
    fn i128_eval_matches_saturating_abs() {
        assert_unary_eval(ClosureOp::Abs, Policy::Saturating, |a: i128| {
            Some(a.saturating_abs())
        });
    }

    #[test]
    fn i128_eval_matches_checked_isqrt() {
        assert_unary_eval(ClosureOp::Isqrt, Policy::Checked, |a: i128| {
            a.checked_isqrt()
        });
    }
}
