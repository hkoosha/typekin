use crate::*;
use std::env;
use std::error::Error;

//noinspection RsConstantConditionIf
fn assert_rejected(
    name: &str,
    expected_error: &str,
    source: &str,
) {
    if !env::var("TYPEKIN_SLOW_TESTS").is_ok() {
        return;
    }

    let output = compile(name, "", source, true);

    assert!(
        !output.status.success(),
        "unexpectedly compiled: name={} output={:?}"
        name, output,
    );
    assert!(
        output.stderr.contains(expected_error),
        "rejected for an unexpected reason; name={}, expected={}, output={:?}"
        name, expected_error, output
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    friends = into_u8(MakeSource) -> Make,
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
                    friends = into_u8(BitSource) -> Bit,
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
    fn self_operators_do_not_use_friendship() {
        const SRC: &str = r#"
                #[typekin::integral(
                    konst = none,
                    friends = Number(Number) -> Math,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);
            "#;
        assert_rejected(
            "self-operators-do-not-use-friendship",
            "self math operations use `with_self_math`, not `friends`",
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
