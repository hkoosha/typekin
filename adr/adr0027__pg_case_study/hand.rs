//! The same PostgreSQL types written by hand: std derives plus the `bitflags`
//! crate, covering the API the study's prototype exercised (ADR 0027).

use core::marker::PhantomData;
use core::num::NonZeroU16;
use core::num::NonZeroU32;
use core::num::NonZeroU64;
use core::ops::Add;
use core::ops::Mul;
use core::ops::Sub;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OffsetNumber(NonZeroU16);

impl OffsetNumber {
    pub const MAX: u16 = 2048;

    pub const fn try_make(raw: u16) -> Result<Self, u16> {
        if raw == 0 || raw > Self::MAX {
            return Err(raw);
        }
        return match NonZeroU16::new(raw) {
            Some(it) => Ok(Self(it)),
            None => Err(raw),
        };
    }

    pub const fn raw(self) -> u16 {
        return self.0.get();
    }
}

impl Add<u16> for OffsetNumber {
    type Output = Self;

    fn add(
        self,
        rhs: u16,
    ) -> Self {
        return Self::try_make(self.raw() + rhs).expect("OffsetNumber out of range");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockNumber(u32);

impl BlockNumber {
    pub const fn try_make(raw: u32) -> Result<Self, u32> {
        if raw == u32::MAX {
            return Err(raw);
        }
        return Ok(Self(raw));
    }

    pub const fn raw(self) -> u32 {
        return self.0;
    }
}

/// No `PartialOrd`/`Ord`: xids compare only through `precedes`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NormalXid(NonZeroU32);

impl NormalXid {
    pub const FIRST: u32 = 3;

    pub const fn try_make(raw: u32) -> Result<Self, u32> {
        if raw < Self::FIRST {
            return Err(raw);
        }
        return match NonZeroU32::new(raw) {
            Some(it) => Ok(Self(it)),
            None => Err(raw),
        };
    }

    pub const fn raw(self) -> u32 {
        return self.0.get();
    }

    pub const fn precedes(
        self,
        that: Self,
    ) -> bool {
        let difference = that.raw().wrapping_sub(self.raw());
        return difference != 0 && difference < (1 << 31);
    }

    pub fn advance(self) -> Self {
        let next = self.raw().checked_add(1).unwrap_or(Self::FIRST);
        return Self::try_make(next).expect("normal xid");
    }
}

impl Add for NormalXid {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self {
        return Self::try_make(self.raw().wrapping_add(rhs.raw())).expect("normal xid");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lsn(NonZeroU64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WalBytes(u64);

impl Lsn {
    pub const fn try_make(raw: u64) -> Result<Self, u64> {
        return match NonZeroU64::new(raw) {
            Some(it) => Ok(Self(it)),
            None => Err(raw),
        };
    }

    pub const fn raw(self) -> u64 {
        return self.0.get();
    }
}

impl WalBytes {
    pub const fn make(raw: u64) -> Self {
        return Self(raw);
    }

    pub const fn raw(self) -> u64 {
        return self.0;
    }
}

impl Add<WalBytes> for Lsn {
    type Output = Self;

    fn add(
        self,
        rhs: WalBytes,
    ) -> Self {
        let raw = self.raw().checked_add(rhs.raw()).expect("LSN overflow");
        return Self::try_make(raw).expect("valid LSN");
    }
}

impl Sub<WalBytes> for Lsn {
    type Output = Self;

    fn sub(
        self,
        rhs: WalBytes,
    ) -> Self {
        let raw = self.raw().checked_sub(rhs.raw()).expect("LSN underflow");
        return Self::try_make(raw).expect("valid LSN");
    }
}

impl Sub for Lsn {
    type Output = WalBytes;

    fn sub(
        self,
        rhs: Self,
    ) -> WalBytes {
        let raw = self.raw().checked_sub(rhs.raw()).expect("LSN underflow");
        return WalBytes(Self::try_make(raw).expect("valid LSN").raw());
    }
}

impl core::fmt::Display for Lsn {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let raw = self.raw();
        return write!(f, "{:X}/{:08X}", raw >> 32, raw as u32);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LpOff(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LpLen(u16);

impl LpOff {
    pub const fn try_make(raw: u16) -> Result<Self, u16> {
        if raw >= 0x8000 {
            return Err(raw);
        }
        return Ok(Self(raw));
    }

    pub const fn raw(self) -> u16 {
        return self.0;
    }
}

impl LpLen {
    pub const fn try_make(raw: u16) -> Result<Self, u16> {
        if raw >= 0x8000 {
            return Err(raw);
        }
        return Ok(Self(raw));
    }

    pub const fn raw(self) -> u16 {
        return self.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LpFlags {
    Unused = 0,
    Normal = 1,
    Redirect = 2,
    Dead = 3,
}

impl LpFlags {
    const fn from_bits(raw: u32) -> Self {
        return match raw & 0b11 {
            0 => Self::Unused,
            1 => Self::Normal,
            2 => Self::Redirect,
            _ => Self::Dead,
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ItemIdValues {
    pub off: LpOff,
    pub flags: LpFlags,
    pub len: LpLen,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(u32);

impl ItemId {
    const OFF_MASK: u32 = 0x7FFF;
    const FLAGS_SHIFT: u32 = 15;
    const LEN_SHIFT: u32 = 17;

    pub const fn make(values: ItemIdValues) -> Self {
        return Self(
            values.off.raw() as u32
                | (values.flags as u32) << Self::FLAGS_SHIFT
                | (values.len.raw() as u32) << Self::LEN_SHIFT,
        );
    }

    pub const fn raw(self) -> u32 {
        return self.0;
    }

    pub const fn off(self) -> LpOff {
        return LpOff((self.0 & Self::OFF_MASK) as u16);
    }

    pub const fn flags(self) -> LpFlags {
        return LpFlags::from_bits(self.0 >> Self::FLAGS_SHIFT);
    }

    pub const fn len(self) -> LpLen {
        return LpLen((self.0 >> Self::LEN_SHIFT) as u16);
    }

    pub const fn fields(self) -> ItemIdValues {
        return ItemIdValues {
            off: self.off(),
            flags: self.flags(),
            len: self.len(),
        };
    }

    pub const fn with_off(
        self,
        off: LpOff,
    ) -> Self {
        return Self(self.0 & !Self::OFF_MASK | off.raw() as u32);
    }

    pub const fn with_flags(
        self,
        flags: LpFlags,
    ) -> Self {
        return Self(self.0 & !(0b11 << Self::FLAGS_SHIFT) | (flags as u32) << Self::FLAGS_SHIFT);
    }

    pub const fn with_len(
        self,
        len: LpLen,
    ) -> Self {
        return Self(self.0 & !(Self::OFF_MASK << Self::LEN_SHIFT) | (len.raw() as u32) << Self::LEN_SHIFT);
    }

    pub fn is_used(self) -> bool {
        return self.flags() != LpFlags::Unused;
    }

    pub fn has_storage(self) -> bool {
        return self.len().raw() != 0;
    }
}

impl core::fmt::Debug for ItemId {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        return f
            .debug_struct("ItemId")
            .field("off", &self.off())
            .field("flags", &self.flags())
            .field("len", &self.len())
            .finish();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexTupleSize(u16);

impl IndexTupleSize {
    pub const fn try_make(raw: u16) -> Result<Self, u16> {
        if raw >= 0x2000 {
            return Err(raw);
        }
        return Ok(Self(raw));
    }

    pub const fn raw(self) -> u16 {
        return self.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IndexTupleInfoValues {
    pub size: IndexTupleSize,
    pub am_reserved: bool,
    pub has_varwidths: bool,
    pub has_nulls: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IndexTupleInfo(u16);

impl IndexTupleInfo {
    const SIZE_MASK: u16 = 0x1FFF;

    pub const fn make(values: IndexTupleInfoValues) -> Self {
        return Self(
            values.size.raw()
                | (values.am_reserved as u16) << 13
                | (values.has_varwidths as u16) << 14
                | (values.has_nulls as u16) << 15,
        );
    }

    pub const fn raw(self) -> u16 {
        return self.0;
    }

    pub const fn size(self) -> IndexTupleSize {
        return IndexTupleSize(self.0 & Self::SIZE_MASK);
    }

    pub const fn am_reserved(self) -> bool {
        return self.0 & 1 << 13 != 0;
    }

    pub const fn has_varwidths(self) -> bool {
        return self.0 & 1 << 14 != 0;
    }

    pub const fn has_nulls(self) -> bool {
        return self.0 & 1 << 15 != 0;
    }

    pub const fn with_size(
        self,
        size: IndexTupleSize,
    ) -> Self {
        return Self(self.0 & !Self::SIZE_MASK | size.raw());
    }

    pub const fn with_has_nulls(
        self,
        on: bool,
    ) -> Self {
        return Self(self.0 & !(1 << 15) | (on as u16) << 15);
    }

    pub const fn with_has_varwidths(
        self,
        on: bool,
    ) -> Self {
        return Self(self.0 & !(1 << 14) | (on as u16) << 14);
    }

    pub const fn with_am_reserved(
        self,
        on: bool,
    ) -> Self {
        return Self(self.0 & !(1 << 13) | (on as u16) << 13);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Natts(u16);

impl Natts {
    pub const fn try_make(raw: u16) -> Result<Self, u16> {
        if raw > 1664 {
            return Err(raw);
        }
        return Ok(Self(raw));
    }

    pub const fn raw(self) -> u16 {
        return self.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Infomask2(u16);

impl Infomask2 {
    const NATTS_MASK: u16 = 0x07FF;
    const UNUSED_MASK: u16 = 0x1800;

    pub const fn try_make(raw: u16) -> Result<Self, u16> {
        if raw & Self::UNUSED_MASK != 0 {
            return Err(raw);
        }
        if Natts::try_make(raw & Self::NATTS_MASK).is_err() {
            return Err(raw);
        }
        return Ok(Self(raw));
    }

    pub const fn raw(self) -> u16 {
        return self.0;
    }

    pub const fn natts(self) -> Natts {
        return Natts(self.0 & Self::NATTS_MASK);
    }

    pub const fn keys_updated(self) -> bool {
        return self.0 & 1 << 13 != 0;
    }

    pub const fn hot_updated(self) -> bool {
        return self.0 & 1 << 14 != 0;
    }

    pub const fn heap_only(self) -> bool {
        return self.0 & 1 << 15 != 0;
    }

    pub const fn with_natts(
        self,
        natts: Natts,
    ) -> Self {
        return Self(self.0 & !Self::NATTS_MASK | natts.raw());
    }

    pub const fn with_hot_updated(
        self,
        on: bool,
    ) -> Self {
        return Self(self.0 & !(1 << 14) | (on as u16) << 14);
    }

    pub const fn with_heap_only(
        self,
        on: bool,
    ) -> Self {
        return Self(self.0 & !(1 << 15) | (on as u16) << 15);
    }

    pub const fn with_keys_updated(
        self,
        on: bool,
    ) -> Self {
        return Self(self.0 & !(1 << 13) | (on as u16) << 13);
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Infomask: u16 {
        const HAS_NULL = 0x0001;
        const HAS_VARWIDTH = 0x0002;
        const HAS_EXTERNAL = 0x0004;
        const HAS_OID_OLD = 0x0008;
        const XMAX_KEYSHR_LOCK = 0x0010;
        const COMBOCID = 0x0020;
        const XMAX_EXCL_LOCK = 0x0040;
        const XMAX_LOCK_ONLY = 0x0080;
        const XMAX_SHR_LOCK = 0x0050;
        const XMIN_COMMITTED = 0x0100;
        const XMIN_INVALID = 0x0200;
        const XMIN_FROZEN = 0x0300;
        const XMAX_COMMITTED = 0x0400;
        const XMAX_INVALID = 0x0800;
        const XMAX_IS_MULTI = 0x1000;
        const UPDATED = 0x2000;
        const MOVED_OFF = 0x4000;
        const MOVED_IN = 0x8000;
    }
}

mod sealed {
    pub trait Catalog {}
}

pub trait Catalog: sealed::Catalog {}

pub enum Relation {}
pub enum Type {}
pub enum Proc {}
pub enum Namespace {}

impl sealed::Catalog for Relation {}
impl sealed::Catalog for Type {}
impl sealed::Catalog for Proc {}
impl sealed::Catalog for Namespace {}
impl Catalog for Relation {}
impl Catalog for Type {}
impl Catalog for Proc {}
impl Catalog for Namespace {}

pub struct Oid<K: Catalog>(NonZeroU32, PhantomData<K>);

impl<K: Catalog> Oid<K> {
    pub const fn try_make(raw: u32) -> Result<Self, u32> {
        return match NonZeroU32::new(raw) {
            Some(it) => Ok(Self(it, PhantomData)),
            None => Err(raw),
        };
    }

    pub const fn raw(self) -> u32 {
        return self.0.get();
    }
}

impl<K: Catalog> Clone for Oid<K> {
    fn clone(&self) -> Self {
        return *self;
    }
}

impl<K: Catalog> Copy for Oid<K> {}

impl<K: Catalog> PartialEq for Oid<K> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        return self.0 == other.0;
    }
}

impl<K: Catalog> Eq for Oid<K> {}

impl<K: Catalog> core::hash::Hash for Oid<K> {
    fn hash<H: core::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.0.hash(state);
    }
}

impl<K: Catalog> core::fmt::Debug for Oid<K> {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        return f.debug_tuple("Oid").field(&self.0).finish();
    }
}

/// NaN is rejected, so the order is total.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Selectivity(f64);

impl Selectivity {
    pub fn try_make(raw: f64) -> Result<Self, f64> {
        if !(0.0..=1.0).contains(&raw) {
            return Err(raw);
        }
        return Ok(Self(raw));
    }

    pub const fn raw(self) -> f64 {
        return self.0;
    }

    pub fn clamped(raw: f64) -> Self {
        return Self::try_make(raw.clamp(0.0, 1.0)).unwrap();
    }
}

impl Eq for Selectivity {}

impl Ord for Selectivity {
    fn cmp(
        &self,
        other: &Self,
    ) -> core::cmp::Ordering {
        return self.0.total_cmp(&other.0);
    }
}

impl core::hash::Hash for Selectivity {
    fn hash<H: core::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        (self.0 + 0.0).to_bits().hash(state);
    }
}

impl Mul for Selectivity {
    type Output = Self;

    fn mul(
        self,
        rhs: Self,
    ) -> Self {
        return Self::try_make(self.0 * rhs.0).expect("selectivity out of range");
    }
}

impl Add for Selectivity {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self {
        return Self::try_make(self.0 + rhs.0).expect("selectivity out of range");
    }
}
