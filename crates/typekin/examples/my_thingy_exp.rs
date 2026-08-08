#![allow(
    dead_code,
    deprecated,
    unused_doc_comments,
    unused_attributes,
    unused_mut,
    unused_imports,
    non_upper_case_globals,
    clippy::min_ident_chars,
    clippy::assign_op_pattern,
    clippy::indexing_slicing,
    clippy::same_name_method,
    clippy::iter_without_into_iter,
    clippy::needless_return
)]

#[derive(Copy, PartialEq, Eq, Ord, Hash, Debug, PartialOrd, Clone)]
struct Flags(InternalBitFlags);

#[repr(transparent)]
#[derive(Copy, PartialEq, Eq, Ord, Hash, PartialOrd, Clone)]
struct InternalBitFlags(u32);

impl Flags {
    pub const A: Self = Self::from_bits_retain(0b00000001);
    pub const B: Self = Self::from_bits_retain(0b00000010);
    pub const C: Self = Self::from_bits_retain(0b00000100);
    pub const ABC: Self = Self::from_bits_retain(
        Self::A.bits() | Self::B.bits() | Self::C.bits(),
    );
}

impl bitflags::Flags for Flags {
    const FLAGS: &'static [bitflags::Flag<Flags>] = {
        mod __bitflags_flag_names {
            pub(super) const A: &'static str = "A";
            pub(super) const B: &'static str = "B";
            pub(super) const C: &'static str = "C";
            pub(super) const ABC: &'static str = "ABC";
        }
        &[
            { bitflags::Flag::new(__bitflags_flag_names::A, Flags::A) },
            { bitflags::Flag::new(__bitflags_flag_names::B, Flags::B) },
            { bitflags::Flag::new(__bitflags_flag_names::C, Flags::C) },
            { bitflags::Flag::new(__bitflags_flag_names::ABC, Flags::ABC) },
        ]
    };

    type Bits = u32;

    fn all_named() -> Flags {
        const ALL_NAMED: u32 = {
            let mut truncated = <u32 as bitflags::Bits>::EMPTY;
            let mut i = 0;
            {
                {
                    let flag = &<Flags as bitflags::Flags>::FLAGS[i];
                    if flag.is_named() {
                        truncated = truncated | flag.value().bits();
                    }
                    i += 1;
                }
            };
            {
                {
                    let flag = &<Flags as bitflags::Flags>::FLAGS[i];
                    if flag.is_named() {
                        truncated = truncated | flag.value().bits();
                    }
                    i += 1;
                }
            };
            {
                {
                    let flag = &<Flags as bitflags::Flags>::FLAGS[i];
                    if flag.is_named() {
                        truncated = truncated | flag.value().bits();
                    }
                    i += 1;
                }
            };
            {
                {
                    let flag = &<Flags as bitflags::Flags>::FLAGS[i];
                    if flag.is_named() {
                        truncated = truncated | flag.value().bits();
                    }
                    i += 1;
                }
            };
            let _ = i;
            truncated
        };
        Flags::from_bits_retain(ALL_NAMED)
    }

    fn bits(&self) -> u32 {
        Flags::bits(self)
    }

    fn from_bits_retain(bits: u32) -> Flags {
        Flags::from_bits_retain(bits)
    }
}

impl core::default::Default for InternalBitFlags {
    #[inline]
    fn default() -> Self {
        InternalBitFlags::empty()
    }
}
impl core::fmt::Debug for InternalBitFlags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        if self.is_empty() {
            f.write_fmt(format_args!("{0:#x}", <u32 as bitflags::Bits>::EMPTY))
        }
        else {
            core::fmt::Display::fmt(self, f)
        }
    }
}
impl core::fmt::Display for InternalBitFlags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        bitflags::parser::to_writer(&Flags(*self), f)
    }
}
impl core::str::FromStr for InternalBitFlags {
    type Err = bitflags::parser::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        bitflags::parser::from_str::<Flags>(s).map(|flags| flags.0)
    }
}
impl AsRef<u32> for InternalBitFlags {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
impl From<u32> for InternalBitFlags {
    fn from(bits: u32) -> Self {
        Self::from_bits_retain(bits)
    }
}
impl InternalBitFlags {
    #[doc = " Get a flags value with all bits unset."]
    #[inline]
    pub const fn empty() -> Self {
        Self(<u32 as bitflags::Bits>::EMPTY)
    }
    #[doc = " Get a flags value with all known bits set."]
    #[inline]
    pub const fn all() -> Self {
        const ALL: InternalBitFlags = {
            let mut truncated = <u32 as bitflags::Bits>::EMPTY;
            let mut _i = 0;
            {
                {
                    truncated |=
                        <Flags as bitflags::Flags>::FLAGS[_i].value().bits();
                    _i += 1;
                }
            };
            {
                {
                    truncated |=
                        <Flags as bitflags::Flags>::FLAGS[_i].value().bits();
                    _i += 1;
                }
            };
            {
                {
                    truncated |=
                        <Flags as bitflags::Flags>::FLAGS[_i].value().bits();
                    _i += 1;
                }
            };
            {
                {
                    truncated |=
                        <Flags as bitflags::Flags>::FLAGS[_i].value().bits();
                    _i += 1;
                }
            };
            InternalBitFlags(truncated)
        };
        ALL
    }
    #[doc = " Get the underlying bits value."]
    #[doc = ""]
    #[doc = " The returned value is exactly the bits set in this flags value."]
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    #[doc = " Convert from a bits value."]
    #[doc = ""]
    #[doc = " This method will return `None` if any unknown bits are set."]
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        let truncated = Self::from_bits_truncate(bits).0;
        if truncated == bits {
            Some(Self(bits))
        }
        else {
            None
        }
    }
    #[doc = " Convert from a bits value, unsetting any unknown bits."]
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(bits & Self::all().0)
    }
    #[doc = " Convert from a bits value exactly."]
    #[inline]
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }
    #[doc = " Get a flags value with the bits of a flag with the given name set."]
    #[doc = ""]
    #[doc = " This method will return `None` if `name` is empty or doesn't"]
    #[doc = " correspond to any named flag."]
    #[inline]
    pub fn from_name(name: &str) -> Option<Self> {
        mod __bitflags_flag_names {
            pub(super) const A: &'static str = "A";
            pub(super) const B: &'static str = "B";
            pub(super) const C: &'static str = "C";
            pub(super) const ABC: &'static str = "ABC";
        }
        {
            {
                if name == __bitflags_flag_names::A {
                    return Some(Self(Flags::A.bits()));
                }
            };
        };
        {
            {
                if name == __bitflags_flag_names::B {
                    return Some(Self(Flags::B.bits()));
                }
            };
        };
        {
            {
                if name == __bitflags_flag_names::C {
                    return Some(Self(Flags::C.bits()));
                }
            };
        };
        {
            {
                if name == __bitflags_flag_names::ABC {
                    return Some(Self(Flags::ABC.bits()));
                }
            };
        };
        let _ = name;
        None
    }
    #[doc = " Whether all bits in `self` are unset."]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == <u32 as bitflags::Bits>::EMPTY
    }
    #[doc = " Whether all known bits in this flags value are set."]
    #[inline]
    pub const fn is_all(&self) -> bool {
        Self::all().0 | self.0 == self.0
    }
    #[doc = " Whether any set bits in `other` are also set in `self`."]
    #[inline]
    pub const fn intersects(
        &self,
        other: Self,
    ) -> bool {
        self.0 & other.0 != <u32 as bitflags::Bits>::EMPTY
    }
    #[doc = " Whether all set bits in `other` are also set in `self`."]
    #[inline]
    pub const fn contains(
        &self,
        other: Self,
    ) -> bool {
        self.0 & other.0 == other.0
    }
    #[doc = " The bitwise or (`|`) of the bits in `self` and `other`."]
    #[inline]
    pub fn insert(
        &mut self,
        other: Self,
    ) {
        *self = Self(self.0).union(other);
    }
    #[doc = " The intersection of `self` with the complement of `other` (`&!`)."]
    #[doc = ""]
    #[doc = " This method is not equivalent to `self & !other` when `other` has unknown bits set."]
    #[doc = " `remove` won't truncate `other`, but the `!` operator will."]
    #[inline]
    pub fn remove(
        &mut self,
        other: Self,
    ) {
        *self = Self(self.0).difference(other);
    }
    #[doc = " The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
    #[inline]
    pub fn toggle(
        &mut self,
        other: Self,
    ) {
        *self = Self(self.0).symmetric_difference(other);
    }
    #[doc = " Call `insert` when `value` is `true` or `remove` when `value` is `false`."]
    #[inline]
    pub fn set(
        &mut self,
        other: Self,
        value: bool,
    ) {
        if value {
            self.insert(other);
        }
        else {
            self.remove(other);
        }
    }
    #[doc = " The bitwise and (`&`) of the bits in `self` and `other`."]
    #[inline]
    #[must_use]
    pub const fn intersection(
        self,
        other: Self,
    ) -> Self {
        Self(self.0 & other.0)
    }
    #[doc = " The bitwise or (`|`) of the bits in `self` and `other`."]
    #[inline]
    #[must_use]
    pub const fn union(
        self,
        other: Self,
    ) -> Self {
        Self(self.0 | other.0)
    }
    #[doc = " The intersection of `self` with the complement of `other` (`&!`)."]
    #[doc = ""]
    #[doc = " This method is not equivalent to `self & !other` when `other` has unknown bits set."]
    #[doc = " `difference` won't truncate `other`, but the `!` operator will."]
    #[inline]
    #[must_use]
    pub const fn difference(
        self,
        other: Self,
    ) -> Self {
        Self(self.0 & !other.0)
    }
    #[doc = " The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
    #[inline]
    #[must_use]
    pub const fn symmetric_difference(
        self,
        other: Self,
    ) -> Self {
        Self(self.0 ^ other.0)
    }
    #[doc = " The bitwise negation (`!`) of the bits in `self`, truncating the result."]
    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.0)
    }
}
impl core::fmt::Binary for InternalBitFlags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let inner = self.0;
        core::fmt::Binary::fmt(&inner, f)
    }
}
impl core::fmt::Octal for InternalBitFlags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let inner = self.0;
        core::fmt::Octal::fmt(&inner, f)
    }
}
impl core::fmt::LowerHex for InternalBitFlags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let inner = self.0;
        core::fmt::LowerHex::fmt(&inner, f)
    }
}
impl core::fmt::UpperHex for InternalBitFlags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let inner = self.0;
        core::fmt::UpperHex::fmt(&inner, f)
    }
}
impl core::ops::BitOr for InternalBitFlags {
    type Output = Self;
    #[doc = " The bitwise or (`|`) of the bits in `self` and `other`."]
    #[inline]
    fn bitor(
        self,
        other: InternalBitFlags,
    ) -> Self {
        self.union(other)
    }
}
impl core::ops::BitOrAssign for InternalBitFlags {
    #[doc = " The bitwise or (`|`) of the bits in `self` and `other`."]
    #[inline]
    fn bitor_assign(
        &mut self,
        other: Self,
    ) {
        self.insert(other);
    }
}
impl core::ops::BitXor for InternalBitFlags {
    type Output = Self;
    #[doc = " The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
    #[inline]
    fn bitxor(
        self,
        other: Self,
    ) -> Self {
        self.symmetric_difference(other)
    }
}
impl core::ops::BitXorAssign for InternalBitFlags {
    #[doc = " The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
    #[inline]
    fn bitxor_assign(
        &mut self,
        other: Self,
    ) {
        self.toggle(other);
    }
}
impl core::ops::BitAnd for InternalBitFlags {
    type Output = Self;
    #[doc = " The bitwise and (`&`) of the bits in `self` and `other`."]
    #[inline]
    fn bitand(
        self,
        other: Self,
    ) -> Self {
        self.intersection(other)
    }
}
impl core::ops::BitAndAssign for InternalBitFlags {
    #[doc = " The bitwise and (`&`) of the bits in `self` and `other`."]
    #[inline]
    fn bitand_assign(
        &mut self,
        other: Self,
    ) {
        *self = Self::from_bits_retain(self.bits()).intersection(other);
    }
}
impl core::ops::Sub for InternalBitFlags {
    type Output = Self;
    #[doc = " The intersection of `self` with the complement of `other` (`&!`)."]
    #[doc = ""]
    #[doc = " This method is not equivalent to `self & !other` when `other` has unknown bits set."]
    #[doc = " `difference` won't truncate `other`, but the `!` operator will."]
    #[inline]
    fn sub(
        self,
        other: Self,
    ) -> Self {
        self.difference(other)
    }
}
impl core::ops::SubAssign for InternalBitFlags {
    #[doc = " The intersection of `self` with the complement of `other` (`&!`)."]
    #[doc = ""]
    #[doc = " This method is not equivalent to `self & !other` when `other` has unknown bits set."]
    #[doc = " `difference` won't truncate `other`, but the `!` operator will."]
    #[inline]
    fn sub_assign(
        &mut self,
        other: Self,
    ) {
        self.remove(other);
    }
}
impl core::ops::Not for InternalBitFlags {
    type Output = Self;
    #[doc = " The bitwise negation (`!`) of the bits in `self`, truncating the result."]
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<InternalBitFlags> for InternalBitFlags {
    #[doc = " The bitwise or (`|`) of the bits in each flags value."]
    fn extend<T: IntoIterator<Item = Self>>(
        &mut self,
        iterator: T,
    ) {
        for item in iterator {
            self.insert(item)
        }
    }
}
impl FromIterator<InternalBitFlags> for InternalBitFlags {
    #[doc = " The bitwise or (`|`) of the bits in each flags value."]
    fn from_iter<T: IntoIterator<Item = Self>>(iterator: T) -> Self {
        use core::iter::Extend;
        let mut result = Self::empty();
        result.extend(iterator);
        result
    }
}
impl InternalBitFlags {
    #[doc = " Yield a set of contained flags values."]
    #[doc = ""]
    #[doc = " Each yielded flags value will correspond to a defined named flag. Any unknown bits"]
    #[doc = " will be yielded together as a final flags value."]
    #[inline]
    pub const fn iter(&self) -> bitflags::iter::Iter<Flags> {
        bitflags::iter::Iter::__private_const_new(
            <Flags as bitflags::Flags>::FLAGS,
            Flags::from_bits_retain(self.bits()),
            Flags::from_bits_retain(self.bits()),
        )
    }
    #[doc = " Yield a set of contained named flags values."]
    #[doc = ""]
    #[doc = " This method is like [`iter`](#method.iter), except only yields bits in contained named flags."]
    #[doc = " Any unknown bits, or bits not corresponding to a contained flag will not be yielded."]
    #[inline]
    pub const fn iter_names(&self) -> bitflags::iter::IterNames<Flags> {
        bitflags::iter::IterNames::__private_const_new(
            <Flags as bitflags::Flags>::FLAGS,
            Flags::from_bits_retain(self.bits()),
            Flags::from_bits_retain(self.bits()),
        )
    }
}
impl IntoIterator for InternalBitFlags {
    type Item = Flags;
    type IntoIter = bitflags::iter::Iter<Flags>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl InternalBitFlags {
    #[doc = " Returns a mutable reference to the raw value of the flags currently stored."]
    #[inline]
    pub fn bits_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}
impl Flags {
    #[doc = " Get a flags value with all bits unset."]
    #[inline]
    pub const fn empty() -> Self {
        Self(InternalBitFlags::empty())
    }
    #[doc = " Get a flags value with all known bits set."]
    #[inline]
    pub const fn all() -> Self {
        Self(InternalBitFlags::all())
    }
    #[doc = " Get the underlying bits value."]
    #[doc = ""]
    #[doc = " The returned value is exactly the bits set in this flags value."]
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0.bits()
    }
    #[doc = " Convert from a bits value."]
    #[doc = ""]
    #[doc = " This method will return `None` if any unknown bits are set."]
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match InternalBitFlags::from_bits(bits) {
            Some(bits) => Some(Self(bits)),
            None => None,
        }
    }
    #[doc = " Convert from a bits value, unsetting any unknown bits."]
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(InternalBitFlags::from_bits_truncate(bits))
    }
    #[doc = " Convert from a bits value exactly."]
    #[inline]
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(InternalBitFlags::from_bits_retain(bits))
    }
    #[doc = " Get a flags value with the bits of a flag with the given name set."]
    #[doc = ""]
    #[doc = " This method will return `None` if `name` is empty or doesn't"]
    #[doc = " correspond to any named flag."]
    #[inline]
    pub fn from_name(name: &str) -> Option<Self> {
        match InternalBitFlags::from_name(name) {
            Some(bits) => Some(Self(bits)),
            None => None,
        }
    }
    #[doc = " Whether all bits in `self` are unset."]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[doc = " Whether all known bits in this flags value are set."]
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.0.is_all()
    }
    #[doc = " Whether any set bits in `other` are also set in `self`."]
    #[inline]
    pub const fn intersects(
        &self,
        other: Self,
    ) -> bool {
        self.0.intersects(other.0)
    }
    #[doc = " Whether all set bits in `other` are also set in `self`."]
    #[inline]
    pub const fn contains(
        &self,
        other: Self,
    ) -> bool {
        self.0.contains(other.0)
    }
    #[doc = " The bitwise or (`|`) of the bits in `self` and `other`."]
    #[inline]
    pub fn insert(
        &mut self,
        other: Self,
    ) {
        self.0.insert(other.0)
    }
    #[doc = " The intersection of `self` with the complement of `other` (`&!`)."]
    #[doc = ""]
    #[doc = " This method is not equivalent to `self & !other` when `other` has unknown bits set."]
    #[doc = " `remove` won't truncate `other`, but the `!` operator will."]
    #[inline]
    pub fn remove(
        &mut self,
        other: Self,
    ) {
        self.0.remove(other.0)
    }
    #[doc = " The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
    #[inline]
    pub fn toggle(
        &mut self,
        other: Self,
    ) {
        self.0.toggle(other.0)
    }
    #[doc = " Call `insert` when `value` is `true` or `remove` when `value` is `false`."]
    #[inline]
    pub fn set(
        &mut self,
        other: Self,
        value: bool,
    ) {
        self.0.set(other.0, value)
    }
    #[doc = " The bitwise and (`&`) of the bits in `self` and `other`."]
    #[inline]
    #[must_use]
    pub const fn intersection(
        self,
        other: Self,
    ) -> Self {
        Self(self.0.intersection(other.0))
    }
    #[doc = " The bitwise or (`|`) of the bits in `self` and `other`."]
    #[inline]
    #[must_use]
    pub const fn union(
        self,
        other: Self,
    ) -> Self {
        Self(self.0.union(other.0))
    }
    #[doc = " The intersection of `self` with the complement of `other` (`&!`)."]
    #[doc = ""]
    #[doc = " This method is not equivalent to `self & !other` when `other` has unknown bits set."]
    #[doc = " `difference` won't truncate `other`, but the `!` operator will."]
    #[inline]
    #[must_use]
    pub const fn difference(
        self,
        other: Self,
    ) -> Self {
        Self(self.0.difference(other.0))
    }
    #[doc = " The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
    #[inline]
    #[must_use]
    pub const fn symmetric_difference(
        self,
        other: Self,
    ) -> Self {
        Self(self.0.symmetric_difference(other.0))
    }
    #[doc = " The bitwise negation (`!`) of the bits in `self`, truncating the result."]
    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(self.0.complement())
    }
}
impl core::fmt::Binary for Flags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let inner = self.0;
        core::fmt::Binary::fmt(&inner, f)
    }
}
impl core::fmt::Octal for Flags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let inner = self.0;
        core::fmt::Octal::fmt(&inner, f)
    }
}
impl core::fmt::LowerHex for Flags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let inner = self.0;
        core::fmt::LowerHex::fmt(&inner, f)
    }
}
impl core::fmt::UpperHex for Flags {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> core::fmt::Result {
        let inner = self.0;
        core::fmt::UpperHex::fmt(&inner, f)
    }
}
impl core::ops::BitOr for Flags {
    type Output = Self;
    #[doc = " The bitwise or (`|`) of the bits in `self` and `other`."]
    #[inline]
    fn bitor(
        self,
        other: Flags,
    ) -> Self {
        self.union(other)
    }
}
impl core::ops::BitOrAssign for Flags {
    #[doc = " The bitwise or (`|`) of the bits in `self` and `other`."]
    #[inline]
    fn bitor_assign(
        &mut self,
        other: Self,
    ) {
        self.insert(other);
    }
}
impl core::ops::BitXor for Flags {
    type Output = Self;
    #[doc = " The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
    #[inline]
    fn bitxor(
        self,
        other: Self,
    ) -> Self {
        self.symmetric_difference(other)
    }
}
impl core::ops::BitXorAssign for Flags {
    #[doc = " The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
    #[inline]
    fn bitxor_assign(
        &mut self,
        other: Self,
    ) {
        self.toggle(other);
    }
}
impl core::ops::BitAnd for Flags {
    type Output = Self;
    #[doc = " The bitwise and (`&`) of the bits in `self` and `other`."]
    #[inline]
    fn bitand(
        self,
        other: Self,
    ) -> Self {
        self.intersection(other)
    }
}
impl core::ops::BitAndAssign for Flags {
    #[doc = " The bitwise and (`&`) of the bits in `self` and `other`."]
    #[inline]
    fn bitand_assign(
        &mut self,
        other: Self,
    ) {
        *self = Self::from_bits_retain(self.bits()).intersection(other);
    }
}
impl core::ops::Sub for Flags {
    type Output = Self;
    #[doc = " The intersection of `self` with the complement of `other` (`&!`)."]
    #[doc = ""]
    #[doc = " This method is not equivalent to `self & !other` when `other` has unknown bits set."]
    #[doc = " `difference` won't truncate `other`, but the `!` operator will."]
    #[inline]
    fn sub(
        self,
        other: Self,
    ) -> Self {
        self.difference(other)
    }
}
impl core::ops::SubAssign for Flags {
    #[doc = " The intersection of `self` with the complement of `other` (`&!`)."]
    #[doc = ""]
    #[doc = " This method is not equivalent to `self & !other` when `other` has unknown bits set."]
    #[doc = " `difference` won't truncate `other`, but the `!` operator will."]
    #[inline]
    fn sub_assign(
        &mut self,
        other: Self,
    ) {
        self.remove(other);
    }
}
impl core::ops::Not for Flags {
    type Output = Self;
    #[doc = " The bitwise negation (`!`) of the bits in `self`, truncating the result."]
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<Flags> for Flags {
    #[doc = " The bitwise or (`|`) of the bits in each flags value."]
    fn extend<T: IntoIterator<Item = Self>>(
        &mut self,
        iterator: T,
    ) {
        for item in iterator {
            self.insert(item)
        }
    }
}
impl FromIterator<Flags> for Flags {
    #[doc = " The bitwise or (`|`) of the bits in each flags value."]
    fn from_iter<T: IntoIterator<Item = Self>>(iterator: T) -> Self {
        use core::iter::Extend;
        let mut result = Self::empty();
        result.extend(iterator);
        result
    }
}
impl Flags {
    #[doc = " Yield a set of contained flags values."]
    #[doc = ""]
    #[doc = " Each yielded flags value will correspond to a defined named flag. Any unknown bits"]
    #[doc = " will be yielded together as a final flags value."]
    #[inline]
    pub const fn iter(&self) -> bitflags::iter::Iter<Flags> {
        bitflags::iter::Iter::__private_const_new(
            <Flags as bitflags::Flags>::FLAGS,
            Flags::from_bits_retain(self.bits()),
            Flags::from_bits_retain(self.bits()),
        )
    }
    #[doc = " Yield a set of contained named flags values."]
    #[doc = ""]
    #[doc = " This method is like [`iter`](#method.iter), except only yields bits in contained named flags."]
    #[doc = " Any unknown bits, or bits not corresponding to a contained flag will not be yielded."]
    #[inline]
    pub const fn iter_names(&self) -> bitflags::iter::IterNames<Flags> {
        bitflags::iter::IterNames::__private_const_new(
            <Flags as bitflags::Flags>::FLAGS,
            Flags::from_bits_retain(self.bits()),
            Flags::from_bits_retain(self.bits()),
        )
    }
}
impl IntoIterator for Flags {
    type Item = Flags;
    type IntoIter = bitflags::iter::Iter<Flags>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn main() {}
