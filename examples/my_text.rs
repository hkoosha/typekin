extern crate alloc;

use alloc::string::String;

struct SlugSource(String);

fn into_slug(source: SlugSource) -> String {
    return source.0;
}

fn is_slug(value: &str) -> bool {
    return !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-');
}

#[typekin::text(
    konst = false,
    valid = is_slug,
    in = ["draft", "published", "published-news"],
    friends = into_slug(SlugSource) -> Make,
    with = [display],
)]
#[repr(transparent)]
struct Slug(String);

fn main() {
    let slug = Slug::of(SlugSource(String::from("draft")));
    let slug = slug
        .try_replace_range(.., "published")
        .expect("published is a valid slug");
    assert!(Slug::try_from_str("draft").unwrap().try_clear().is_err());
    let slug = slug + "-news";

    println!("{slug}");
}
