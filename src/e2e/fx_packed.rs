use crate::*;

/// A validated field: three bits hold more than its domain admits.
#[typekin::integral(domain = 1..=5)]
pub(crate) struct Level(u8);

/// A signed field filling exactly a 4-bit window.
#[typekin::integral(domain = -8..8)]
pub(crate) struct Offset(i8);

/// An unvalidated-by-window field: six bits hold exactly its domain.
#[typekin::integral(domain = ..64)]
pub(crate) struct Tag(u8);

/// A validated field, a signed field, a `bool`, an unclaimed gap (bits
/// 8..10), and a field whose window implies its domain.
#[typekin::packed(repr = u16)]
pub(crate) struct Gapped {
    #[bits(0..3)]
    pub(crate) level: Level,
    #[bits(3..7)]
    pub(crate) offset: Offset,
    #[bit(7)]
    pub(crate) flag: bool,
    #[bits(10..16)]
    pub(crate) tag: Tag,
}

pub(crate) const GAPPED_LEVEL_MASK: u16 = 0b111;
pub(crate) const GAPPED_OFFSET_MASK: u16 = 0b1111 << 3;
pub(crate) const GAPPED_FLAG_MASK: u16 = 1 << 7;
pub(crate) const GAPPED_TAG_MASK: u16 = 0b11_1111 << 10;

/// `Gapped`'s validity, written out by hand.
pub(crate) fn gapped_is_valid(raw: u16) -> bool {
    let gap = raw & 0b11 << 8;
    let level = raw & GAPPED_LEVEL_MASK;
    return gap == 0 && (1..=5).contains(&level);
}

pub(crate) fn gapped_sample() -> Gapped {
    return Gapped::make(GappedValues {
        level: Level::try_make(3).unwrap(),
        offset: Offset::try_make(-2).unwrap(),
        flag: true,
        tag: Tag::try_make(42).unwrap(),
    });
}

/// A signed wrapper in a narrow window and a signed primitive in its own
/// width.
#[typekin::packed(repr = u16)]
pub(crate) struct Signed {
    #[bits(0..4)]
    pub(crate) offset: Offset,
    #[bits(4..12)]
    pub(crate) byte: i8,
}

pub(crate) fn signed_of(
    offset: i8,
    byte: i8,
) -> Signed {
    return Signed::make(SignedValues {
        offset: Offset::try_make(offset).unwrap(),
        byte,
    });
}

#[typekin::packed(repr = u16, with = ord)]
pub(crate) struct Inner {
    #[bits(0..4)]
    pub(crate) offset: Offset,
    #[bits(4..12)]
    pub(crate) byte: i8,
    #[bit(15)]
    pub(crate) flag: bool,
}

#[typekin::bitflag(integral = [domain = ..=0b111])]
#[repr(u8)]
pub(crate) enum Access {
    Read = 1,
    Write = 2,
    Exec = 4,
}

/// A packed record filling a full-width window and a bitflag value in a
/// narrowed one.
#[typekin::packed(repr = u32)]
pub(crate) struct Outer {
    #[bits(16..32)]
    pub(crate) inner: Inner,
    #[bits(0..3)]
    pub(crate) access: AccessValue,
    #[bits(8..16)]
    pub(crate) low: u8,
}

#[typekin::integral(domain = 1..)]
pub(crate) struct Id(u8);

#[typekin::packed(repr = u16, with = non_zero)]
pub(crate) struct NonZeroRecord {
    #[bits(8..16)]
    pub(crate) id: Id,
    #[bits(0..8)]
    pub(crate) data: u8,
}

#[typekin::packed(repr = u16, with = serde)]
pub(crate) struct SerdeRecord {
    #[bits(0..3)]
    pub(crate) level: Level,
    #[bits(8..16)]
    pub(crate) byte: u8,
}

#[typekin::packed(repr = u16, without = values_struct)]
pub(crate) struct NoValues {
    #[bits(0..3)]
    pub(crate) level: Level,
    #[bits(8..16)]
    pub(crate) byte: u8,
}

#[typekin::packed(repr = u8, without = [debug, eq, hash])]
pub(crate) struct Bare {
    #[bits(0..8)]
    pub(crate) byte: u8,
}

#[typekin::packed(repr = u16, with = ord)]
pub(crate) struct Ordered {
    #[bits(8..16)]
    pub(crate) high: u8,
    #[bits(0..8)]
    pub(crate) low: u8,
}

#[typekin::packed(repr = u8, values_name = RenamedParts)]
pub(crate) struct Renamed {
    #[bits(0..3)]
    pub(crate) level: Level,
    #[bit(7)]
    pub(crate) flag: bool,
}

#[typekin::packed(repr = u8, hide_in = none)]
pub(crate) struct Visible {
    #[bits(0..8)]
    secret: u8,
}

#[typekin::packed(repr = u8)]
pub(crate) struct Hidden {
    #[bits(0..8)]
    secret: u8,
}

/// Uses `Visible`'s private-field accessors from the defining module.
pub(crate) fn visible_secret_after(
    first: u8,
    second: u8,
) -> (u8, u8) {
    let visible = Visible::make(VisibleValues { secret: first });
    let VisibleValues { secret } = visible.with_secret(second).fields();
    return (visible.secret(), secret);
}

/// Uses `Hidden`'s private-field accessors from the defining module, which
/// is outside the hidden module `__hidden`.
pub(crate) fn hidden_secret_after(
    first: u8,
    second: u8,
) -> (u8, u8) {
    let hidden: __hidden::Hidden = Hidden::make(HiddenValues { secret: first });
    let HiddenValues { secret } = hidden.with_secret(second).fields();
    return (hidden.secret(), secret);
}

#[typekin::integral(konst = nightly, domain = 1..=5)]
pub(crate) struct NightlyLevel(u8);

/// Generated traits are `const` under `konst = nightly`.
#[typekin::packed(repr = u8, konst = nightly)]
pub(crate) struct NightlyRecord {
    #[bits(0..3)]
    pub(crate) level: NightlyLevel,
    #[bit(7)]
    pub(crate) flag: bool,
}
