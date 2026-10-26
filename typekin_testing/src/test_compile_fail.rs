use crate::*;
//noinspection RsConstantConditionIf
fn assert_rejected(
    name: &str,
    expected_error: &str,
    source: &str,
) {
    let output = compile(name, "", source, true);

    assert!(
        !output.status.success(),
        "unexpectedly compiled: name={} output={:?}",
        name,
        output,
    );
    assert!(
        output.stderr.contains(expected_error),
        "rejected for an unexpected reason; name={}, expected={}, output={:?}",
        name,
        expected_error,
        output
    );
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod tests {
    use super::*;

    #[test]
    fn integral_in_property_was_renamed() {
        const SRC: &str = r#"
                #[typekin::integral(in = 1..)]
                struct Code(u8);
            "#;

        assert_rejected(
            "integral-in-property-was-renamed",
            "`in` was renamed to `domain`",
            SRC,
        );
    }

    #[test]
    fn text_in_property_was_renamed() {
        const SRC: &str = r#"
                #[typekin::text(in = ["draft"])]
                struct State(String);
            "#;

        assert_rejected(
            "text-in-property-was-renamed",
            "`in` was renamed to `domain`",
            SRC,
        );
    }

    #[test]
    fn facet_cannot_select_non_zero_storage() {
        const SRC: &str = r#"
                #[typekin::integral(
                    facet = [
                        int {
                            with = non_zero,
                        },
                    ],
                )]
                struct Code(u8);
            "#;

        assert_rejected(
            "facet-cannot-select-non-zero-storage",
            "`non_zero` is wrapper-wide and cannot be configured inside a facet",
            SRC,
        );
    }

    #[test]
    fn rewritten_integral_storage_requires_a_private_field() {
        const SRC: &str = r#"
                #[typekin::integral(domain = 1..)]
                struct Code(pub u8);
            "#;

        assert_rejected(
            "rewritten-integral-storage-requires-private-field",
            "integral storage can only rewrite a private tuple field",
            SRC,
        );
    }

    #[test]
    fn wrapping_storage_requires_a_private_field() {
        const SRC: &str = r#"
                #[typekin::integral(profile = wrapping {})]
                struct Code(pub u8);
            "#;

        assert_rejected(
            "wrapping-storage-requires-private-field",
            "integral storage can only rewrite a private tuple field",
            SRC,
        );
    }

    #[test]
    fn non_zero_storage_rejects_a_custom_unchecked_constructor() {
        const SRC: &str = r#"
                #[typekin::integral(
                    domain = 1..,
                    without = fn_make_unchecked,
                )]
                struct Code(u8);

                impl Code {
                    const fn _unchecked(raw: u8) -> Self {
                        return unsafe { core::mem::transmute(raw) };
                    }
                }
            "#;

        assert_rejected(
            "non-zero-storage-rejects-custom-unchecked-constructor",
            "nonzero storage does not support custom unchecked constructors",
            SRC,
        );
    }

    #[test]
    fn bitflag_rejects_non_zero_storage() {
        const SRC: &str = r#"
                #[typekin::bitflag(integral = [with = non_zero])]
                enum Code {
                    One = 1,
                }
            "#;

        assert_rejected(
            "bitflag-rejects-non-zero-storage",
            "bitflag integral configuration does not support `non_zero`",
            SRC,
        );
    }

    #[test]
    fn bitflag_rejects_a_domain_without_zero() {
        const SRC: &str = r#"
                #[typekin::bitflag(integral = [domain = 1..])]
                enum Code {
                    One = 1,
                }
            "#;

        assert_rejected(
            "bitflag-rejects-domain-without-zero",
            "bitflag integral domain must include zero",
            SRC,
        );
    }

    #[test]
    fn validated_integral_has_no_make() {
        const SRC: &str = r#"
                const fn is_even(value: u8) -> bool { value % 2 == 0 }

                #[typekin::integral(konst = none, valid = is_even)]
                #[repr(transparent)]
                struct Checked(u8);

                fn requires_make() {
                    let _ = Checked::make(2);
                }
            "#;
        assert_rejected(
            "validated-integral-make",
            "no associated function or constant named `make`",
            SRC,
        );
    }

    #[test]
    fn make_does_not_grant_math() {
        const SRC: &str = r#"
                struct MakeSource(u8);
                fn into_u8(source: MakeSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = none,
                    friend = into_u8(MakeSource) -> Make,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_math() {
                    let _ = Number::make(1) + MakeSource(2);
                }
            "#;

        assert_rejected("make-does-not-grant-math", "Math", SRC);
    }

    #[test]
    fn bit_does_not_grant_math() {
        const SRC: &str = r#"
                struct BitSource(u8);
                fn into_u8(source: BitSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = none,
                    friend = into_u8(BitSource) -> Bit,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_math() {
                    let _ = Number::make(1) + BitSource(2);
                }
            "#;
        assert_rejected("bit-does-not-grant-math", "Math", SRC);
    }

    #[test]
    fn without_removes_raw() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    without = fn_conv_raw,
                    get_raw = Self::value,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                impl Number {
                    const fn value(self) -> u8 { self.0 }
                }

                fn requires_raw() {
                    let _ = Number::make(2).raw();
                }
            "#;

        assert_rejected("without-removes-raw", "no method named `raw`", SRC);
    }

    #[test]
    fn without_removes_self_math() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    without = self_math,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_self_math() {
                    let _ = Number::make(2) + Number::make(3);
                }
            "#;

        assert_rejected(
            "without-removes-self-math",
            "the trait `Math` is not implemented for `Number`",
            SRC,
        );
    }

    #[test]
    fn without_removes_self_bit() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    without = self_bit,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_self_bit() {
                    let _ = Number::make(2) | Number::make(3);
                }
            "#;
        assert_rejected(
            "without-removes-self-bit",
            "the trait `Bit` is not implemented for `Number`",
            SRC,
        );
    }

    #[test]
    fn output_type_removes_bitxor_assignment() {
        const SRC: &str = r#"
                struct Output;

                #[typekin::integral(
                    konst = none,
                    produce = Point::into_output(_) -> Output,
                )]
                struct Point(u8);

                impl Point {
                    fn into_output(self) -> Output { Output }
                }

                fn requires_bitxor_assignment() {
                    let mut point = Point::make(1);
                    point ^= Point::make(2);
                }
            "#;

        assert_rejected(
            "output-type-removes-bitxor-assignment",
            "BitXorAssign",
            SRC,
        );
    }

    #[test]
    fn point_profile_has_no_point_addition() {
        let source = "
            #[typekin::integral(konst = none, profile = point {})]
            struct Position(u8);

            fn forbidden() {
                let lhs = Position::make(1);
                let rhs = Position::make(2);
                let _ = lhs + rhs;
            }
        ";

        assert_rejected("point-addition", "cannot add", source);
    }

    #[test]
    fn point_profile_has_no_point_addition_assignment() {
        let source = "
            #[typekin::integral(konst = none, profile = point {})]
            struct Position(u8);

            fn forbidden() {
                let mut lhs = Position::make(1);
                let rhs = Position::make(2);
                lhs += rhs;
            }
        ";

        assert_rejected("point-addition-assignment", "AddAssign", source);
    }

    #[test]
    fn point_profile_has_no_primitive_subtraction() {
        let source = "
            #[typekin::integral(konst = none, profile = point {})]
            struct Position(u8);

            fn forbidden() {
                let position = Position::make(2);
                let primitive = 1u8;
                let _ = position - primitive;
            }
        ";

        assert_rejected("point-primitive-subtraction", "u8: Math", source);
    }

    #[test]
    fn point_profile_has_no_core_integer_helpers() {
        let source = "
            #[typekin::integral(konst = none, profile = point {})]
            struct Position(u8);

            fn forbidden() {
                let position = Position::make(1);
                let _ = position.count_ones();
            }
        ";

        assert_rejected(
            "point-core-integer-helpers",
            "no method named `count_ones`",
            source,
        );
    }

    #[test]
    fn circular_profile_without_precedes_has_no_precedes_method() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    profile = circular {
                        precedes = false,
                    },
                )]
                struct Sequence(u8);

                fn requires_precedes() {
                    let _ = Sequence::make(0).precedes(Sequence::make(1));
                }
            "#;

        assert_rejected(
            "circular-profile-without-precedes",
            "no method named `precedes`",
            SRC,
        );
    }

    #[test]
    fn circular_profile_has_no_linear_ordering() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    profile = circular {},
                )]
                struct Sequence(u8);

                fn requires_ord<T: Ord>() {}

                fn requires_linear_ordering() {
                    requires_ord::<Sequence>();
                }
            "#;

        assert_rejected(
            "circular-profile-has-no-linear-ordering",
            "Sequence: Ord",
            SRC,
        );
    }

    #[test]
    fn circular_profile_has_no_primitive_arithmetic() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    profile = circular {},
                )]
                struct Sequence(u8);

                fn requires_primitive_addition() {
                    let _ = Sequence::make(1) + 1u8;
                }
            "#;

        assert_rejected(
            "circular-profile-has-no-primitive-arithmetic",
            "the trait bound `u8: Math` is not satisfied",
            SRC,
        );
    }

    #[test]
    fn circular_profile_has_no_core_integer_helpers() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    profile = circular {},
                )]
                struct Sequence(u8);

                fn requires_count_ones() {
                    let _ = Sequence::make(1).count_ones();
                }
            "#;

        assert_rejected(
            "circular-profile-has-no-core-integer-helpers",
            "no method named `count_ones`",
            SRC,
        );
    }

    #[test]
    fn without_removes_from_str() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    without = from_str,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_from_str<T>()
                where
                    T: core::str::FromStr<Err = ()>,
                {}

                fn requires_number_from_str() {
                    requires_from_str::<Number>();
                }
            "#;
        assert_rejected(
            "without-removes-from-str",
            "the trait bound `Number: FromStr` is not satisfied",
            SRC,
        );
    }

    #[test]
    fn without_removes_common_integral_methods() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    without = core_int,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_count_ones() {
                    let _ = Number::make(2).count_ones();
                }
            "#;
        assert_rejected(
            "without-removes-common-integral-methods",
            "no method named `count_ones`",
            SRC,
        );
    }

    #[test]
    fn core_integral_methods_exclude_unsafe_operations() {
        const SRC: &str = r#"
                #[typekin::integral(konst = none)]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_unchecked_add() {
                    let _ = Number::make(2).unchecked_add(Number::make(3));
                }
            "#;
        assert_rejected(
            "core-integral-methods-exclude-unsafe-operations",
            "no method named `unchecked_add`",
            SRC,
        );
    }

    #[test]
    fn validated_text_has_no_infallible_from() {
        const SRC: &str = r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_infallible_from() {
                    let _: Checked = String::from("text").into();
                }
            "#;
        assert_rejected(
            "validated-text-has-no-infallible-from",
            "the trait bound `Checked: From<String>` is not satisfied",
            SRC,
        );
    }

    #[test]
    fn validated_text_has_no_deref_mut() {
        const SRC: &str = r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_deref_mut(value: &mut Checked) {
                    let _: &mut str = value;
                }
            "#;
        assert_rejected("validated-text-has-no-deref-mut", "DerefMut", SRC);
    }

    #[test]
    fn validated_text_has_no_infallible_str_from() {
        const SRC: &str = r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_infallible_from() {
                    let _: Checked = "text".into();
                }
            "#;
        assert_rejected(
            "validated-text-has-no-infallible-str-from",
            "the trait bound `Checked: From<&str>` is not satisfied",
            SRC,
        );
    }

    #[test]
    fn validated_text_has_no_as_mut() {
        const SRC: &str = r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_as_mut(value: &mut Checked) {
                    let _: &mut str = AsMut::<str>::as_mut(value);
                }
            "#;
        assert_rejected("validated-text-has-no-as-mut", "AsMut<str>", SRC);
    }

    #[test]
    fn validated_text_has_no_mutable_string() {
        const SRC: &str = r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_mutable_string(_: &mut String) {}

                fn mutable_string_escape(value: &mut Checked) {
                    requires_mutable_string(value);
                }
            "#;
        assert_rejected(
            "validated-text-has-no-mutable-string",
            "expected `&mut String`, found `&mut Checked`",
            SRC,
        );
    }
}
