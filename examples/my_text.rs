extern crate alloc;

use alloc::string::String;

fn is_slug(value: &str) -> bool {
    return !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-');
}

#[typekin::text(
    konst = false,
    valid = is_slug,
    in = ["draft", "published"],
    with = [display],
)]
#[repr(transparent)]
struct Slug(String);

fn main() {
    let slug = Slug::try_from_str("draft").expect("draft is a valid slug");
    let slug = slug
        .map(|value| value.replace_range(.., "published"))
        .expect("published is a valid slug");

    println!("{slug}");
}
