#[repr(transparent)]
#[derive(Clone)]
pub struct MyTag(String);

#[allow(dead_code)]
#[allow(unused_qualifications)]
const _: () = {
    impl MyTag {
        #[inline(always)]
        fn is_valid(value: &String) -> bool {
            value.as_str() == "draft"
                || value.as_str() == "published"
                || value.as_str() == "published-new"
                || value.as_str() == "archive"
                || value.as_str() == "-"
        }
        #[inline(always)]
        pub fn try_make(value: String) -> Result<Self, ()> {
            return if Self::is_valid(&value) {
                Ok(Self(value))
            }
            else {
                Err(())
            };
        }
        #[inline(always)]
        pub fn try_from_str(value: &str) -> Result<Self, ()> {
            return Self::try_make(String::from(value));
        }
        #[inline(always)]
        pub fn map<F>(
            self,
            function: F,
        ) -> Result<Self, ()>
        where
            F: FnOnce(&mut String),
        {
            let mut value = self.0;
            function(&mut value);
            return Self::try_make(value);
        }
        #[inline(always)]
        pub fn try_push(
            self,
            character: char,
        ) -> Result<Self, ()> {
            return self.map(|value| value.push(character));
        }
        #[inline(always)]
        pub fn try_push_str(
            self,
            text: &str,
        ) -> Result<Self, ()> {
            return self.map(|value| value.push_str(text));
        }
        #[inline(always)]
        pub fn try_insert(
            self,
            index: usize,
            character: char,
        ) -> Result<Self, ()> {
            return self.map(|value| value.insert(index, character));
        }
        #[inline(always)]
        pub fn try_insert_str(
            self,
            index: usize,
            text: &str,
        ) -> Result<Self, ()> {
            return self.map(|value| value.insert_str(index, text));
        }
        #[inline(always)]
        pub fn try_replace_range<R>(
            self,
            range: R,
            text: &str,
        ) -> Result<Self, ()>
        where
            R: core::ops::RangeBounds<usize>,
        {
            return self.map(|value| value.replace_range(range, text));
        }
        #[inline(always)]
        pub fn try_truncate(
            self,
            length: usize,
        ) -> Result<Self, ()> {
            return self.map(|value| value.truncate(length));
        }
        #[inline(always)]
        pub fn try_clear(self) -> Result<Self, ()> {
            return self.map(|value| value.clear());
        }
        #[inline(always)]
        pub fn try_retain<F>(
            self,
            mut predicate: F,
        ) -> Result<Self, ()>
        where
            F: FnMut(char) -> bool,
        {
            return self
                .map(|value| value.retain(|character| predicate(character)));
        }
        #[inline(always)]
        pub fn try_remove(
            self,
            index: usize,
        ) -> Result<(Self, char), ()> {
            let mut value = self.0;
            let removed = value.remove(index);
            return match Self::try_make(value) {
                Ok(value) => Ok((value, removed)),
                Err(()) => Err(()),
            };
        }
        #[inline(always)]
        pub fn try_pop(self) -> Result<(Self, Option<char>), ()> {
            let mut value = self.0;
            let popped = value.pop();
            return match Self::try_make(value) {
                Ok(value) => Ok((value, popped)),
                Err(()) => Err(()),
            };
        }
        #[must_use]
        #[inline(always)]
        pub fn into_inner(self) -> String {
            return self.0;
        }
        #[must_use]
        #[inline(always)]
        pub fn into_bytes(self) -> Vec<u8> {
            return self.0.into_bytes();
        }
        #[must_use]
        #[inline(always)]
        pub const fn as_str(&self) -> &str {
            return self.0.as_str();
        }
        #[must_use]
        #[inline(always)]
        pub const fn as_bytes(&self) -> &[u8] {
            return self.0.as_bytes();
        }
        #[must_use]
        #[inline(always)]
        pub const fn len(&self) -> usize {
            return self.0.len();
        }
        #[must_use]
        #[inline(always)]
        pub const fn is_empty(&self) -> bool {
            return self.0.is_empty();
        }
        #[must_use]
        #[inline(always)]
        pub const fn capacity(&self) -> usize {
            return self.0.capacity();
        }
    }
    impl core::ops::Add<&str> for MyTag {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            text: &str,
        ) -> Self {
            return match self.try_push_str(text) {
                Ok(value) => value,
                Err(()) => core::panic!("invalid value from text addition"),
            };
        }
    }
    impl AsRef<str> for MyTag {
        #[inline(always)]
        fn as_ref(&self) -> &str {
            return self.as_str();
        }
    }
    impl AsRef<[u8]> for MyTag {
        #[inline(always)]
        fn as_ref(&self) -> &[u8] {
            return self.as_bytes();
        }
    }
    impl core::borrow::Borrow<str> for MyTag {
        #[inline(always)]
        fn borrow(&self) -> &str {
            return self.as_str();
        }
    }
    impl core::ops::Deref for MyTag {
        type Target = str;
        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            return self.as_str();
        }
    }
    impl core::fmt::Debug for MyTag {
        #[inline(always)]
        fn fmt(
            &self,
            formatter: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            return formatter
                .debug_tuple("MyTag")
                .field(&self.as_str())
                .finish();
        }
    }
    impl core::hash::Hash for MyTag {
        #[inline(always)]
        fn hash<H>(
            &self,
            state: &mut H,
        ) where
            H: core::hash::Hasher,
        {
            return core::hash::Hash::hash(self.as_str(), state);
        }
    }
    impl PartialEq for MyTag {
        #[inline(always)]
        fn eq(
            &self,
            other: &Self,
        ) -> bool {
            return self.as_str() == other.as_str();
        }
    }
    impl Eq for MyTag {}
    impl PartialOrd for MyTag {
        #[inline(always)]
        fn partial_cmp(
            &self,
            other: &Self,
        ) -> Option<core::cmp::Ordering> {
            return Some(self.cmp(other));
        }
    }
    impl Ord for MyTag {
        #[inline(always)]
        fn cmp(
            &self,
            other: &Self,
        ) -> core::cmp::Ordering {
            return self.as_str().cmp(other.as_str());
        }
    }
    impl From<MyTag> for String {
        #[inline(always)]
        fn from(value: MyTag) -> Self {
            return value.into_inner();
        }
    }
    impl core::fmt::Display for MyTag {
        #[inline(always)]
        fn fmt(
            &self,
            formatter: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            return core::fmt::Display::fmt(self.0.as_str(), formatter);
        }
    }
    trait TextMake {
        fn make_text(self) -> MyTag;
    }
    impl TextMake for MyHeadline<'_> {
        #[inline(always)]
        fn make_text(self) -> MyTag {
            return match MyTag::try_make(into_my_tag(self)) {
                Ok(value) => value,
                Err(()) => {
                    core::panic!("invalid value from untrusted text friend")
                }
            };
        }
    }
    impl MyTag {
        #[allow(private_bounds)]
        #[inline(always)]
        pub fn of<T>(value: T) -> Self
        where
            T: TextMake,
        {
            return TextMake::make_text(value);
        }
    }
};

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
