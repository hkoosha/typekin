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

#[repr(transparent)]
struct Slug(String);

#[allow(dead_code)]
#[allow(unused_qualifications)]
const _: () = {
    impl Slug {
        #[inline(always)]
        fn is_valid(value: &::alloc::string::String) -> bool {
            (is_slug(value.as_str()))
                && (((value.as_str() == "draft")
                    || (value.as_str() == "published"))
                    || (value.as_str() == "published-news"))
        }
        #[inline(always)]
        pub fn try_make(
            value: ::alloc::string::String
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
            return Self::try_make(::alloc::string::String::from(value));
        }
        #[inline(always)]
        pub fn map<F>(
            self,
            function: F,
        ) -> ::core::result::Result<Self, ()>
        where
            F: ::core::ops::FnOnce(&mut ::alloc::string::String),
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
        pub fn into_inner(self) -> ::alloc::string::String {
            return self.0;
        }
        #[must_use]
        #[inline(always)]
        pub fn into_bytes(self) -> ::alloc::vec::Vec<u8> {
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
    impl ::core::ops::Add<&str> for Slug {
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
    impl ::core::convert::AsRef<str> for Slug {
        #[inline(always)]
        fn as_ref(&self) -> &str {
            return self.as_str();
        }
    }
    impl ::core::convert::AsRef<[u8]> for Slug {
        #[inline(always)]
        fn as_ref(&self) -> &[u8] {
            return self.as_bytes();
        }
    }
    impl ::core::borrow::Borrow<str> for Slug {
        #[inline(always)]
        fn borrow(&self) -> &str {
            return self.as_str();
        }
    }
    impl ::core::ops::Deref for Slug {
        type Target = str;
        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            return self.as_str();
        }
    }
    impl ::core::fmt::Debug for Slug {
        #[inline(always)]
        fn fmt(
            &self,
            formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            return ::core::fmt::Debug::fmt(self.as_str(), formatter);
        }
    }
    impl ::core::hash::Hash for Slug {
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
    impl ::core::cmp::PartialEq for Slug {
        #[inline(always)]
        fn eq(
            &self,
            other: &Self,
        ) -> bool {
            return self.as_str() == other.as_str();
        }
    }
    impl ::core::cmp::Eq for Slug {}
    impl ::core::cmp::PartialOrd for Slug {
        #[inline(always)]
        fn partial_cmp(
            &self,
            other: &Self,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            return ::core::option::Option::Some(self.cmp(other));
        }
    }
    impl ::core::cmp::Ord for Slug {
        #[inline(always)]
        fn cmp(
            &self,
            other: &Self,
        ) -> ::core::cmp::Ordering {
            return self.as_str().cmp(other.as_str());
        }
    }
    impl ::core::convert::From<Slug> for ::alloc::string::String {
        #[inline(always)]
        fn from(value: Slug) -> Self {
            return value.into_inner();
        }
    }
    impl ::core::fmt::Display for Slug {
        #[inline(always)]
        fn fmt(
            &self,
            formatter: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            return ::core::fmt::Display::fmt(self.0.as_str(), formatter);
        }
    }
    trait TextMake {
        fn make_text(self) -> Slug;
    }
    impl TextMake for SlugSource {
        #[inline(always)]
        fn make_text(self) -> Slug {
            return match Slug::try_make(into_slug(self)) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(()) => {
                    ::core::panic!("invalid value from untrusted text friend")
                }
            };
        }
    }
    impl Slug {
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

fn main() {
    let slug = Slug::of(SlugSource(String::from("draft")));
    let slug = slug
        .try_replace_range(.., "published")
        .expect("published is a valid slug");
    assert!(Slug::try_from_str("draft").unwrap().try_clear().is_err());
    let slug = slug + "-news";

    println!("{slug}");
}
