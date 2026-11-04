//! A packed record (`raw_*` is hand-written shifts and masks on the `u32`,
//! `wrapped_*` typekin's `make`, `with_*`, and getters). Optimized pairs must
//! be identical; `PACKED_CONTROL_OPERATIONS` pairs the bare shift with the
//! `id` getter, which re-checks `PageId`'s domain and must still differ.

use std::num::NonZeroU8;

#[typekin::integral(domain = 1..)]
pub struct PageId(u8);

#[typekin::integral(domain = ..64)]
pub struct Version(u8);

#[typekin::bitflag(repr = u8)]
pub enum PageState {
    Dirty = 0b01,
    Pinned = 0b10,
}

#[typekin::packed(repr = u32)]
pub struct PageHeader {
    #[bits(24..32)]
    pub id: PageId,
    #[bits(16..24)]
    pub state: PageStateValue,
    #[bits(10..16)]
    pub version: Version,
    #[bit(9)]
    pub sealed: bool,
    #[bits(0..8)]
    pub data: u8,
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_packed_make(
    id: NonZeroU8,
    state: u8,
    version: u8,
    sealed: bool,
    data: u8,
) -> u32 {
    return ((id.get() as u32) << 24)
        | ((state as u32) << 16)
        | ((version as u32) << 10)
        | ((sealed as u32) << 9)
        | data as u32;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_packed_make(
    id: PageId,
    state: PageStateValue,
    version: Version,
    sealed: bool,
    data: u8,
) -> PageHeader {
    return PageHeader::make(PageHeaderValues {
        id,
        state,
        version,
        sealed,
        data,
    });
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_packed_with_state(
    header: u32,
    state: u8,
) -> u32 {
    return (header & !0x00FF_0000) | ((state as u32) << 16);
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_packed_with_state(
    header: PageHeader,
    state: PageStateValue,
) -> PageHeader {
    return header.with_state(state);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_packed_version(header: u32) -> u8 {
    return ((header >> 10) & 0x3F) as u8;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_packed_version(header: PageHeader) -> Version {
    return header.version();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_packed_data(header: u32) -> u8 {
    return header as u8;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_packed_data(header: PageHeader) -> u8 {
    return header.data();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_packed_sealed(header: u32) -> bool {
    return (header >> 9) & 1 != 0;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_packed_sealed(header: PageHeader) -> bool {
    return header.sealed();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_packed_id(header: u32) -> u8 {
    return (header >> 24) as u8;
}

/// `id` re-checks `PageId`'s `1..` domain on read (PLAN_PACKED §4.3): a
/// compare and a cold panic the bare shift does not have.
#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_packed_id(header: PageHeader) -> PageId {
    return header.id();
}

/// Through pointers: LLVM folds calls whose result it can compute, and a
/// linked binary drops the uncalled functions.
pub fn main() {
    let raw_make: extern "C" fn(NonZeroU8, u8, u8, bool, u8) -> u32 =
        std::hint::black_box(raw_packed_make);
    let make: extern "C" fn(
        PageId,
        PageStateValue,
        Version,
        bool,
        u8,
    ) -> PageHeader = std::hint::black_box(wrapped_packed_make);
    let raw_with_state: extern "C" fn(u32, u8) -> u32 =
        std::hint::black_box(raw_packed_with_state);
    let with_state: extern "C" fn(PageHeader, PageStateValue) -> PageHeader =
        std::hint::black_box(wrapped_packed_with_state);
    let raw_version: extern "C" fn(u32) -> u8 =
        std::hint::black_box(raw_packed_version);
    let version: extern "C" fn(PageHeader) -> Version =
        std::hint::black_box(wrapped_packed_version);
    let raw_data: extern "C" fn(u32) -> u8 =
        std::hint::black_box(raw_packed_data);
    let data: extern "C" fn(PageHeader) -> u8 =
        std::hint::black_box(wrapped_packed_data);
    let raw_sealed: extern "C" fn(u32) -> bool =
        std::hint::black_box(raw_packed_sealed);
    let sealed: extern "C" fn(PageHeader) -> bool =
        std::hint::black_box(wrapped_packed_sealed);
    let raw_id: extern "C" fn(u32) -> u8 = std::hint::black_box(raw_packed_id);
    let id: extern "C" fn(PageHeader) -> PageId =
        std::hint::black_box(wrapped_packed_id);
    for (raw_id_value, raw_version_value, sealed_value, data_value) in [
        (1u8, 0u8, false, 0u8),
        (5, 42, true, 0xFF),
        (255, 63, false, 7),
    ] {
        let page_id = PageId::try_make(raw_id_value).unwrap();
        let page_version = Version::try_make(raw_version_value).unwrap();
        let state = PageState::Dirty.into_value();
        let raw_header = raw_make(
            NonZeroU8::new(raw_id_value).unwrap(),
            state.raw(),
            raw_version_value,
            sealed_value,
            data_value,
        );
        let header =
            make(page_id, state, page_version, sealed_value, data_value);
        assert_eq!(raw_header, header.raw());
        let pinned = PageState::Pinned.into_value();
        let raw_updated = raw_with_state(raw_header, pinned.raw());
        let updated = with_state(header, pinned);
        assert_eq!(raw_updated, updated.raw());
        assert_eq!(raw_version(raw_header), version(header).raw());
        assert_eq!(raw_data(raw_header), data(header));
        assert_eq!(raw_sealed(raw_header), sealed(header));
        assert_eq!(raw_id(raw_header), id(header).raw());
    }
}
