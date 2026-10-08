#[repr(transparent)]
#[derive(Clone)]
pub struct MyTag(String);

#[allow(dead_code)]
#[allow(unused_qualifications)]
const _: () = {
    impl MyTag {
        #[inline(always)]
        fn is_valid(value: &::std::string::String) -> bool {
            ((((value.as_str() == "draft") || (value.as_str() == "published"))
                || (value.as_str() == "published-new"))
                || (value.as_str() == "archive"))
                || (value.as_str() == "-")
        }
        #[inline(always)]
        pub fn try_make(
            value: ::std::string::String
        ) -> ::core::result::Result<Self, ()> {
            return if Self::is_valid(&value) {
                ::core::result::Result::Ok(Self(value))
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        pub fn try_from_str(value: &str) -> ::core::result::Result<Self, ()> {
            return Self::try_make(::std::string::String::from(value));
        }
        #[inline(always)]
        pub fn map<F>(
            self,
            function: F,
        ) -> ::core::result::Result<Self, ()>
        where
            F: ::core::ops::FnOnce(&mut ::std::string::String),
        {
            let mut value = self.0;
            function(&mut value);
            return Self::try_make(value);
        }
        #[inline(always)]
        pub fn try_push(
            self,
            character: char,
        ) -> ::core::result::Result<Self, ()> {
            return self.map(|value| value.push(character));
        }
        #[inline(always)]
        pub fn try_push_str(
            self,
            text: &str,
        ) -> ::core::result::Result<Self, ()> {
            return self.map(|value| value.push_str(text));
        }
        #[inline(always)]
        pub fn try_insert(
            self,
            index: usize,
            character: char,
        ) -> ::core::result::Result<Self, ()> {
            return self.map(|value| value.insert(index, character));
        }
        #[inline(always)]
        pub fn try_insert_str(
            self,
            index: usize,
            text: &str,
        ) -> ::core::result::Result<Self, ()> {
            return self.map(|value| value.insert_str(index, text));
        }
        #[inline(always)]
        pub fn try_replace_range<R>(
            self,
            range: R,
            text: &str,
        ) -> ::core::result::Result<Self, ()>
        where
            R: ::core::ops::RangeBounds<usize>,
        {
            return self.map(|value| value.replace_range(range, text));
        }
        #[inline(always)]
        pub fn try_truncate(
            self,
            length: usize,
        ) -> ::core::result::Result<Self, ()> {
            return self.map(|value| value.truncate(length));
        }
        #[inline(always)]
        pub fn try_clear(self) -> ::core::result::Result<Self, ()> {
            return self.map(|value| value.clear());
        }
        #[inline(always)]
        pub fn try_retain<F>(
            self,
            mut predicate: F,
        ) -> ::core::result::Result<Self, ()>
        where
            F: ::core::ops::FnMut(char) -> bool,
        {
            return self
                .map(|value| value.retain(|character| predicate(character)));
        }
        #[inline(always)]
        pub fn try_remove(
            self,
            index: usize,
        ) -> ::core::result::Result<(Self, char), ()> {
            let mut value = self.0;
            let removed = value.remove(index);
            return match Self::try_make(value) {
                ::core::result::Result::Ok(value) => {
                    ::core::result::Result::Ok((value, removed))
                }
                ::core::result::Result::Err(()) => {
                    ::core::result::Result::Err(())
                }
            };
        }
        #[inline(always)]
        pub fn try_pop(
            self
        ) -> ::core::result::Result<(Self, ::core::option::Option<char>), ()>
        {
            let mut value = self.0;
            let popped = value.pop();
            return match Self::try_make(value) {
                ::core::result::Result::Ok(value) => {
                    ::core::result::Result::Ok((value, popped))
                }
                ::core::result::Result::Err(()) => {
                    ::core::result::Result::Err(())
                }
            };
        }
        #[must_use]
        #[inline(always)]
        pub fn into_inner(self) -> ::std::string::String {
            return self.0;
        }
        #[must_use]
        #[inline(always)]
        pub fn into_bytes(self) -> ::std::vec::Vec<u8> {
            return self.0.into_bytes();
        }
        #[must_use]
        #[inline(always)]
        pub fn as_str(&self) -> &str {
            return self.0.as_str();
        }
        #[must_use]
        #[inline(always)]
        pub fn as_bytes(&self) -> &[u8] {
            return self.0.as_bytes();
        }
        #[must_use]
        #[inline(always)]
        pub fn len(&self) -> usize {
            return self.0.len();
        }
        #[must_use]
        #[inline(always)]
        pub fn is_empty(&self) -> bool {
            return self.0.is_empty();
        }
        #[must_use]
        #[inline(always)]
        pub fn capacity(&self) -> usize {
            return self.0.capacity();
        }
    }
    impl ::core::ops::Add<&str> for MyTag {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            text: &str,
        ) -> Self {
            return match self.try_push_str(text) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(()) => {
                    ::core::panic!("invalid value from text addition")
                }
            };
        }
    }
    impl ::core::convert::AsRef<str> for MyTag {
        #[inline(always)]
        fn as_ref(&self) -> &str {
            return self.as_str();
        }
    }
    impl ::core::convert::AsRef<[u8]> for MyTag {
        #[inline(always)]
        fn as_ref(&self) -> &[u8] {
            return self.as_bytes();
        }
    }
    impl ::core::borrow::Borrow<str> for MyTag {
        #[inline(always)]
        fn borrow(&self) -> &str {
            return self.as_str();
        }
    }
    impl ::core::ops::Deref for MyTag {
        type Target = str;
        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            return self.as_str();
        }
    }
    impl ::core::fmt::Debug for MyTag {
        #[inline(always)]
        fn fmt(
            &self,
            formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            return ::core::fmt::Debug::fmt(self.as_str(), formatter);
        }
    }
    impl ::core::hash::Hash for MyTag {
        #[inline(always)]
        fn hash<H>(
            &self,
            state: &mut H,
        ) where
            H: ::core::hash::Hasher,
        {
            return ::core::hash::Hash::hash(self.as_str(), state);
        }
    }
    impl ::core::cmp::PartialEq for MyTag {
        #[inline(always)]
        fn eq(
            &self,
            other: &Self,
        ) -> bool {
            return self.as_str() == other.as_str();
        }
    }
    impl ::core::cmp::Eq for MyTag {}
    impl ::core::cmp::PartialOrd for MyTag {
        #[inline(always)]
        fn partial_cmp(
            &self,
            other: &Self,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            return ::core::option::Option::Some(self.cmp(other));
        }
    }
    impl ::core::cmp::Ord for MyTag {
        #[inline(always)]
        fn cmp(
            &self,
            other: &Self,
        ) -> ::core::cmp::Ordering {
            return self.as_str().cmp(other.as_str());
        }
    }
    impl ::core::convert::From<MyTag> for ::std::string::String {
        #[inline(always)]
        fn from(value: MyTag) -> Self {
            return value.into_inner();
        }
    }
    impl ::core::fmt::Display for MyTag {
        #[inline(always)]
        fn fmt(
            &self,
            formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            return ::core::fmt::Display::fmt(self.0.as_str(), formatter);
        }
    }
    trait TextMake {
        fn make_text(self) -> MyTag;
    }
    impl TextMake for MyHeadline<'_> {
        #[inline(always)]
        fn make_text(self) -> MyTag {
            return match MyTag::try_make(into_my_tag(self)) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(()) => {
                    ::core::panic!("invalid value from untrusted text friend")
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
