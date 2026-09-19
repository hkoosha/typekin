#[repr(transparent)]
#[derive(Copy, Clone)]
#[typekin::integral(konst = false, friends = [
    PageId(conv=id_to_header, level=[Full]),
    PageState(conv=state_to_header, level=[Full]),
    PageData(conv=PageData::to_header, level=[Full]),
])]
struct PageHeader(u32);

//          0b00000101_10101010_00000000_11111111;
// FORMAT:  ^...ID...^ ^......STATE....^ ^.DATA.^

#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageId(u8);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageState(u8);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageData(u8);

fn id_to_header(it: PageId) -> u32 {
    let bits = it.0 as u32;
    return bits << 24;
}

fn state_to_header(it: PageState) -> u32 {
    let bits = it.0 as u32;
    return bits << 8;
}

impl PageData {
    fn to_header(self) -> u32 {
        return self.0 as u32;
    }
}

fn main() {
    let id = PageId(0b0000_0101);
    let state = PageState(0b1010_1010);
    let data = PageData(0b1111_1111);

    // While id, state & data all have value of 0b1111, they will not overwrite
    // each other; because their friendship relationship guards how they are
    // cast into a PageHeader before being bit-or-ed into header:
    let mut header = PageHeader::of(0);
    header |= id;
    header |= state;
    header |= data;

    let expected = 0b00000101_10101010_00000000_11111111;
    //               ^..ID..^ ^......STATE....^ ^.DATA.^

    assert_eq!(header.raw(), expected);
}
