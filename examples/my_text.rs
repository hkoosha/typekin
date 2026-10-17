#[typekin::text(
        konst = stable,
        std = true,
        in = ["draft", "published", "published-new", "archive", "-"],
        friends = into_my_tag(MyHeadline<'_>) -> Make,
    )]
#[repr(transparent)]
#[derive(Clone)]
pub struct MyTag(String);

const fn tag_len(tag: &MyTag) -> usize {
    return tag.len();
}

pub struct MyHeadline<'a>(pub &'a str);

fn into_my_tag(source: MyHeadline) -> String {
    return source
        .0
        .splitn(2, ':')
        .next()
        .expect("missing tag in headline")
        .to_string()
        .to_lowercase();
}

fn main() {
    fn assert_panics<T>(f: impl FnOnce() -> T + std::panic::UnwindSafe) {
        std::panic::set_hook(Box::new(move |_| {}));
        let result = std::panic::catch_unwind(f);
        let _ = std::panic::take_hook();
        assert!(result.is_err());
    }

    let it = MyTag::try_make("draft".to_string()).unwrap();
    assert_eq!(tag_len(&it), 5);
    let it = it.try_replace_range(.., "published").unwrap();

    assert!(it.clone().try_clear().is_err());
    println!("empty text rejected: true");

    assert_panics(|| it.clone() + "-foo");
    println!("invalid suffix rejected: true");

    assert_panics(|| MyTag::of(MyHeadline("draft bad friend")));
    println!("from invalid friend rejected: true");

    let that = MyHeadline("Archive: something happened");
    println!("from valid friend: {}", MyTag::of(that));
    println!("from valid string op: `{}`", it.clone() + "-new");
}
