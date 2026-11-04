use std::collections::BTreeSet;

/// Every range rejects NaN, so `Unit` is totally ordered: `Eq`, `Ord`, and
/// `Hash` are generated. Queries the domain decides, such as `is_nan` and
/// `is_finite`, return constants.
#[typekin::float(domain = 0.0..=1.0, with = display)]
pub struct Unit(f64);

fn main() {
    assert!(Unit::try_make(f64::NAN).is_err());
    assert!(Unit::try_make(1.5).is_err());

    let half = Unit::try_make(0.5).unwrap();
    let quarter = Unit::try_make(0.25).unwrap();
    assert_eq!(half.max(quarter), half);
    assert!(half.is_finite());

    let set = [
        half,
        quarter,
        half,
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(set.len(), 2);

    // `-0.0 == 0.0`, and both hash alike.
    assert_eq!(Unit::try_make(-0.0).unwrap(), Unit::try_make(0.0).unwrap());

    println!("{half:.2} {quarter}");
}
