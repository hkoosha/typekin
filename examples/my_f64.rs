// `without = fn_signum` drops the generated `signum` for a custom one.
#[typekin::float(konst = none, without = fn_signum)]
pub struct Meters(f64);

impl Meters {
    /// `0.0` for either zero, unlike `f64::signum`.
    pub fn signum(self) -> Self {
        let raw = self.raw();
        return match raw == 0.0 {
            true => Self::make(0.0),
            false => Self::make(raw.signum()),
        };
    }
}

fn main() {
    let width = Meters::make(2.5);
    let height = Meters::make(4.0);

    let area = width * height;
    assert_eq!(area.raw(), 10.0);
    assert_eq!((-width).raw(), -2.5);

    // NaN-able: no `Eq`, `Ord`, or `Hash`.
    let nan = Meters::make(f64::NAN);
    assert_eq!(nan.partial_cmp(&width), None);

    // Checked conversions reject rounding, NaN, and out-of-range values.
    assert_eq!(height.try_into_u8(), Ok(4));
    assert_eq!(width.try_into_u8(), Err(()));
    assert_eq!(nan.try_into_i64(), Err(()));

    assert_eq!(Meters::make(0.0).signum().raw(), 0.0);
    assert_eq!(width.signum().raw(), 1.0);

    let total: Meters = [
        width,
        height,
    ]
    .iter()
    .sum();
    println!("{total:?} {area:e}");
}
