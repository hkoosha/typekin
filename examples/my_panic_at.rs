#[typekin::integral(
    konst = none,
    panic_at = nothing,
    domain = 1..=100,
)]
struct Percent(u8);

#[typekin::integral(konst = none, panic_at = nothing)]
struct Count(u8);

fn main() {
    let ten = Percent::try_make(10).unwrap();
    let ninety = Percent::try_make(90).unwrap();
    assert_eq!((ten + ten).map(Percent::raw), Ok(20));
    // An invalid result reports its raw value.
    assert_eq!((ninety + ninety).map(Percent::raw), Err(Some(180)));

    // A result that does not exist reports none.
    let quotient = Count::make(1) / Count::make(0);
    assert_eq!(quotient.map(Count::raw), Err(None));
    let sum = Count::make(u8::MAX) + Count::make(1);
    assert_eq!(sum.map(Count::raw), Err(None));
}
