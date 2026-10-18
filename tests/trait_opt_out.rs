#[typekin::integral(konst = none, without = [clone, copy, repr])]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct ManualIntegral(u8);

#[typekin::text(konst = none, without = [clone, repr])]
#[repr(transparent)]
#[derive(Clone)]
struct ManualText(String);

#[derive(Copy, Clone)]
#[repr(u8)]
#[typekin::bitflag(konst = none, without = [clone, copy, repr])]
enum ManualFlag {
    Read = 1,
}

#[test]
fn traits_and_layout_can_be_supplied_by_the_caller() {
    fn accepts_copy_clone<T: Copy + Clone>() {}
    fn accepts_clone<T: Clone>() {}

    accepts_copy_clone::<ManualIntegral>();
    accepts_copy_clone::<ManualFlag>();
    accepts_clone::<ManualText>();

    assert_eq!(ManualIntegral::make(1).raw(), 1);
    assert_eq!(ManualFlag::Read.raw(), 1);
    assert_eq!(ManualText::try_from_str("text").unwrap().as_str(), "text");
}
