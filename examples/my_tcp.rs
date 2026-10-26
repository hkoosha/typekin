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

#[typekin::circular(
    without = math_add,
    produce = Self::into_offset(_) -> SeqOffset<D>,
    facet = circular {
        with = math_add,
        produce = Self,
        friend = Self::offset_raw(SeqOffset<D>) -> Math,
    },
)]
struct SeqPoint<D: Direction>(u32);

#[typekin::delta]
struct SeqOffset<D: Direction>(i32);

impl<D: Direction> SeqPoint<D> {
    #[inline(always)]
    const fn into_offset(self) -> SeqOffset<D> {
        return match SeqOffset::try_make(self.raw() as i32) {
            Ok(value) => value,
            Err(_) => panic!("unvalidated sequence offset rejected"),
        };
    }
    #[inline(always)]
    const fn offset_raw(offset: SeqOffset<D>) -> u32 {
        return offset.raw() as u32;
    }
}

struct ConnState {
    snd_nxt: SeqPoint<Out>,
    rcv_nxt: SeqPoint<In>,
    snd_una: SeqPoint<Out>,
}

impl ConnState {
    fn sent_but_not_acked(&self) -> SeqOffset<Out> {
        return self.snd_nxt - self.snd_una;
    }
}

fn sequence_arithmetic<D: Direction>() {
    let min = SeqPoint::<D>::make(u32::MIN);
    let max = SeqPoint::<D>::make(u32::MAX);
    let one = SeqOffset::<D>::make(1);
    let two = SeqOffset::<D>::make(2);
    let three = SeqOffset::<D>::make(3);

    assert!(max.precedes(min));
    assert!(!min.precedes(max));
    assert!(!min.precedes(min));
    // Half a sequence space has no defined serial ordering.
    let antipode = SeqPoint::<D>::make(1 << 31);
    assert!(!min.precedes(antipode));
    assert!(!antipode.precedes(min));

    assert_eq!(max + one, min);
    assert_eq!(one + two, three);
    assert_eq!(min - one, max);
    assert_eq!(two - three, SeqOffset::<D>::make(-1));
    assert_eq!(max - min, SeqOffset::<D>::make(-1));

    assert_eq!(size_of::<SeqPoint<D>>(), 4);
    assert_eq!(align_of::<SeqPoint<D>>(), align_of::<u32>());
    assert_eq!(size_of::<SeqOffset<D>>(), 4);
}

fn main() {
    sequence_arithmetic::<In>();
    sequence_arithmetic::<Out>();

    let mut conn = ConnState {
        snd_nxt: SeqPoint::make(7),
        rcv_nxt: SeqPoint::make(3),
        snd_una: SeqPoint::make(1),
    };
    let sent_but_not_acked: SeqOffset<Out> = conn.sent_but_not_acked();
    assert_eq!(sent_but_not_acked.raw(), 6);

    // Incoming progress cannot participate in the outgoing subtraction above.
    let received = SeqOffset::<In>::make(4);
    conn.rcv_nxt = conn.rcv_nxt + received;
    assert_eq!(conn.rcv_nxt.raw(), 7);
    assert_eq!(conn.sent_but_not_acked(), sent_but_not_acked);

    let acknowledged = SeqOffset::<Out>::make(2);
    conn.snd_una = conn.snd_una + acknowledged;
    assert_eq!(conn.sent_but_not_acked().raw(), 4);

    // Rejected by the types: point + point, offset + point, offset - point,
    // point<Out> + offset<In>, and conn.rcv_nxt - conn.snd_una.
    println!("TCP sequence spaces: 6 bytes in flight, 4 after acknowledgement");
}
