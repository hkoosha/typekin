extern crate alloc;

#[allow(dead_code)]
#[cfg(test)]
mod tests {
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
        konst = false, std = false,
        valid = [is_slug, is_not_reserved],
        in = ["draft", "published", "reserved", "Draft"],
        friends = [
            SlugSource::into_string(SlugSource) -> Make,
            TrustSlugSource::into_string(TrustSlugSource) -> [Make, Trust],
        ],
        with = [display],
    )]
    #[repr(transparent)]
    struct Slug(String);

    #[typekin::text(konst = false, std = false)]
    #[repr(transparent)]
    struct PlainText(String);

    #[typekin::text(
        konst = false, std = false,
        friends = SlugSource::into_string(SlugSource) -> Make,
        valid = is_slug,
    )]
    #[repr(transparent)]
    struct CallbackText(String);

    #[typekin::text(konst = false, std = false, valid = [is_slug, is_not_reserved])]
    #[repr(transparent)]
    struct CallbackChainText(String);

    #[typekin::text(konst = false, std = false, in = ["", "é"])]
    #[repr(transparent)]
    struct LiteralText(String);

    #[typekin::text(konst = false, std = false, friends = _(Self) -> [Rel], with = [display])]
    #[repr(transparent)]
    struct DisplayText(String);

    std::thread_local! {
        static MUTATION_VALIDATIONS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    }

    fn validate_mutation(value: &str) -> bool {
        MUTATION_VALIDATIONS.with(|calls| calls.set(calls.get() + 1));
        return !value.is_empty() && value != "b" && !value.contains('!');
    }

    #[typekin::text(konst = false, std = false, valid = validate_mutation)]
    #[repr(transparent)]
    struct CheckedMutationText(String);

    fn validate_empty_mutation(value: &str) -> bool {
        MUTATION_VALIDATIONS.with(|calls| calls.set(calls.get() + 1));
        return value.is_empty();
    }

    #[typekin::text(konst = false, std = false, valid = validate_empty_mutation)]
    #[repr(transparent)]
    struct CheckedEmptyText(String);

    #[typekin::text(konst = false, std = false, in = ["a", "ab", "abc"])]
    #[repr(transparent)]
    struct MemberMutationText(String);

    #[typekin::text(konst = false, std = false, valid = is_not_reserved)]
    #[repr(transparent)]
    struct MutationAllocationText(String);

    #[test]
    fn validates_callbacks_and_literal_membership() {
        for value in [
            "draft",
            "published",
        ] {
            assert_eq!(
                Slug::try_make(value.into()).map(Slug::into_inner),
                Ok(value.into())
            );
            assert_eq!(
                Slug::try_from_str(value).map(Slug::into_inner),
                Ok(value.into())
            );
        }

        // Each value violates only one constraint: first callback, second
        // callback, or membership. None of those constraints may be skipped.
        for value in [
            "Draft", "reserved", "other",
        ] {
            assert_eq!(Slug::try_make(value.into()), Err(()));
            assert_eq!(Slug::try_from_str(value), Err(()));
        }
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
    fn literal_membership_is_exact_without_callback_validation() {
        for value in [
            "", "é",
        ] {
            assert_eq!(
                LiteralText::try_from_str(value).map(LiteralText::into_inner),
                Ok(value.into())
            );
        }
        for value in [
            "e\u{301}", "É", "é ", "é\0",
        ] {
            assert_eq!(LiteralText::try_from_str(value), Err(()));
        }
    }

    #[test]
    fn maps_only_to_valid_text() {
        let published = Slug::try_from_str("draft")
            .unwrap()
            .map(|value| value.replace_range(.., "published"));
        assert_eq!(published.map(Slug::into_inner), Ok("published".into()));

        for invalid in [
            "Draft", "reserved", "other",
        ] {
            let result = Slug::try_from_str("draft").unwrap().map(|value| {
                value.clear();
                value.push_str(invalid);
            });
            assert_eq!(result, Err(()));
        }
    }

    #[test]
    fn exposes_read_only_string_surface_and_ordering() {
        let draft = Slug::try_from_str("draft").unwrap();
        let published = Slug::try_from_str("published").unwrap();

        assert_eq!(draft.as_str(), "draft");
        assert_eq!(draft.as_bytes(), b"draft");
        assert_eq!(draft.len(), 5);
        assert!(!draft.is_empty());
        assert!(draft.capacity() >= draft.len());
        assert_eq!(<&str>::from(&*draft), "draft");
        assert!(draft < published);
        assert_eq!(draft.to_string(), "draft");
        assert_eq!(draft.into_bytes(), b"draft");
    }

    #[test]
    fn borrowed_str_keys_match_hash_and_ordered_collection_entries() {
        let hashed = std::collections::HashMap::from([
            (Slug::try_from_str("published").unwrap(), 2),
            (Slug::try_from_str("draft").unwrap(), 1),
        ]);
        let ordered = std::collections::BTreeMap::from([
            (Slug::try_from_str("published").unwrap(), 2),
            (Slug::try_from_str("draft").unwrap(), 1),
        ]);

        for (key, expected) in [
            ("draft", Some(&1)),
            ("published", Some(&2)),
            ("other", None),
        ] {
            assert_eq!(hashed.get(key), expected);
            assert_eq!(ordered.get(key), expected);
        }
        assert_eq!(
            ordered.keys().map(Slug::as_str).collect::<Vec<_>>(),
            [
                "draft",
                "published"
            ]
        );
    }

    #[test]
    fn read_only_unicode_views_and_self_relations_are_usable() {
        let value = DisplayText::from("é🦀Z");
        let text = <DisplayText as AsRef<str>>::as_ref(&value);
        let bytes = <DisplayText as AsRef<[u8]>>::as_ref(&value);

        assert_eq!(text, "é🦀Z");
        assert_eq!(bytes, "é🦀Z".as_bytes());
        assert_eq!(text.as_ptr(), value.as_str().as_ptr());
        assert_eq!(bytes.as_ptr(), value.as_bytes().as_ptr());
        assert_eq!(
            value.char_indices().collect::<Vec<_>>(),
            [
                (0, 'é'),
                (2, '🦀'),
                (6, 'Z')
            ]
        );
        assert_eq!(value.get(2..6), Some("🦀"));
        assert_eq!(value.get(1..2), None);

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
    fn friendship_validates_untrusted_and_bypasses_only_trusted_sources() {
        let valid = Slug::of(SlugSource("draft".into()));
        assert_eq!(valid.as_str(), "draft");

        for invalid in [
            "Draft", "reserved", "other",
        ] {
            assert_panics(|| {
                let _ = Slug::of(SlugSource(invalid.into()));
            });

            let trusted = Slug::of(TrustSlugSource(invalid.into()));
            assert_eq!(trusted.as_str(), invalid);
        }
    }

    #[test]
    fn single_owned_friend_preserves_allocation_and_validates() {
        let mut raw = String::with_capacity(64);
        raw.push_str("draft");
        let pointer = raw.as_ptr();
        let capacity = raw.capacity();
        let text = CallbackText::of(SlugSource(raw));
        assert_eq!(text.as_str(), "draft");
        assert_eq!(text.as_str().as_ptr(), pointer);
        assert_eq!(text.into_inner().capacity(), capacity);

        for invalid in [
            "", "Draft", "é",
        ] {
            assert_panics(|| {
                let _ = CallbackText::of(SlugSource(String::from(invalid)));
            });
        }
    }

    #[test]
    fn unvalidated_text_has_infallible_conversions() {
        let from_string: PlainText = String::from("any value").into();
        let from_str: PlainText = "another value".into();

        assert_eq!(from_string.into_inner(), "any value");
        assert_eq!(String::from(from_str), "another value");
    }

    #[test]
    fn transparent_text_preserves_string_layout_and_owned_allocation() {
        assert_eq!(
            core::mem::size_of::<Slug>(),
            core::mem::size_of::<String>()
        );
        assert_eq!(
            core::mem::align_of::<Slug>(),
            core::mem::align_of::<String>()
        );

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
        let text = PlainText::from("a").try_push('é').unwrap();
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

        let (text, removed) = PlainText::from("é").try_remove(0).unwrap();
        assert_eq!(removed, 'é');
        assert_eq!(text.as_str(), "");
    }

    #[test]
    fn checked_replacement_supports_byte_range_variants() {
        fn text() -> PlainText {
            return PlainText::from("aé🍆z");
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
        let text = PlainText::from("aé🍆界")
            .try_retain(move |character| state.keep(character, visited_ref))
            .unwrap();
        assert_eq!(text.as_str(), "a🍆");
        assert_eq!(
            visited,
            [
                'a', 'é', '🍆', '界'
            ]
        );
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
            MUTATION_VALIDATIONS.with(|calls| calls.set(0));
            assert!(mutate(text).is_ok());
            MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 1));
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
            MUTATION_VALIDATIONS.with(|calls| calls.set(0));
            assert_eq!(mutate(text), Err(()));
            MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 1));
        }

        let text = CheckedMutationText::try_from_str("abc").unwrap();
        MUTATION_VALIDATIONS.with(|calls| calls.set(0));
        assert_eq!((text + "d").as_str(), "abcd");
        MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 1));

        let text = CheckedMutationText::try_from_str("abc").unwrap();
        MUTATION_VALIDATIONS.with(|calls| calls.set(0));

        assert_panics(|| text + "!");

        MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 1));
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
            MUTATION_VALIDATIONS.with(|calls| calls.set(0));

            assert_panics(|| mutate(text));

            MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 0));
        }

        assert_panics(|| {
            let _ = PlainText::from("").try_remove(0);
        });
    }

    #[test]
    fn checked_truncate_beyond_length_keeps_text_and_still_validates() {
        let text = CheckedMutationText::try_from_str("é").unwrap();
        MUTATION_VALIDATIONS.with(|calls| calls.set(0));
        let text = text.try_truncate(usize::MAX).unwrap();
        assert_eq!(text.as_str(), "é");
        MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 1));

        let text = CheckedMutationText::try_from_str("é").unwrap();
        MUTATION_VALIDATIONS.with(|calls| calls.set(0));
        let text = text.try_truncate(2).unwrap();
        assert_eq!(text.as_str(), "é");
        MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 1));
    }

    #[test]
    fn checked_mutations_and_add_preserve_the_owned_allocation() {
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
        let text = text + "é🍆";
        check(&text, pointer, capacity, "é🍆");
    }

    #[test]
    fn checked_empty_pop_and_clear_still_validate_once() {
        let text = CheckedEmptyText::try_from_str("").unwrap();
        MUTATION_VALIDATIONS.with(|calls| calls.set(0));
        let (text, removed) = text.try_pop().unwrap();
        assert_eq!(removed, None);
        assert_eq!(text.as_str(), "");
        MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 1));

        MUTATION_VALIDATIONS.with(|calls| calls.set(0));
        let text = text.try_clear().unwrap();
        assert_eq!(text.as_str(), "");
        MUTATION_VALIDATIONS.with(|calls| assert_eq!(calls.get(), 1));
    }

    mod trusted_relation {
        #[typekin::text(
            konst = false, std = false,
            valid = super::is_slug,
            friends = _(Self) -> [Rel, Trust],
        )]
        #[repr(transparent)]
        struct RelatedText(String);

        #[test]
        fn trusted_self_relation_needs_no_conversion_and_does_not_skip_validation()
         {
            let earlier = RelatedText::try_from_str("draft").unwrap();
            let later = RelatedText::try_from_str("published").unwrap();
            assert!(earlier < later);
            assert_eq!(earlier.cmp(&later), core::cmp::Ordering::Less);
            assert_eq!(earlier, RelatedText::try_from_str("draft").unwrap());
            for invalid in [
                "", "Draft", "é",
            ] {
                let checked: Result<RelatedText, ()> =
                    RelatedText::try_make(String::from(invalid));
                assert_eq!(checked, Err(()));
                assert_eq!(RelatedText::try_from_str(invalid), Err(()));
            }
        }
    }

    mod empty_membership {
        use crate::tests::assert_panics;

        struct CheckedSource(String);
        struct TrustSource(String);

        fn checked_text(source: CheckedSource) -> String {
            source.0
        }

        fn trusted_text(source: TrustSource) -> String {
            source.0
        }

        #[typekin::text(
            konst = false, std = false,
            in = [],
            friends = [
                checked_text(CheckedSource) -> Make,
                trusted_text(TrustSource) -> [Make, Trust],
            ],
        )]
        #[repr(transparent)]
        struct EmptyDomain(String);

        #[typekin::text(konst = false, std = false, in = [], valid = super::is_not_reserved)]
        #[repr(transparent)]
        struct CallbackEmptyDomain(String);

        #[typekin::text(konst = false, std = false, in = [""])]
        #[repr(transparent)]
        struct EmptyStringOnly(String);

        #[test]
        fn empty_membership_rejects_every_checked_input_with_and_without_callbacks()
         {
            for value in [
                "", "draft", "reserved", "é🦀", "\0",
            ] {
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
        fn a_single_empty_literal_accepts_only_the_empty_string() {
            assert_eq!(
                EmptyStringOnly::try_make(String::new())
                    .map(EmptyStringOnly::into_inner),
                Ok(String::new()),
            );
            assert_eq!(
                EmptyStringOnly::try_from_str("")
                    .map(EmptyStringOnly::into_inner),
                Ok(String::new()),
            );
            for value in [
                "draft", "é", "\0",
            ] {
                assert_eq!(EmptyStringOnly::try_make(value.into()), Err(()));
                assert_eq!(EmptyStringOnly::try_from_str(value), Err(()));
            }
        }

        #[test]
        fn only_trusted_make_can_construct_an_empty_domain_value() {
            for value in [
                "", "draft", "é🦀",
            ] {
                let checked = CheckedSource(String::from(value));
                assert_panics(|| EmptyDomain::of(checked));
                let trusted = TrustSource(String::from(value));
                assert_eq!(EmptyDomain::of(trusted).into_inner(), value);
            }
        }

        #[test]
        fn trusted_empty_domain_values_revalidate_maps_and_all_checked_mutators()
         {
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
                assert!(
                    mutate(text),
                    "{name} must reject an empty-domain value"
                );
            }

            let text = EmptyDomain::of(TrustSource(String::from("é🦀")));
            assert_panics(|| text + "");
        }
    }

    pub fn assert_panics<T>(f: impl FnOnce() -> T + std::panic::UnwindSafe) {
        std::panic::set_hook(Box::new(move |_| {}));
        let result = std::panic::catch_unwind(f);
        let _ = std::panic::take_hook();
        assert!(result.is_err());
    }
}
