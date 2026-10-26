mod sealed {
    pub trait Direction {}
}

trait Direction: sealed::Direction {}

struct In;
struct Out;

impl sealed::Direction for In {}
impl sealed::Direction for Out {}
impl Direction for In {}
impl Direction for Out {}

#[typekin::integral(
    konst = none,
    profile = [circular, point],
    produce = Self::into_offset(_) -> SeqOffset<D>,
    facet = circular {
        with = math_add,
        friend = Self::offset_raw(SeqOffset<D>) -> Math,
        produce = Self,
    },
)]
struct SeqPoint<D: Direction>(u32);

#[typekin::delta(konst = none)]
struct SeqOffset<D: Direction>(i32);

#[typekin::wrapping(konst = none)]
struct WrappingCounter(u32);

#[typekin::wrapping(
    konst = none,
    domain = 1..,
)]
struct NonZeroSerial(u32);

impl<D: Direction> SeqPoint<D> {
    fn into_offset(self) -> SeqOffset<D> {
        return SeqOffset::make(self.raw() as i32);
    }

    fn offset_raw(offset: SeqOffset<D>) -> u32 {
        return offset.raw() as u32;
    }
}

fn directional_sequence<D: Direction>() {
    let maximum = SeqPoint::<D>::make(u32::MAX);
    let zero = SeqPoint::<D>::make(0);
    let offset = zero - maximum;

    assert_eq!(offset.raw(), 1);
    assert!(maximum.precedes(zero));
    assert!(!zero.precedes(SeqPoint::make(1 << 31)));

    let wrapped = maximum + SeqOffset::<D>::make(1);
    assert_eq!(wrapped.raw(), 0);

    let offset = SeqOffset::<D>::make(i32::MAX) + SeqOffset::<D>::make(1);
    assert_eq!(offset.raw(), i32::MIN);
}

fn main() {
    directional_sequence::<In>();
    directional_sequence::<Out>();

    let maximum = WrappingCounter::make(u32::MAX);
    let wrapped = maximum + WrappingCounter::make(1);
    assert_eq!(wrapped.raw(), 0);
    assert!(WrappingCounter::make(1) < WrappingCounter::make(2));

    let serial = NonZeroSerial::try_make(7).unwrap();
    assert_eq!(serial.raw(), 7);
    assert_eq!(NonZeroSerial::try_make(0), Err(0));
}
