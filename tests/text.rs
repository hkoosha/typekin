#![allow(dead_code)]
extern crate alloc;

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
struct TrustedSlugSource(String);

impl SlugSource {
    fn into_string(self) -> String {
        return self.0;
    }
}

impl TrustedSlugSource {
    fn into_string(self) -> String {
        return self.0;
    }
}

#[typekin::text(
    konst = false,
    valid = [is_slug, is_not_reserved],
    in = ["draft", "published"],
    friends = [
        SlugSource(conv = SlugSource::into_string, cap = [Make]),
        TrustedSlugSource(conv = TrustedSlugSource::into_string, cap = [Make], trusted = true),
    ],
    with = [display],
)]
#[repr(transparent)]
struct Slug(String);

#[typekin::text(konst = false)]
#[repr(transparent)]
struct PlainText(String);

#[test]
fn validates_callbacks_and_literal_membership() {
    assert_eq!(
        Slug::try_from_str("draft").map(Slug::into_inner),
        Ok("draft".into())
    );
    assert_eq!(Slug::try_from_str("reserved"), Err(()));
    assert_eq!(Slug::try_from_str("other"), Err(()));
    assert_eq!(Slug::try_from_str("Draft"), Err(()));
}

#[test]
fn maps_only_to_valid_text() {
    let published = Slug::try_from_str("draft")
        .unwrap()
        .map(|value| value.replace_range(.., "published"));
    assert_eq!(published.map(Slug::into_inner), Ok("published".into()));

    let invalid = Slug::try_from_str("draft")
        .unwrap()
        .map(|value| value.push_str("-other"));
    assert_eq!(invalid, Err(()));
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
fn friendship_validates_untrusted_and_bypasses_only_trusted_sources() {
    let valid = Slug::of(SlugSource("draft".into()));
    assert_eq!(valid.as_str(), "draft");

    assert!(
        std::panic::catch_unwind(|| {
            let _ = Slug::of(SlugSource("other".into()));
        })
        .is_err()
    );

    let trusted = Slug::of(TrustedSlugSource("other".into()));
    assert_eq!(trusted.as_str(), "other");
}

#[test]
fn unvalidated_text_has_infallible_conversions() {
    let from_string: PlainText = String::from("any value").into();
    let from_str: PlainText = "another value".into();

    assert_eq!(from_string.into_inner(), "any value");
    assert_eq!(String::from(from_str), "another value");
}
