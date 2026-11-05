//! Nominal values are only told apart: no operators, ordering, or numeric
//! conversions; construction, `raw()`, `Eq`, and `Hash` remain.

#[doc(hidden)]
mod __port {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub(super) struct Port(u16);

    #[allow(unused_imports)]
    use super::*;
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_port(self) -> u16;
        }
        trait Make: Seal {
            fn make(self) -> Port;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        impl Seal for Port {
            #[inline(always)]
            fn conv_port(self) -> u16 {
                return Self::raw(self);
            }
        }
        impl Make for Port {
            #[inline(always)]
            fn make(self) -> Port {
                let raw = <Self as Seal>::conv_port(self);
                return Port::_or_panic(raw);
            }
        }
        impl Rel for Port {}
        impl Seal for u16 {
            #[inline(always)]
            fn conv_port(self) -> u16 {
                return self as u16;
            }
        }
        impl Make for u16 {
            #[inline(always)]
            fn make(self) -> Port {
                let raw = <Self as Seal>::conv_port(self);
                return Port::_or_panic(raw);
            }
        }
        impl Seal for u8 {
            #[inline(always)]
            fn conv_port(self) -> u16 {
                return self as u16;
            }
        }
        impl Make for u8 {
            #[inline(always)]
            fn make(self) -> Port {
                let raw = <Self as Seal>::conv_port(self);
                return Port::_or_panic(raw);
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl Seal for usize {
            #[inline(always)]
            fn conv_port(self) -> u16 {
                return self as u16;
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl Make for usize {
            #[inline(always)]
            fn make(self) -> Port {
                let raw = <Self as Seal>::conv_port(self);
                return Port::_or_panic(raw);
            }
        }
        impl core::fmt::Debug for Port {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Port").field(&self.0).finish();
            }
        }
        impl core::hash::Hash for Port {
            #[inline(always)]
            fn hash<H>(
                &self,
                state: &mut H,
            ) where
                H: core::hash::Hasher,
            {
                return core::hash::Hash::hash(&self.0, state);
            }
        }
        impl Eq for Port {}
        if !(size_of::<Port>() == size_of::<u16>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<Port>() == align_of::<u16>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::fmt::Display for Port {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Display::fmt(&raw, f);
            }
        }
        impl<T> PartialEq<T> for Port
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_port(*rhs);
                return lhs == rhs;
            }
        }
        impl Port {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> Port
            where
                T: Make,
            {
                return <T as Make>::make(it);
            }
            #[inline(always)]
            pub const fn make(it: u16) -> Port {
                return Self::_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u16 {
                return self.0;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: u16,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[inline(always)]
            pub const fn try_make(it: u16) -> Result<Self, u16> {
                return Ok(Self::_or_panic(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _or_panic(it: u16) -> Self {
                return Self(it);
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Port {}
    };
}
#[allow(unused_imports)]
use __port::Port;

#[doc(hidden)]
mod __echo_id {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub(super) struct EchoId(u16);

    #[allow(unused_imports)]
    use super::*;
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_echo_id(self) -> u16;
        }
        trait Make: Seal {
            fn make(self) -> EchoId;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        impl Seal for EchoId {
            #[inline(always)]
            fn conv_echo_id(self) -> u16 {
                return Self::raw(self);
            }
        }
        impl Make for EchoId {
            #[inline(always)]
            fn make(self) -> EchoId {
                let raw = <Self as Seal>::conv_echo_id(self);
                return EchoId::_or_panic(raw);
            }
        }
        impl Rel for EchoId {}
        impl Seal for u16 {
            #[inline(always)]
            fn conv_echo_id(self) -> u16 {
                return self as u16;
            }
        }
        impl Make for u16 {
            #[inline(always)]
            fn make(self) -> EchoId {
                let raw = <Self as Seal>::conv_echo_id(self);
                return EchoId::_or_panic(raw);
            }
        }
        impl Seal for u8 {
            #[inline(always)]
            fn conv_echo_id(self) -> u16 {
                return self as u16;
            }
        }
        impl Make for u8 {
            #[inline(always)]
            fn make(self) -> EchoId {
                let raw = <Self as Seal>::conv_echo_id(self);
                return EchoId::_or_panic(raw);
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl Seal for usize {
            #[inline(always)]
            fn conv_echo_id(self) -> u16 {
                return self as u16;
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl Make for usize {
            #[inline(always)]
            fn make(self) -> EchoId {
                let raw = <Self as Seal>::conv_echo_id(self);
                return EchoId::_or_panic(raw);
            }
        }
        impl core::fmt::Debug for EchoId {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("EchoId").field(&self.0).finish();
            }
        }
        impl core::hash::Hash for EchoId {
            #[inline(always)]
            fn hash<H>(
                &self,
                state: &mut H,
            ) where
                H: core::hash::Hasher,
            {
                return core::hash::Hash::hash(&self.0, state);
            }
        }
        impl Eq for EchoId {}
        if !(size_of::<EchoId>() == size_of::<u16>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<EchoId>() == align_of::<u16>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl<T> PartialEq<T> for EchoId
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_echo_id(*rhs);
                return lhs == rhs;
            }
        }
        impl EchoId {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> EchoId
            where
                T: Make,
            {
                return <T as Make>::make(it);
            }
            #[inline(always)]
            pub const fn make(it: u16) -> EchoId {
                return Self::_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u16 {
                return self.0;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: u16,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[inline(always)]
            pub const fn try_make(it: u16) -> Result<Self, u16> {
                return Ok(Self::_or_panic(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _or_panic(it: u16) -> Self {
                return Self(it);
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl EchoId {}
    };
}
#[allow(unused_imports)]
use __echo_id::EchoId;

#[doc(hidden)]
mod __header_words {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    #[doc = " An IPv4 IHL or TCP data offset: a header length in 32-bit words."]
    pub(super) struct HeaderWords(core::num::NonZeroU8);

    #[allow(unused_imports)]
    use super::*;
    #[doc = " Builds a [`HeaderWords`] from a constant checked at compile time: an invalid value is a compile error."]
    #[doc = ""]
    #[doc = " `header_words!(value)` uses a `const` item, checked by `cargo check`; `header_words!(const value)` uses an inline `const` block."]
    #[allow(unused_macros)]
    macro_rules! header_words {
        (const $value: expr) => {
            const {
                match HeaderWords::try_make($value) {
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_1155c682096456e2();
                        value
                    }
                    ::core::result::Result::Err(_) => ::core::panic!(
                        "{}",
                        ::core::concat!(
                            ::core::stringify!($value),
                            " is outside the domain of HeaderWords"
                        ),
                    ),
                }
            }
        };
        ($value: expr) => {{
            const VALUE: HeaderWords = match HeaderWords::try_make($value) {
                ::core::result::Result::Ok(value) => {
                    value.__typekin_lit_1155c682096456e2();
                    value
                }
                ::core::result::Result::Err(_) => ::core::panic!(
                    "{}",
                    ::core::concat!(
                        ::core::stringify!($value),
                        " is outside the domain of HeaderWords"
                    ),
                ),
            };
            VALUE
        }};
    }
    #[allow(unused_imports)]
    pub(crate) use header_words;
    impl HeaderWords {
        #[doc(hidden)]
        #[inline(always)]
        pub const fn __typekin_lit_1155c682096456e2(&self) {}
    }
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_header_words(self) -> u8;
        }
        trait Make: Seal {
            fn make(self) -> HeaderWords;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        impl Seal for HeaderWords {
            #[inline(always)]
            fn conv_header_words(self) -> u8 {
                return Self::raw(self);
            }
        }
        impl Make for HeaderWords {
            #[inline(always)]
            fn make(self) -> HeaderWords {
                let raw = <Self as Seal>::conv_header_words(self);
                return HeaderWords::_or_panic(raw);
            }
        }
        impl Rel for HeaderWords {}
        impl core::fmt::Debug for HeaderWords {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f
                    .debug_tuple("HeaderWords")
                    .field(&self.0.get())
                    .finish();
            }
        }
        impl core::hash::Hash for HeaderWords {
            #[inline(always)]
            fn hash<H>(
                &self,
                state: &mut H,
            ) where
                H: core::hash::Hasher,
            {
                return core::hash::Hash::hash(&self.0.get(), state);
            }
        }
        impl Eq for HeaderWords {}
        if !(size_of::<HeaderWords>() == size_of::<u8>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<HeaderWords>() == align_of::<u8>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl<T> PartialEq<T> for HeaderWords
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_header_words(*rhs);
                return lhs == rhs;
            }
        }
        impl HeaderWords {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> HeaderWords
            where
                T: Make,
            {
                return <T as Make>::make(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u8 {
                return self.0.get();
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: u8,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[inline(always)]
            pub const fn try_make(it: u8) -> Result<Self, u8> {
                return if it >= 5 && it <= 15 {
                    return Ok(Self(match core::num::NonZeroU8::new(it) {
                        Some(value) => value,
                        None => {
                            core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    }));
                }
                else {
                    Err(it)
                };
            }
            #[must_use]
            #[inline(always)]
            pub(self) fn _or_panic(it: u8) -> Self {
                if it >= 5 && it <= 15 {
                    return Self(match core::num::NonZeroU8::new(it) {
                        Some(value) => value,
                        None => {
                            core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    });
                }
                else {
                    core::panic!("invalid value");
                };
            }
            #[doc = r" Builds a result that an expansion-time closure proof"]
            #[doc = r" shows valid; debug builds still check it."]
            #[must_use]
            #[inline(always)]
            pub(self) const fn _proven(it: u8) -> Self {
                debug_assert!(
                    ((it >= (5)) && (it <= (15))),
                    "typekin closure proof violated",
                );
                return Self(match core::num::NonZeroU8::new(it) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
                });
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl HeaderWords {}
    };
}
#[allow(unused_imports)]
use __header_words::HeaderWords;
#[allow(unused_imports)]
use __header_words::header_words;

fn main() {
    let http = Port::make(80);
    let id = EchoId::make(80);
    let words = header_words!(5);

    // Rejected by the types: `http == id`, `http + 1`, `http < http`, and
    // `http.into_u32()`.
    assert_eq!(http.raw(), id.raw());
    assert_eq!(HeaderWords::try_make(4), Err(4));
    println!("port {http}, header {} bytes", usize::from(words.raw()) * 4);
}
