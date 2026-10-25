use crate::*;
use serde_test::{
    Configure,
    Token,
    assert_de_tokens_error,
    assert_tokens,
};
use std::alloc::GlobalAlloc;
use std::cell::Cell;
use std::rc::Rc;

//noinspection SpellCheckingInspection
const MY_WORD: &str = "safeé";

fn valid_owned_text(value: &str) -> bool {
    if value == "panic" {
        panic!("validator panic");
    }
    return value == "safe" || value == MY_WORD;
}

#[typekin::text(valid = valid_owned_text)]
struct OwnedText(String);

fn tracked_text(value: &str) -> (OwnedText, AllocationWatch) {
    let mut backing = String::with_capacity(64);
    backing.push_str(value);
    let text = OwnedText::try_make(backing).unwrap();
    assert!(text.capacity() >= 64);
    let watch = AllocationWatch::watch(text.as_str().as_ptr());
    return (text, watch);
}

struct EmptySetSource(String);
struct TrustEmptySetSource(String);

fn empty_set_text(source: EmptySetSource) -> String {
    source.0
}

fn trusted_empty_set_text(source: TrustEmptySetSource) -> String {
    source.0
}

#[typekin::text(
    in = [],
    friends = [
        empty_set_text(EmptySetSource) -> Make,
        trusted_empty_set_text(TrustEmptySetSource) -> [Make, unsafe],
    ],
)]
struct EmptySetText(String);

struct DropProbe(Rc<Cell<usize>>);

impl DropProbe {
    fn assert_live(&self) {
        assert_eq!(self.0.get(), 0);
    }
}

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

// =============================================================================

fn is_slug(value: &str) -> bool {
    return !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-');
}

fn is_not_reserved(value: &str) -> bool {
    return value != "reserved";
}

struct SlugSource(String);
struct TrustSlugSource(String);

impl SlugSource {
    fn into_string(self) -> String {
        return self.0;
    }
}

impl TrustSlugSource {
    fn into_string(self) -> String {
        return self.0;
    }
}

#[typekin::text(
    in = ["draft", "published"],
    friends = [
        SlugSource::into_string(SlugSource) -> Make,
        TrustSlugSource::into_string(TrustSlugSource) -> unsafe,
    ],
)]
struct Slug(String);

#[typekin::text]
struct FullyGeneratedText(String);

#[typekin::text(without = [clone, repr])]
#[repr(transparent)]
#[derive(Clone)]
struct ManuallyDerivedText(String);

#[typekin::text(
    friends = SlugSource::into_string(SlugSource) -> Make,
    valid = is_slug,
)]
struct CallbackText(String);

#[typekin::text(valid = [is_slug, is_not_reserved])]
struct CallbackChainText(String);

#[typekin::text(in = ["", "é"])]
struct LiteralText(String);

#[typekin::text(friends = self(Self) -> [Rel])]
struct DisplayText(String);

#[typekin::text(without = display)]
struct NoDisplayText(String);

impl core::fmt::Display for NoDisplayText {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        return formatter.write_str(self.as_str());
    }
}

thread_local! {
    static MUTATION_CALLS: Cell<usize> = const { Cell::new(0) };
}

fn reset() {
    MUTATION_CALLS.set(0);
}
fn inc() {
    MUTATION_CALLS.update(|it| it + 1);
}
fn require(it: usize) {
    MUTATION_CALLS.with(|v| assert_eq!(v.get(), it));
}

fn validate_mutation(value: &str) -> bool {
    inc();
    return !value.is_empty() && value != "b" && !value.contains('!');
}

#[typekin::text(valid = validate_mutation)]
struct CheckedMutationText(String);

fn validate_empty_mutation(value: &str) -> bool {
    inc();
    return value.is_empty();
}

#[typekin::text(valid = validate_empty_mutation)]
struct CheckedEmptyText(String);

#[typekin::text(in = ["a", "ab", "abc"])]
struct MemberMutationText(String);

#[typekin::text(valid = is_not_reserved)]
struct MutationAllocationText(String);

fn is_serde_text_valid(value: &str) -> bool {
    return value == "valid";
}

#[typekin::text(
    valid = is_serde_text_valid,
    with = serde,
)]
struct SerdeText(String);

#[typekin::text(
    valid = is_slug,
    friends = Self::into_inner(Self) -> [Rel, unsafe],
)]
struct RelatedText(String);

struct CheckedSource(String);
struct TrustSource(String);

fn checked_text(source: CheckedSource) -> String {
    source.0
}

fn trusted_text(source: TrustSource) -> String {
    source.0
}

#[typekin::text(
    in = [],
    friends = [
        checked_text(CheckedSource) -> Make,
        trusted_text(TrustSource) -> [unsafe],
    ],
)]
struct EmptyDomain(String);

#[typekin::text(in = [])]
struct CallbackEmptyDomain(String);

#[typekin::text(in = [""])]
struct EmptyStringOnly(String);

#[cfg(test)]
#[cfg(feature = "fast_tests")]
mod tests {
    use super::*;

    #[test]
    fn text_serde_uses_owned_strings_and_revalidates() {
        assert_tokens(
            &SerdeText::try_from_str("valid").unwrap().readable(),
            &[Token::Str("valid")],
        );
        assert_tokens(
            &SerdeText::try_from_str("valid").unwrap().compact(),
            &[Token::Str("valid")],
        );
        assert_de_tokens_error::<SerdeText>(
            &[Token::Str("invalid")],
            "invalid value",
        );
    }

    #[test]
    fn full_text_generates_layout_and_clone() {
        accepts_clone::<FullyGeneratedText>();
        accepts_debug::<FullyGeneratedText>();
        accepts_eq::<FullyGeneratedText>();
        accepts_hash::<FullyGeneratedText>();
        accepts_ord::<FullyGeneratedText>();

        let text = FullyGeneratedText::try_from_str("full").unwrap().clone();
        assert_eq!(text.as_str(), "full");
        assert_eq!(size_of::<FullyGeneratedText>(), size_of::<String>());
        assert_eq!(align_of::<FullyGeneratedText>(), align_of::<String>());
    }

    #[test]
    fn formats_debug_with_the_type_name() {
        let text = FullyGeneratedText::try_from_str("full").unwrap();
        assert_eq!(format!("{text:?}"), r#"FullyGeneratedText("full")"#,);
    }

    #[test]
    fn text_layout_and_clone_can_be_supplied_manually() {
        accepts_clone::<ManuallyDerivedText>();
        let text = ManuallyDerivedText::try_from_str("manual").unwrap().clone();
        assert_eq!(text.as_str(), "manual");
    }

    #[test]
    fn accepts_draft_and_rejects_case_variant() {
        let value = "draft";
        assert_eq!(
            Slug::try_make(value.into()).map(Slug::into_inner),
            Ok(value.into())
        );
        assert_eq!(
            Slug::try_from_str(value).map(Slug::into_inner),
            Ok(value.into())
        );

        let value = "Draft";
        assert_eq!(Slug::try_make(value.into()), Err(()));
        assert_eq!(Slug::try_from_str(value), Err(()));
    }

    #[test]
    fn accepts_published_and_rejects_case_variant() {
        let value = "published";
        assert_eq!(
            Slug::try_make(value.into()).map(Slug::into_inner),
            Ok(value.into())
        );
        assert_eq!(
            Slug::try_from_str(value).map(Slug::into_inner),
            Ok(value.into())
        );

        let value = "Draft";
        assert_eq!(Slug::try_make(value.into()), Err(()));
        assert_eq!(Slug::try_from_str(value), Err(()));
    }

    #[test]
    fn callback_validation_works_without_literal_membership() {
        assert_eq!(
            CallbackText::try_from_str("reserved")
                .map(CallbackText::into_inner),
            Ok("reserved".into())
        );
        assert_eq!(CallbackChainText::try_from_str("reserved"), Err(()));
        assert_eq!(CallbackText::try_from_str("Draft"), Err(()));
        assert_eq!(CallbackChainText::try_from_str("Draft"), Err(()));
        assert_eq!(
            CallbackChainText::try_from_str("other")
                .map(CallbackChainText::into_inner),
            Ok("other".into())
        );
    }

    #[test]
    fn empty_literal_membership_rejects_decomposed_accent() {
        let value = "";
        assert_eq!(
            LiteralText::try_from_str(value).map(LiteralText::into_inner),
            Ok(value.into())
        );

        let value = "e\u{301}";
        assert_eq!(LiteralText::try_from_str(value), Err(()));
    }

    #[test]
    fn accented_literal_membership_rejects_decomposed_accent() {
        let value = "é";
        assert_eq!(
            LiteralText::try_from_str(value).map(LiteralText::into_inner),
            Ok(value.into())
        );

        let value = "e\u{301}";
        assert_eq!(LiteralText::try_from_str(value), Err(()));
    }

    #[test]
    fn map_rejects_case_variant() {
        let published = Slug::try_from_str("draft")
            .unwrap()
            .map(|value| value.replace_range(.., "published"));
        assert_eq!(published.map(Slug::into_inner), Ok("published".into()));

        let invalid = "Draft";
        let result = Slug::try_from_str("draft").unwrap().map(|value| {
            value.clear();
            value.push_str(invalid);
        });
        assert_eq!(result, Err(()));
    }

    #[test]
    fn exposes_read_only_string_surface() {
        let draft = Slug::try_from_str("draft").unwrap();
        assert_eq!(draft.as_str(), "draft");
        assert_eq!(draft.as_bytes(), b"draft");
        assert_eq!(draft.len(), 5);
        assert!(!draft.is_empty());
        assert!(draft.capacity() >= draft.len());
        assert_eq!(<&str>::from(&*draft), "draft");
        assert_eq!(draft.to_string(), "draft");
        assert_eq!(draft.into_bytes(), b"draft");
    }

    #[test]
    fn hash_map_borrowed_str_finds_draft() {
        let hashed = std::collections::HashMap::from([
            (Slug::try_from_str("published").unwrap(), 2),
            (Slug::try_from_str("draft").unwrap(), 1),
        ]);

        let (key, expected) = ("draft", Some(&1));
        assert_eq!(hashed.get(key), expected);
    }

    #[test]
    fn hash_map_borrowed_str_rejects_other() {
        let hashed = std::collections::HashMap::from([
            (Slug::try_from_str("published").unwrap(), 2),
            (Slug::try_from_str("draft").unwrap(), 1),
        ]);

        let (key, expected) = ("other", None);
        assert_eq!(hashed.get(key), expected);
    }

    #[test]
    fn btree_map_borrowed_str_finds_draft() {
        let ordered = std::collections::BTreeMap::from([
            (Slug::try_from_str("published").unwrap(), 2),
            (Slug::try_from_str("draft").unwrap(), 1),
        ]);

        let (key, expected) = ("draft", Some(&1));
        assert_eq!(ordered.get(key), expected);

        assert_eq!(
            ordered.keys().map(Slug::as_str).collect::<Vec<_>>(),
            ["draft", "published"]
        );
    }

    #[test]
    fn btree_map_borrowed_str_rejects_other() {
        let ordered = std::collections::BTreeMap::from([
            (Slug::try_from_str("published").unwrap(), 2),
            (Slug::try_from_str("draft").unwrap(), 1),
        ]);

        let (key, expected) = ("other", None);
        assert_eq!(ordered.get(key), expected);

        assert_eq!(
            ordered.keys().map(Slug::as_str).collect::<Vec<_>>(),
            ["draft", "published"]
        );
    }

    #[test]
    fn read_only_unicode_views_are_usable() {
        let value = DisplayText::from("é🦀Z");
        let text = <DisplayText as AsRef<str>>::as_ref(&value);
        let bytes = <DisplayText as AsRef<[u8]>>::as_ref(&value);

        assert_eq!(text, "é🦀Z");
        assert_eq!(bytes, "é🦀Z".as_bytes());
        assert_eq!(text.as_ptr(), value.as_str().as_ptr());
        assert_eq!(bytes.as_ptr(), value.as_bytes().as_ptr());
        assert_eq!(
            value.char_indices().collect::<Vec<_>>(),
            [(0, 'é'), (2, '🦀'), (6, 'Z')]
        );
        assert_eq!(value.get(2..6), Some("🦀"));
        assert_eq!(value.get(1..2), None);
    }

    #[test]
    fn text_orders_same_type_values() {
        let value = DisplayText::from("é🦀Z");
        let earlier = DisplayText::from("é🦀A");
        assert_eq!(value, DisplayText::from("é🦀Z"));
        assert!(value > earlier);
        assert_eq!(value.cmp(&earlier), core::cmp::Ordering::Greater);
        assert_eq!(
            value.partial_cmp(&earlier),
            Some(core::cmp::Ordering::Greater)
        );
    }

    #[test]
    fn display_preserves_unicode_precision_alignment_and_fill() {
        let value = DisplayText::from("é🦀Z");

        assert_eq!(format!("{value:.2}"), "é🦀");
        assert_eq!(format!("{value:>6.2}"), "    é🦀");
        assert_eq!(format!("{value:*^7.2}"), "**é🦀***");
        assert_eq!(format!("{value:<5}"), "é🦀Z  ");
    }

    #[test]
    fn display_reports_exhausted_output_capacity() {
        use core::fmt::{
            self,
            Write,
        };

        struct FixedWriter {
            bytes: [u8; 8],
            len: usize,
        }

        impl Write for FixedWriter {
            fn write_str(
                &mut self,
                value: &str,
            ) -> fmt::Result {
                if value.len() > self.bytes.len() - self.len {
                    return Err(fmt::Error);
                }
                let end = self.len + value.len();
                self.bytes[self.len..end].copy_from_slice(value.as_bytes());
                self.len = end;
                return Ok(());
            }
        }

        let value = DisplayText::from("é🦀Z");
        let mut writer = FixedWriter {
            bytes: [0; 8],
            len: 0,
        };
        write!(&mut writer, "{value}").unwrap();
        assert_eq!(&writer.bytes[..writer.len], value.as_bytes());

        let mut writer = FixedWriter {
            bytes: [0; 8],
            len: 0,
        };
        assert_eq!(write!(&mut writer, "{value:>9}"), Err(fmt::Error));
    }

    #[test]
    fn friendship_rejects_untrusted_draft_source() {
        let valid = Slug::of(SlugSource("draft".into()));
        assert_eq!(valid.as_str(), "draft");

        let invalid = "Draft";
        assert_panics(|| {
            let _ = Slug::of(SlugSource(invalid.into()));
        });

        let trusted = Slug::of(TrustSlugSource(invalid.into()));
        assert_eq!(trusted.as_str(), invalid);
    }

    #[test]
    fn owned_friend_accepts_draft_without_reallocation() {
        let mut raw = String::with_capacity(64);
        raw.push_str("draft");
        let pointer = raw.as_ptr();
        let capacity = raw.capacity();
        let text = CallbackText::of(SlugSource(raw));
        assert_eq!(text.as_str(), "draft");
        assert_eq!(text.as_str().as_ptr(), pointer);
        assert_eq!(text.into_inner().capacity(), capacity);

        let invalid = "Draft";
        assert_panics(|| {
            let _ = CallbackText::of(SlugSource(String::from(invalid)));
        });
    }

    #[test]
    fn unvalidated_text_has_infallible_conversions() {
        let from_string: FullyGeneratedText = String::from("any value").into();
        let from_str: FullyGeneratedText = "another value".into();

        assert_eq!(from_string.into_inner(), "any value");
        assert_eq!(String::from(from_str), "another value");
    }

    #[test]
    fn transparent_text_preserves_string_layout_and_owned_allocation() {
        assert_eq!(size_of::<Slug>(), size_of::<String>());
        assert_eq!(align_of::<Slug>(), align_of::<String>());

        let mut raw = String::with_capacity(64);
        raw.push_str("draft");
        let pointer = raw.as_ptr();
        let capacity = raw.capacity();

        let value = Slug::try_make(raw).unwrap();
        assert_eq!(value.as_str().as_ptr(), pointer);
        assert_eq!(value.capacity(), capacity);

        let raw = String::from(value);
        assert_eq!(raw.as_ptr(), pointer);
        assert_eq!(raw.capacity(), capacity);
    }

    #[test]
    fn checked_mutations_transform_unicode_and_return_removed_characters() {
        let text = FullyGeneratedText::from("a").try_push('é').unwrap();
        assert_eq!(text.as_str(), "aé");
        let text = text.try_push_str("🍆z").unwrap();
        assert_eq!(text.as_str(), "aé🍆z");
        let text = text.try_insert(1, 'ø').unwrap();
        assert_eq!(text.as_str(), "aøé🍆z");
        let text = text.try_insert_str(3, "界").unwrap();
        assert_eq!(text.as_str(), "aø界é🍆z");

        let (text, removed) = text.try_remove(3).unwrap();
        assert_eq!(removed, '界');
        assert_eq!(text.as_str(), "aøé🍆z");
        let (text, removed) = text.try_pop().unwrap();
        assert_eq!(removed, Some('z'));
        let (text, removed) = text.try_pop().unwrap();
        assert_eq!(removed, Some('🍆'));
        assert_eq!(text.as_str(), "aøé");
        let text = text.try_truncate(3).unwrap();
        assert_eq!(text.as_str(), "aø");
        let text = text.try_clear().unwrap();
        assert_eq!(text.as_str(), "");
        let (text, removed) = text.try_pop().unwrap();
        assert_eq!(removed, None);
        assert_eq!(text.as_str(), "");

        let (text, removed) =
            FullyGeneratedText::from("é").try_remove(0).unwrap();
        assert_eq!(removed, 'é');
        assert_eq!(text.as_str(), "");
    }

    #[test]
    fn checked_replacement_supports_byte_range_variants() {
        fn text() -> FullyGeneratedText {
            return FullyGeneratedText::from("aé🍆z");
        }

        assert_eq!(
            text().try_replace_range(1..3, "ø").unwrap().as_str(),
            "aø🍆z"
        );
        assert_eq!(
            text().try_replace_range(1..=2, "ø").unwrap().as_str(),
            "aø🍆z"
        );
        assert_eq!(
            text().try_replace_range(..3, "ø").unwrap().as_str(),
            "ø🍆z"
        );
        assert_eq!(
            text().try_replace_range(..=2, "ø").unwrap().as_str(),
            "ø🍆z"
        );
        assert_eq!(text().try_replace_range(3.., "ø").unwrap().as_str(), "aéø");
        assert_eq!(text().try_replace_range(.., "ø").unwrap().as_str(), "ø");
        assert_eq!(
            text().try_replace_range(3..3, "ø").unwrap().as_str(),
            "aéø🍆z"
        );
        assert_eq!(
            text()
                .try_replace_range(
                    (
                        core::ops::Bound::Excluded(0),
                        core::ops::Bound::Included(2)
                    ),
                    "ø",
                )
                .unwrap()
                .as_str(),
            "aø🍆z"
        );
    }

    #[test]
    fn checked_retain_accepts_move_only_state_and_visits_characters() {
        struct RetainState {
            index: usize,
        }

        impl RetainState {
            fn keep(
                &mut self,
                character: char,
                visited: &mut Vec<char>,
            ) -> bool {
                visited.push(character);
                let keep = self.index % 2 == 0;
                self.index += 1;
                return keep;
            }
        }

        let mut state = RetainState { index: 0 };
        let mut visited = Vec::new();
        let visited_ref = &mut visited;
        let text = FullyGeneratedText::from("aé🍆界")
            .try_retain(move |character| state.keep(character, visited_ref))
            .unwrap();
        assert_eq!(text.as_str(), "a🍆");
        assert_eq!(visited, ['a', 'é', '🍆', '界']);
    }

    #[test]
    fn checked_mutations_validate_once_and_reject_callback_violations() {
        type Mutation =
            fn(CheckedMutationText) -> Result<CheckedMutationText, ()>;
        let accepting: [Mutation; 9] = [
            |text| text.try_push('d'),
            |text| text.try_push_str("de"),
            |text| text.try_insert(0, 'd'),
            |text| text.try_insert_str(0, "de"),
            |text| text.try_replace_range(.., "é"),
            |text| text.try_truncate(2),
            |text| text.try_retain(|character| character != 'b'),
            |text| text.try_remove(0).map(|(text, _)| text),
            |text| text.try_pop().map(|(text, _)| text),
        ];
        for mutate in accepting {
            let text = CheckedMutationText::try_from_str("abc").unwrap();
            reset();
            assert!(mutate(text).is_ok());
            require(1);
        }

        let rejecting: [Mutation; 10] = [
            |text| text.try_push('!'),
            |text| text.try_push_str("!"),
            |text| text.try_insert(0, '!'),
            |text| text.try_insert_str(0, "!"),
            |text| text.try_replace_range(.., "!"),
            |text| text.try_truncate(0),
            |text| text.try_clear(),
            |text| text.try_retain(|_| false),
            |text| text.try_remove(0).map(|(text, _)| text),
            |text| text.try_pop().map(|(text, _)| text),
        ];
        for (index, mutate) in rejecting.into_iter().enumerate() {
            let initial = match index {
                8 => "ab",
                9 => "ba",
                _ => "abc",
            };
            let text = CheckedMutationText::try_from_str(initial).unwrap();
            reset();
            assert_eq!(mutate(text), Err(()));
            require(1);
        }

        let text = CheckedMutationText::try_from_str("abc").unwrap();
        reset();
        assert_eq!((text + "d").as_str(), "abcd");
        require(1);

        let text = CheckedMutationText::try_from_str("abc").unwrap();
        reset();

        assert_panics(|| text + "!");

        require(1);
    }

    #[test]
    fn checked_mutations_reject_membership_violations_without_callbacks() {
        type Mutation =
            fn(MemberMutationText) -> Result<MemberMutationText, ()>;
        let mutations: [Mutation; 10] = [
            |text| text.try_push('x'),
            |text| text.try_push_str("xy"),
            |text| text.try_insert(0, 'x'),
            |text| text.try_insert_str(0, "xy"),
            |text| text.try_replace_range(.., "x"),
            |text| text.try_truncate(0),
            |text| text.try_clear(),
            |text| text.try_retain(|character| character != 'a'),
            |text| text.try_remove(0).map(|(text, _)| text),
            |text| text.try_pop().map(|(text, _)| text),
        ];
        for mutate in mutations {
            let text = MemberMutationText::try_from_str("a").unwrap();
            assert_eq!(mutate(text), Err(()));
        }

        assert_panics(|| {
            let _ = MemberMutationText::try_from_str("a").unwrap() + "x";
        });

        let text = MemberMutationText::try_from_str("a").unwrap() + "bc";
        assert_eq!(text.as_str(), "abc");
    }

    #[test]
    fn checked_mutations_preserve_string_index_panics_before_validation() {
        let invalid: [fn(CheckedMutationText); 12] = [
            |text| {
                let _ = text.try_insert(1, 'x');
            },
            |text| {
                let _ = text.try_insert(3, 'x');
            },
            |text| {
                let _ = text.try_insert_str(1, "x");
            },
            |text| {
                let _ = text.try_insert_str(3, "x");
            },
            |text| {
                let _ = text.try_remove(1);
            },
            |text| {
                let _ = text.try_remove(2);
            },
            |text| {
                let _ = text.try_replace_range(1..2, "x");
            },
            |text| {
                let _ = text.try_replace_range(0..1, "x");
            },
            |text| {
                let _ = text.try_replace_range(0..3, "x");
            },
            |text| {
                let _ = text.try_replace_range(3.., "x");
            },
            |text| {
                let start = text.len();
                let _ = text.try_replace_range(start..0, "x");
            },
            |text| {
                let _ = text.try_truncate(1);
            },
        ];
        for mutate in invalid {
            let text = CheckedMutationText::try_from_str("é").unwrap();
            reset();
            assert_panics(|| mutate(text));
            require(0);
        }

        assert_panics(|| {
            let _ = FullyGeneratedText::from("").try_remove(0);
        });
    }

    #[test]
    fn checked_truncate_beyond_length_keeps_text_and_still_validates() {
        let text = CheckedMutationText::try_from_str("é").unwrap();
        reset();
        let text = text.try_truncate(usize::MAX).unwrap();
        assert_eq!(text.as_str(), "é");
        require(1);

        let text = CheckedMutationText::try_from_str("é").unwrap();
        reset();
        let text = text.try_truncate(2).unwrap();
        assert_eq!(text.as_str(), "é");
        require(1);
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn checked_mutations_preserve_the_owned_allocation() {
        fn check(
            text: &MutationAllocationText,
            pointer: *const u8,
            capacity: usize,
            expected: &str,
        ) {
            assert_eq!(text.as_str(), expected);
            assert_eq!(text.as_str().as_ptr(), pointer);
            assert_eq!(text.capacity(), capacity);
        }

        let mut raw = String::with_capacity(64);
        raw.push_str("aé");
        let pointer = raw.as_ptr();
        let capacity = raw.capacity();
        let text = MutationAllocationText::try_make(raw).unwrap();
        let text = text.try_push('🍆').unwrap();
        check(&text, pointer, capacity, "aé🍆");
        let text = text.try_push_str("z").unwrap();
        check(&text, pointer, capacity, "aé🍆z");
        let text = text.try_insert(1, 'x').unwrap();
        check(&text, pointer, capacity, "axé🍆z");
        let text = text.try_insert_str(2, "y").unwrap();
        check(&text, pointer, capacity, "axyé🍆z");
        let text = text.try_replace_range(1..3, "b").unwrap();
        check(&text, pointer, capacity, "abé🍆z");
        let text = text.try_truncate(4).unwrap();
        check(&text, pointer, capacity, "abé");
        let text = text.try_retain(|character| character != 'b').unwrap();
        check(&text, pointer, capacity, "aé");
        let (text, removed) = text.try_remove(1).unwrap();
        assert_eq!(removed, 'é');
        check(&text, pointer, capacity, "a");
        let (text, removed) = text.try_pop().unwrap();
        assert_eq!(removed, Some('a'));
        check(&text, pointer, capacity, "");
        let text = text.try_push_str("abc").unwrap();
        let text = text.try_clear().unwrap();
        check(&text, pointer, capacity, "");
    }

    #[test]
    fn text_add_preserves_the_owned_allocation() {
        let mut raw = String::with_capacity(64);
        raw.push_str("aé");
        let pointer = raw.as_ptr();
        let capacity = raw.capacity();

        let text = MutationAllocationText::try_make(raw).unwrap() + "🍆";
        assert_eq!(text.as_str(), "aé🍆");
        assert_eq!(text.as_str().as_ptr(), pointer);
        assert_eq!(text.capacity(), capacity);
    }

    #[test]
    fn checked_empty_pop_still_validates_once() {
        let text = CheckedEmptyText::try_from_str("").unwrap();
        reset();
        let (text, removed) = text.try_pop().unwrap();
        assert_eq!(removed, None);
        assert_eq!(text.as_str(), "");
        require(1);
    }

    #[test]
    fn checked_empty_clear_still_validates_once() {
        let text = CheckedEmptyText::try_from_str("").unwrap();
        reset();
        let text = text.try_clear().unwrap();
        assert_eq!(text.as_str(), "");
        require(1);
    }

    #[test]
    fn trusted_self_relation_preserves_checked_validation() {
        let earlier = RelatedText::try_from_str("draft").unwrap();
        let later = RelatedText::try_from_str("published").unwrap();
        assert!(earlier < later);
        assert_eq!(earlier.cmp(&later), core::cmp::Ordering::Less);
        assert_eq!(earlier, RelatedText::try_from_str("draft").unwrap());
        for invalid in ["", "Draft", "é"] {
            let checked: Result<RelatedText, ()> =
                RelatedText::try_make(String::from(invalid));
            assert_eq!(checked, Err(()));
            assert_eq!(RelatedText::try_from_str(invalid), Err(()));
        }
    }

    #[test]
    fn empty_membership_rejects_every_checked_input_with_and_without_callbacks()
    {
        for value in ["", "draft", "reserved", "é🦀", "\0"] {
            let owned: Result<EmptyDomain, ()> =
                EmptyDomain::try_make(String::from(value));
            let borrowed: Result<EmptyDomain, ()> =
                EmptyDomain::try_from_str(value);
            assert_eq!(owned, Err(()));
            assert_eq!(borrowed, Err(()));

            let owned: Result<CallbackEmptyDomain, ()> =
                CallbackEmptyDomain::try_make(String::from(value));
            let borrowed: Result<CallbackEmptyDomain, ()> =
                CallbackEmptyDomain::try_from_str(value);
            assert_eq!(owned, Err(()));
            assert_eq!(borrowed, Err(()));
        }
    }

    #[test]
    fn empty_literal_rejects_non_empty_value() {
        assert_eq!(
            EmptyStringOnly::try_make(String::new())
                .map(EmptyStringOnly::into_inner),
            Ok(String::new()),
        );
        assert_eq!(
            EmptyStringOnly::try_from_str("").map(EmptyStringOnly::into_inner),
            Ok(String::new()),
        );
        let value = "é";
        assert_eq!(EmptyStringOnly::try_make(value.into()), Err(()));
        assert_eq!(EmptyStringOnly::try_from_str(value), Err(()));
    }

    #[test]
    fn empty_domain_rejects_other_untrusted_source() {
        let value = "é🦀";
        let checked = CheckedSource(String::from(value));
        assert_panics(|| EmptyDomain::of(checked));
        let trusted = TrustSource(String::from(value));
        assert_eq!(EmptyDomain::of(trusted).into_inner(), value);
    }

    #[test]
    fn trusted_empty_domain_values_revalidate_maps_and_all_checked_mutators() {
        type Mutation = fn(EmptyDomain) -> bool;
        let mutations: [(&str, Mutation); 11] = [
            ("map no-op", |text| text.map(|_| {}).is_err()),
            ("push", |text| text.try_push('a').is_err()),
            ("push_str no-op", |text| text.try_push_str("").is_err()),
            ("insert", |text| text.try_insert(0, 'a').is_err()),
            ("insert_str no-op", |text| {
                text.try_insert_str(0, "").is_err()
            }),
            ("replace_range no-op", |text| {
                text.try_replace_range(0..0, "").is_err()
            }),
            ("truncate no-op", |text| {
                text.try_truncate(usize::MAX).is_err()
            }),
            ("clear", |text| text.try_clear().is_err()),
            ("retain no-op", |text| text.try_retain(|_| true).is_err()),
            ("remove", |text| text.try_remove(0).is_err()),
            ("pop", |text| text.try_pop().is_err()),
        ];
        for (name, mutate) in mutations {
            let text = EmptyDomain::of(TrustSource(String::from("é🦀")));
            assert!(mutate(text), "{name} must reject an empty-domain value");
        }

        let text = EmptyDomain::of(TrustSource(String::from("é🦀")));
        assert_panics(|| text + "");
    }

    #[test]
    fn traits_and_layout_can_be_supplied_by_the_caller() {
        accepts_clone::<ManuallyDerivedText>();
        assert_eq!(
            ManuallyDerivedText::try_from_str("text").unwrap().as_str(),
            "text"
        );
    }
}

#[cfg(test)]
#[cfg(feature = "fast_tests")]
mod test_mutation_drop {
    use super::*;
    use std::cell::Cell;
    use std::panic::AssertUnwindSafe;
    use std::rc::Rc;

    #[test]
    fn empty_membership_releases_rejected_empty_inputs() {
        type Construction = fn(String) -> bool;
        let constructors: [(&str, Construction); 3] = [
            ("checked", |raw| EmptySetText::try_make(raw).is_err()),
            ("membership", |raw| {
                CallbackEmptyDomain::try_make(raw).is_err()
            }),
            ("untrusted friend", |raw| {
                assert_panics(|| EmptySetText::of(EmptySetSource(raw)));
                true
            }),
        ];
        for (name, construct) in constructors {
            let value = "";
            let mut raw = String::with_capacity(64);
            raw.push_str(value);
            let watch = AllocationWatch::watch(raw.as_ptr());
            assert!(construct(raw), "{name} must reject {value:?}");
            watch.assert_released();
        }
    }

    #[test]
    fn empty_membership_releases_rejected_ascii_inputs() {
        type Construction = fn(String) -> bool;
        let constructors: [(&str, Construction); 3] = [
            ("checked", |raw| EmptySetText::try_make(raw).is_err()),
            ("membership", |raw| {
                CallbackEmptyDomain::try_make(raw).is_err()
            }),
            ("untrusted friend", |raw| {
                assert_panics(|| EmptySetText::of(EmptySetSource(raw)));
                true
            }),
        ];
        for (name, construct) in constructors {
            let value = "safe";
            let mut raw = String::with_capacity(64);
            raw.push_str(value);
            let watch = AllocationWatch::watch(raw.as_ptr());
            assert!(construct(raw), "{name} must reject {value:?}");
            watch.assert_released();
        }
    }

    #[test]
    fn empty_membership_releases_rejected_unicode_inputs() {
        type Construction = fn(String) -> bool;
        let constructors: [(&str, Construction); 3] = [
            ("checked", |raw| EmptySetText::try_make(raw).is_err()),
            ("membership", |raw| {
                CallbackEmptyDomain::try_make(raw).is_err()
            }),
            ("untrusted friend", |raw| {
                assert_panics(|| EmptySetText::of(EmptySetSource(raw)));
                true
            }),
        ];
        for (name, construct) in constructors {
            let value = "é🦀";
            let mut raw = String::with_capacity(64);
            raw.push_str(value);
            let watch = AllocationWatch::watch(raw.as_ptr());
            assert!(construct(raw), "{name} must reject {value:?}");
            watch.assert_released();
        }
    }

    #[test]
    fn empty_membership_releases_trusted_buffers_after_rejected_noop_changes() {
        type Mutation = fn(EmptySetText) -> bool;
        let mutations: [Mutation; 2] = [
            |text| text.map(|_| {}).is_err(),
            |text| text.try_push_str("").is_err(),
        ];
        for mutate in mutations {
            let mut raw = String::with_capacity(64);
            raw.push_str("é🦀");
            let watch = AllocationWatch::watch(raw.as_ptr());
            let text = EmptySetText::of(TrustEmptySetSource(raw));
            assert_eq!(watch.deallocations(), 0);
            assert!(mutate(text));
            watch.assert_released();
        }
    }

    #[test]
    fn successful_mutation_keeps_the_allocation_until_the_owner_is_dropped() {
        let (text, watch) = tracked_text("safe");
        let original_address = text.as_str().as_ptr() as usize;
        let text = text.try_push('é').unwrap();
        assert_eq!(text.as_str(), MY_WORD);
        assert_eq!(text.as_str().as_ptr() as usize, original_address);
        assert_eq!(watch.deallocations(), 0);
        drop(text);
        watch.assert_released();
    }

    #[test]
    fn every_rejected_mutation_releases_the_owned_backing_allocation() {
        type Mutation = fn(OwnedText) -> bool;
        let mutations: [(&str, Mutation); 11] = [
            ("push", |text| text.try_push('!').is_err()),
            ("push_str", |text| text.try_push_str("!").is_err()),
            ("insert", |text| text.try_insert(1, '!').is_err()),
            ("insert_str", |text| text.try_insert_str(1, "!").is_err()),
            ("replace_range", |text| {
                text.try_replace_range(0..1, "x").is_err()
            }),
            ("truncate", |text| text.try_truncate(3).is_err()),
            ("clear", |text| text.try_clear().is_err()),
            ("retain", |text| {
                text.try_retain(|character| character != 's').is_err()
            }),
            ("remove", |text| text.try_remove(0).is_err()),
            ("pop", |text| text.try_pop().is_err()),
            ("map", |text| text.map(|value| value.clear()).is_err()),
        ];

        for (name, mutate) in mutations {
            let (text, watch) = tracked_text("safe");
            assert!(mutate(text), "{name} must reject invalid output");
            watch.assert_released();
        }
    }

    #[test]
    fn invalid_add_panics_and_releases_the_consumed_buffer() {
        let (text, watch) = tracked_text("safe");
        assert_panics(move || text + "!");
        watch.assert_released();
    }

    #[test]
    fn native_string_index_panics_release_the_consumed_buffer() {
        type Mutation = fn(OwnedText);
        let mutations: [(&str, Mutation); 9] = [
            ("insert inside UTF-8", |text| {
                let _ = text.try_insert(5, 'x');
            }),
            ("insert_str inside UTF-8", |text| {
                let _ = text.try_insert_str(5, "x");
            }),
            ("remove inside UTF-8", |text| {
                let _ = text.try_remove(5);
            }),
            ("replace_range inside UTF-8", |text| {
                let _ = text.try_replace_range(5..6, "x");
            }),
            ("truncate inside UTF-8", |text| {
                let _ = text.try_truncate(5);
            }),
            ("insert past end", |text| {
                let _ = text.try_insert(99, 'x');
            }),
            ("insert_str past end", |text| {
                let _ = text.try_insert_str(99, "x");
            }),
            ("remove past end", |text| {
                let _ = text.try_remove(99);
            }),
            ("replace_range past end", |text| {
                let _ = text.try_replace_range(99..100, "x");
            }),
        ];

        for (_, mutate) in mutations {
            let (text, watch) = tracked_text(MY_WORD);
            assert_panics(move || mutate(text));
            watch.assert_released();
        }
    }

    #[test]
    fn retain_predicate_panic_drops_partially_mutated_buffer_and_owned_capture()
    {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        assert_panics(AssertUnwindSafe(move || {
            let mut visited = 0;
            let _ = text.try_retain(move |_| {
                probe.assert_live();
                visited += 1;
                if visited == 3 {
                    panic!("retain predicate panic");
                }
                return visited != 1;
            });
        }));
        assert_eq!(
            drops.get(),
            1,
            "predicate-owned state must be dropped once"
        );
        watch.assert_released();
    }

    #[test]
    fn map_panic_drops_mutated_buffer_and_owned_capture() {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        assert_panics(AssertUnwindSafe(move || {
            let _ = text.map(move |value| {
                probe.assert_live();
                value.clear();
                value.push_str("invalid");
                panic!("map callback panic");
            });
        }));
        assert_eq!(drops.get(), 1, "map-owned state must be dropped once");
        watch.assert_released();
    }

    #[test]
    fn validator_panic_drops_the_mutated_buffer() {
        let (text, watch) = tracked_text("safe");
        assert_panics(AssertUnwindSafe(move || {
            let _ = text.try_replace_range(.., "panic");
        }));
        watch.assert_released();
    }

    #[test]
    fn validator_panic_after_map_drops_buffer_and_owned_capture() {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        assert_panics(AssertUnwindSafe(move || {
            let _ = text.map(move |value| {
                probe.assert_live();
                value.clear();
                value.push_str("panic");
            });
        }));
        assert_eq!(drops.get(), 1);
        watch.assert_released();
    }

    #[test]
    fn rejected_map_drops_its_owned_capture_and_buffer() {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        let result = text.map(move |value| {
            probe.assert_live();
            value.clear();
        });
        assert!(result.is_err());
        assert_eq!(drops.get(), 1);
        watch.assert_released();
    }

    #[test]
    fn rejected_retain_drops_its_owned_capture_and_buffer() {
        let drops = Rc::new(Cell::new(0));
        let probe = DropProbe(Rc::clone(&drops));
        let (text, watch) = tracked_text("safe");
        let result = text.try_retain(move |_| {
            probe.assert_live();
            return false;
        });
        assert!(result.is_err());
        assert_eq!(drops.get(), 1);
        watch.assert_released();
    }
}
