//! `packed`: typekin's own diagnostics and token details a consumer cannot
//! observe.

#[cfg(test)]
mod expand {
    use crate::packed::{
        Cfg,
        PackedItem,
        ekran,
    };

    pub(crate) fn expansion(
        config: &str,
        source: &str,
    ) -> syn::Result<String> {
        let config = syn::parse_str::<Cfg>(config)?;
        let item = syn::parse_str::<PackedItem>(source)?;
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

    /// The text of the function named `name` up to the next
    /// `# [must_use]`, or to the end.
    pub(crate) fn body_of<'a>(
        expansion: &'a str,
        name: &str,
    ) -> &'a str {
        let start = format!("fn {name} (");
        let at = expansion
            .find(&start)
            .unwrap_or_else(|| panic!("no {start}: {expansion}"));
        let rest = &expansion[at..];
        let end = rest[start.len()..]
            .find("# [must_use]")
            .map_or(rest.len(), |it| it + start.len());
        return &rest[..end];
    }

    /// A record of every field shape: a wrapper, a bitflag value, `bool`,
    /// and a primitive.
    pub(crate) const RECORD: &str = "
        pub struct Header {
            #[bits(24..32)] pub id: PageId,
            #[bits(16..24)] pub state: StateValue,
            #[bit(9)] pub sealed: bool,
            #[bits(0..8)] pub data: u8,
        }
    ";
}

/// One test per rejected configuration key, flag, and `repr`.
#[cfg(test)]
mod configuration {
    use super::expand::{
        RECORD,
        error,
    };

    #[test]
    fn rejects_missing_repr() {
        let it = "konst = stable";
        let error = error(it, RECORD);
        assert_eq!(
            error, "`packed` requires `repr = u8 | u16 | u32 | u64 | u128`",
            "{it}"
        );
    }

    #[test]
    fn rejects_signed_repr() {
        let it = "repr = i32";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` needs an unsigned `repr`", "{it}");
    }

    #[test]
    fn rejects_usize_repr() {
        let it = "repr = usize";
        let error = error(it, RECORD);
        assert_eq!(
            error,
            "`packed` needs a fixed-width `repr`; the width of `usize` \
            depends on the target",
            "{it}",
        );
    }

    #[test]
    fn rejects_non_integer_repr() {
        let it = "repr = f32";
        let error = error(it, RECORD);
        assert_eq!(
            error, "`repr` takes `u8`, `u16`, `u32`, `u64`, or `u128`",
            "{it}"
        );
    }

    #[test]
    fn rejects_domain() {
        let it = "repr = u32, domain = 1..";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `domain`", "{it}");
    }

    #[test]
    fn rejects_valid() {
        let it = "repr = u32, valid = is_valid";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `valid`", "{it}");
    }

    #[test]
    fn rejects_friend() {
        let it = "repr = u32, friend = _(u8) -> Make";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `friend`", "{it}");
    }

    #[test]
    fn rejects_profile() {
        let it = "repr = u32, profile = wrapping";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `profile`", "{it}");
    }

    #[test]
    fn rejects_facet() {
        let it = "repr = u32, facet = point";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `facet`", "{it}");
    }

    #[test]
    fn rejects_produce() {
        let it = "repr = u32, produce = wrap(_) -> Other";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `produce`", "{it}");
    }

    #[test]
    fn rejects_tree() {
        let it = "repr = u32, tree = Leaf";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `tree`", "{it}");
    }

    #[test]
    fn rejects_math() {
        let it = "repr = u32, math = std";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `math`", "{it}");
    }

    #[test]
    fn rejects_panic_at_at_the_key() {
        let it = "repr = u32, panic_at = nothing";
        let this = syn::parse_str::<crate::packed::Cfg>(it);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "`packed` does not support `panic_at`");
        assert_eq!(error.span().start().line, 1, "{it}");
        assert_eq!(
            error.span().start().column,
            it.find("panic_at").unwrap(),
            "{it}",
        );
    }

    #[test]
    fn rejects_operator_flag_at_its_span() {
        let it = "repr = u32, with = [ord, math_add]";
        let this = syn::parse_str::<crate::packed::Cfg>(it);
        assert!(this.is_err(), "{it}");
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "`packed` does not support `math_add`");
        assert_eq!(
            error.span().start().column,
            it.find("math_add").unwrap(),
            "{it}",
        );
    }

    #[test]
    fn rejects_konst_flag() {
        let it = "repr = u32, with = konst_value";
        let error = error(it, RECORD);
        assert_eq!(error, "`packed` does not support `konst_value`", "{it}");
    }

    #[test]
    fn rejects_ord_without_eq() {
        let it = "repr = u32, with = ord, without = eq";
        let error = error(it, RECORD);
        assert_eq!(error, "`ord` needs `eq`", "{it}");
    }

    #[test]
    fn rejects_duplicate_key() {
        let it = "repr = u32, repr = u16";
        let error = error(it, RECORD);
        assert_eq!(error, "duplicated arg", "{it}");
    }

    #[test]
    fn rejects_non_zero_without_wrapper_field() {
        let it = "repr = u16, with = non_zero";
        let source = "
            struct Flags {
                #[bit(0)] on: bool,
                #[bits(8..16)] byte: u8,
            }
        ";
        let error = error(it, source);
        assert_eq!(
            error,
            "`non_zero` needs a field whose type can exclude zero; `bool` \
            and primitive fields admit zero",
            "{it}",
        );
    }
}

/// One test per item and field diagnostic.
#[cfg(test)]
mod item {
    use super::expand::{
        error,
        expanded,
    };

    const CONFIG: &str = "repr = u32";
    const SHAPE: &str = "`packed` needs a non-generic struct with named fields";
    const POSITION: &str = "each field needs one `#[bits(..)]` or `#[bit(..)]`";

    #[test]
    fn rejects_enum() {
        let source = "enum Header { A }";
        let error = error(CONFIG, source);
        assert_eq!(error, SHAPE, "{source}");
    }

    #[test]
    fn rejects_tuple_struct() {
        let source = "struct Header(u32);";
        let error = error(CONFIG, source);
        assert_eq!(error, SHAPE, "{source}");
    }

    #[test]
    fn rejects_unit_struct() {
        let source = "struct Header;";
        let error = error(CONFIG, source);
        assert_eq!(error, SHAPE, "{source}");
    }

    #[test]
    fn rejects_generics() {
        let source = "struct Header<T> { #[bit(0)] on: bool, tag: T }";
        let error = error(CONFIG, source);
        assert_eq!(error, SHAPE, "{source}");
    }

    #[test]
    fn rejects_where_clause() {
        let source = "struct Header where u8: Copy { #[bit(0)] on: bool }";
        let error = error(CONFIG, source);
        assert_eq!(error, SHAPE, "{source}");
    }

    #[test]
    fn rejects_empty_struct() {
        let source = "struct Header {}";
        let error = error(CONFIG, source);
        assert_eq!(error, "`packed` needs at least one field", "{source}");
    }

    #[test]
    fn rejects_missing_position() {
        let source = "struct Header { #[bit(0)] on: bool, off: bool }";
        let error = error(CONFIG, source);
        assert_eq!(error, POSITION, "{source}");
    }

    #[test]
    fn rejects_duplicate_position() {
        let source = "struct Header { #[bit(0)] #[bit(1)] on: bool }";
        let error = error(CONFIG, source);
        assert_eq!(error, POSITION, "{source}");
    }

    #[test]
    fn rejects_other_field_attribute() {
        let source = "struct Header { #[bit(0)] #[allow(unused)] on: bool }";
        let error = error(CONFIG, source);
        assert_eq!(
            error,
            "packed fields take only `#[bits(..)]`, `#[bit(..)]`, and doc \
            comments",
            "{source}",
        );
    }

    #[test]
    fn accepts_doc_comments() {
        let source = "
            struct Header {
                /// Whether it is on.
                #[bit(0)]
                on: bool,
            }
        ";
        let expansion = expanded(CONFIG, source);
        assert!(expansion.contains("\" Whether it is on.\""), "{expansion}");
    }

    #[test]
    fn rejects_extra_tokens_in_bits() {
        let source = "struct Header { #[bits(0..8, offset = 1)] byte: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(error, "`#[bits(..)]` takes only a bit range", "{source}");
    }

    #[test]
    fn rejects_bits_without_range() {
        let source = "struct Header { #[bits] byte: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(error, "`#[bits(..)]` takes only a bit range", "{source}");
    }

    #[test]
    fn rejects_open_range() {
        let source = "struct Header { #[bits(0..)] byte: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(error, "`#[bits(..)]` takes only a bit range", "{source}");
    }

    #[test]
    fn rejects_range_in_bit() {
        let source = "struct Header { #[bit(0..1)] on: bool }";
        let error = error(CONFIG, source);
        assert_eq!(error, "`#[bit(..)]` takes only a bit index", "{source}");
    }

    #[test]
    fn rejects_empty_range() {
        let source = "struct Header { #[bits(4..4)] byte: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(error, "bits 4..4 are empty", "{source}");
    }

    #[test]
    fn rejects_reversed_inclusive_range() {
        let source = "struct Header { #[bits(8..=3)] byte: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(error, "bits 8..=3 are empty", "{source}");
    }

    #[test]
    fn rejects_range_past_repr() {
        let source = "struct Header { #[bits(30..34)] small: Small }";
        let error = error(CONFIG, source);
        assert_eq!(error, "bits 30..34 exceed `u32` (32 bits)", "{source}");
    }

    #[test]
    fn rejects_bit_past_repr() {
        let source = "struct Header { #[bit(32)] on: bool }";
        let error = error(CONFIG, source);
        assert_eq!(error, "bit 32 exceeds `u32` (32 bits)", "{source}");
    }

    #[test]
    fn accepts_inclusive_range_to_the_last_bit() {
        let source = "struct Header { #[bits(24..=31)] byte: u8 }";
        let expansion = expanded(CONFIG, source);
        assert!(expansion.contains(">> 24"), "{expansion}");
    }

    #[test]
    fn rejects_overlap_at_the_later_field() {
        let source = "
            struct Header {
                #[bits(24..32)] id: PageId,
                #[bits(20..26)] state: State,
            }
        ";
        let error = error(CONFIG, source);
        assert_eq!(
            error, "bits 20..26 of `state` overlap bits 24..32 of `id`",
            "{source}"
        );
    }

    #[test]
    fn rejects_overlapping_bit() {
        let source = "
            struct Header {
                #[bits(0..8)] byte: u8,
                #[bit(7)] on: bool,
            }
        ";
        let error = error(CONFIG, source);
        assert_eq!(
            error, "bit 7 of `on` overlaps bits 0..8 of `byte`",
            "{source}"
        );
    }

    #[test]
    fn rejects_wide_bool() {
        let source = "struct Header { #[bits(0..2)] on: bool }";
        let error = error(CONFIG, source);
        assert_eq!(
            error, "`bool` takes exactly one bit; use `#[bit(N)]`",
            "{source}"
        );
    }

    #[test]
    fn rejects_narrow_primitive() {
        let source = "struct Header { #[bits(0..6)] byte: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(
            error,
            "`u8` needs exactly 8 bits; use a wrapper with a narrower \
            `domain`",
            "{source}",
        );
    }

    #[test]
    fn rejects_wide_signed_primitive() {
        let source = "struct Header { #[bits(0..12)] byte: i8 }";
        let error = error(CONFIG, source);
        assert_eq!(
            error,
            "`i8` needs exactly 8 bits; use a wrapper with a narrower \
            `domain`",
            "{source}",
        );
    }

    #[test]
    fn rejects_pointer_sized_field() {
        let source = "struct Header { #[bits(0..32)] size: usize }";
        let error = error(CONFIG, source);
        assert_eq!(
            error,
            "`usize` has a target-dependent width; use a fixed-width integer",
            "{source}",
        );
    }

    #[test]
    fn rejects_float_field() {
        let source = "struct Header { #[bits(0..32)] ratio: f32 }";
        let error = error(CONFIG, source);
        assert_eq!(error, "packed fields cannot be floats", "{source}");
    }

    #[test]
    fn rejects_text_field() {
        let source = "struct Header { #[bits(0..32)] name: String }";
        let error = error(CONFIG, source);
        assert_eq!(error, "packed fields cannot be text", "{source}");
    }

    #[test]
    fn rejects_unnamed_type_field() {
        let source = "struct Header { #[bits(0..16)] pair: (u8, u8) }";
        let error = error(CONFIG, source);
        assert_eq!(
            error,
            "packed fields are `bool`, fixed-width primitive integers, or \
            typekin integer types",
            "{source}",
        );
    }

    #[test]
    fn rejects_field_named_raw() {
        let source = "struct Header { #[bits(0..8)] raw: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(
            error, "field `raw` collides with the generated `raw()`",
            "{source}"
        );
    }

    #[test]
    fn rejects_field_named_try_make() {
        let source = "struct Header { #[bits(0..8)] try_make: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(
            error, "field `try_make` collides with the generated `try_make()`",
            "{source}"
        );
    }

    #[test]
    fn rejects_field_named_make() {
        let source = "struct Header { #[bits(0..8)] make: u8 }";
        let error = error(CONFIG, source);
        assert_eq!(
            error, "field `make` collides with the generated `make()`",
            "{source}"
        );
    }

    #[test]
    fn accepts_field_named_make_without_values_struct() {
        let it = "repr = u32, without = values_struct";
        let source = "struct Header { #[bits(0..8)] make: u8 }";
        let expansion = expanded(it, source);
        assert!(expansion.contains("fn make (self)"), "{expansion}");
    }

    #[test]
    fn rejects_field_named_like_a_setter() {
        let source = "
            struct Header {
                #[bits(0..8)] level: u8,
                #[bits(8..16)] with_level: u8,
            }
        ";
        let error = error(CONFIG, source);
        assert_eq!(
            error,
            "field `with_level` collides with the generated `with_level()`",
            "{source}",
        );
    }
}

/// Token details: what `make` and the setters do and do not do, and which
/// fields get proofs.
#[cfg(test)]
mod tokens {
    use super::expand::{
        RECORD,
        body_of,
        expanded,
    };

    #[test]
    fn emits_no_operator_impls() {
        let expansion = expanded("repr = u32", RECORD);
        assert!(!expansion.contains(":: core :: ops ::"), "{expansion}");
    }

    #[test]
    fn make_validates_only_in_debug_builds() {
        let expansion = expanded("repr = u32", RECORD);
        let make = body_of(&expansion, "make");
        assert_eq!(make.matches("__typekin_packed_valid").count(), 1, "{make}",);
        assert!(
            make.contains(
                ":: core :: debug_assert ! (__typekin_packed_valid (raw)"
            ),
            "{make}",
        );
    }

    #[test]
    fn setters_validate_only_in_debug_builds() {
        let expansion = expanded("repr = u32", RECORD);
        let setter = body_of(&expansion, "with_state");
        assert_eq!(
            setter.matches("__typekin_packed_valid").count(),
            1,
            "{setter}",
        );
        assert!(
            setter.contains(
                ":: core :: debug_assert ! (__typekin_packed_valid (raw)"
            ),
            "{setter}",
        );
    }

    #[test]
    fn non_zero_make_keeps_the_checked_storage_constructor() {
        let expansion = expanded("repr = u32, with = non_zero", RECORD);
        let make = body_of(&expansion, "make");
        assert!(!make.contains("unsafe"), "{make}");
        assert!(make.contains("NonZeroU32 :: new (raw)"), "{make}");
    }

    #[test]
    fn emits_no_unsafe_block_without_non_zero() {
        let expansion = expanded("repr = u32", RECORD);
        assert!(!expansion.contains("unsafe {"), "{expansion}");
    }

    #[test]
    fn proves_each_wrapper_field() {
        let expansion = expanded("repr = u32", RECORD);
        assert_eq!(
            expansion
                .matches("let (signed , bits) : (bool , u32) =")
                .count(),
            2,
            "{expansion}",
        );
        assert!(
            expansion.contains("< PageId > :: __TYPEKIN_WITHIN_V1_RAW ;"),
            "{expansion}",
        );
        assert!(
            expansion.contains("< StateValue > :: __TYPEKIN_WITHIN_V1_RAW ;"),
            "{expansion}",
        );
    }

    #[test]
    fn emits_no_proof_without_wrapper_fields() {
        let source = "
            struct Flags {
                #[bit(0)] on: bool,
                #[bits(8..16)] byte: i8,
            }
        ";
        let expansion = expanded("repr = u16", source);
        assert!(
            !expansion.contains("__typekin_within_subset"),
            "{expansion}"
        );
    }

    #[test]
    fn validator_checks_unclaimed_bits() {
        let expansion = expanded("repr = u32", RECORD);
        let valid = body_of(&expansion, "__typekin_packed_valid");
        assert!(valid.contains("raw & 0xfd00 != 0"), "{valid}");
    }

    #[test]
    fn validator_skips_the_gap_check_when_every_bit_is_claimed() {
        let source = "struct Full { #[bits(0..16)] word: u16 }";
        let expansion = expanded("repr = u16", source);
        let valid = body_of(&expansion, "__typekin_packed_valid");
        assert!(!valid.contains("!= 0"), "{valid}");
    }
}
