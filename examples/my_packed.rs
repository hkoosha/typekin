//! Packed records: a named-field struct becomes a transparent `u32` whose
//! fields are typed bit windows. `make` and the `with_*` setters never
//! validate: each field value is already valid, and rustc proves that every
//! value of each field type fits its window. Only raw ingress (`try_make`)
//! checks the fields and the unclaimed bits.

#[typekin::integral(domain = 1..)]
pub struct PageId(u8);

#[typekin::integral(domain = ..64)]
pub struct Version(u8);

#[typekin::bitflag(repr = u8)]
pub enum PageState {
    Dirty = 0b01,
    Pinned = 0b10,
}

// `domain = ..=64` on `Version` would not compile:
// error: `Version` does not fit bits 10..16 of `PageHeader`: 64 needs more
// than 6 bits
#[typekin::packed(repr = u32, with = non_zero)]
pub struct PageHeader {
    /// Page number; 0 is reserved.
    #[bits(24..32)]
    pub id: PageId,
    #[bits(16..24)]
    pub state: PageStateValue,
    /// 6 bits: `..64` is proven to fit.
    #[bits(10..16)]
    pub version: Version,
    #[bit(9)]
    pub sealed: bool,
    // Bit 8 is unclaimed: always zero.
    #[bits(0..8)]
    pub data: u8,
}

fn main() {
    let id = PageId::try_make(5).unwrap();
    let version = Version::try_make(42).unwrap();

    // Infallible, no validation.
    let header = PageHeader::make(PageHeaderValues {
        id,
        state: PageState::Dirty.into_value(),
        version,
        sealed: false,
        data: 0xFF,
    });
    let header = header.with_state(PageState::Pinned.into_value());
    let PageHeaderValues { data, .. } = header.fields();

    println!("{header:?} = {:#010x}", header.raw());
    println!("id {} data {data:#x}", header.id().raw());

    // Raw ingress validates: `id` bits 0, then unclaimed bit 8 set.
    assert!(PageHeader::try_make(0x0000_00FF).is_err());
    assert!(PageHeader::try_make(0x0100_0100).is_err());

    // `NonZero` storage, proven through `id`.
    assert_eq!(size_of::<Option<PageHeader>>(), 4);
}
