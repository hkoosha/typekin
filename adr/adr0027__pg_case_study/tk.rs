//! PostgreSQL scalar and bit-layout types written with typekin.

/// `OffsetNumber`: 1..=MaxOffsetNumber (BLCKSZ 8192 / sizeof(ItemIdData)).
#[typekin::integral(domain = 1..=2048)]
pub struct OffsetNumber(u16);

/// `BlockNumber`: `InvalidBlockNumber` (0xFFFFFFFF) is excluded.
#[typekin::integral(domain = ..0xFFFF_FFFF)]
pub struct BlockNumber(u32);

/// Normal transaction ids; 0, 1 and 2 are Invalid, Bootstrap and Frozen.
#[typekin::circular(domain = 3..)]
pub struct NormalXid(u32);

impl NormalXid {
    /// `TransactionIdAdvance`: wraps from u32::MAX to FirstNormalTransactionId.
    pub fn advance(self) -> Self {
        return self.map(|raw: u32| raw.checked_add(1).unwrap_or(3));
    }
}

/// `XLogRecPtr` as a position; `InvalidXLogRecPtr` (0) is excluded.
#[typekin::integral(
    konst = none,
    profile = [sequential, point],
    domain = 1..,
    produce = Self::distance(_) -> WalBytes,
    facet = sequential {
        with = math_add,
        friend = Self::bytes_raw(WalBytes) -> Math,
        produce = Self,
    },
)]
pub struct Lsn(u64);

/// A byte distance in the WAL stream.
#[typekin::sequential(konst = none)]
pub struct WalBytes(u64);

impl Lsn {
    fn distance(self) -> WalBytes {
        return WalBytes::make(self.raw());
    }

    fn bytes_raw(bytes: WalBytes) -> u64 {
        return bytes.raw();
    }
}

impl core::fmt::Display for Lsn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let raw = self.raw();
        return write!(f, "{:X}/{:08X}", raw >> 32, raw as u32);
    }
}

/// `lp_off` and `lp_len`: 15-bit byte offsets/lengths within a page.
#[typekin::integral(domain = ..0x8000)]
pub struct LpOff(u16);

#[typekin::integral(domain = ..0x8000)]
pub struct LpLen(u16);

/// `lp_flags`: LP_UNUSED, LP_NORMAL, LP_REDIRECT, LP_DEAD.
#[typekin::integral(domain = ..4)]
pub struct LpFlags(u8);

impl LpFlags {
    pub const UNUSED: Self = lp_flags!(0);
    pub const NORMAL: Self = lp_flags!(1);
    pub const REDIRECT: Self = lp_flags!(2);
    pub const DEAD: Self = lp_flags!(3);
}

/// `ItemIdData` (little-endian GCC bit-field order).
#[typekin::packed(repr = u32)]
pub struct ItemId {
    #[bits(0..15)]
    pub off: LpOff,
    #[bits(15..17)]
    pub flags: LpFlags,
    #[bits(17..32)]
    pub len: LpLen,
}

impl ItemId {
    pub fn is_used(self) -> bool {
        return self.flags() != LpFlags::UNUSED;
    }

    pub fn has_storage(self) -> bool {
        return self.len().raw() != 0;
    }
}

/// `IndexTupleData.t_info`.
#[typekin::integral(domain = ..0x2000)]
pub struct IndexTupleSize(u16);

#[typekin::packed(repr = u16)]
pub struct IndexTupleInfo {
    #[bits(0..13)]
    pub size: IndexTupleSize,
    #[bit(13)]
    pub am_reserved: bool,
    #[bit(14)]
    pub has_varwidths: bool,
    #[bit(15)]
    pub has_nulls: bool,
}

/// `t_infomask2`: attribute count plus three flags; bits 11..13 unused.
#[typekin::integral(domain = ..=1664)]
pub struct Natts(u16);

#[typekin::packed(repr = u16)]
pub struct Infomask2 {
    #[bits(0..11)]
    pub natts: Natts,
    #[bit(13)]
    pub keys_updated: bool,
    #[bit(14)]
    pub hot_updated: bool,
    #[bit(15)]
    pub heap_only: bool,
}

/// `t_infomask` flags (htup_details.h).
#[typekin::bitflag(repr = u16)]
pub enum Infomask {
    HasNull = 0x0001,
    HasVarWidth = 0x0002,
    HasExternal = 0x0004,
    HasOidOld = 0x0008,
    XmaxKeyshrLock = 0x0010,
    ComboCid = 0x0020,
    XmaxExclLock = 0x0040,
    XmaxLockOnly = 0x0080,
    XmaxShrLock = 0x0050,
    XminCommitted = 0x0100,
    XminInvalid = 0x0200,
    XminFrozen = 0x0300,
    XmaxCommitted = 0x0400,
    XmaxInvalid = 0x0800,
    XmaxIsMulti = 0x1000,
    Updated = 0x2000,
    MovedOff = 0x4000,
    MovedIn = 0x8000,
}

/// Catalog OIDs typed by the catalog they point into.
#[typekin::integral(
    domain = 1..,
    tree = Catalog {
        Relation,
        Type,
        Proc,
        Namespace,
    },
)]
pub struct Oid<K: Catalog>(u32);

/// Planner selectivity: a probability.
#[typekin::float(konst = none, domain = 0.0..=1.0)]
pub struct Selectivity(f64);

impl Selectivity {
    /// `CLAMP_PROBABILITY`.
    pub fn clamped(raw: f64) -> Self {
        return Self::try_make(raw.clamp(0.0, 1.0)).unwrap();
    }
}
