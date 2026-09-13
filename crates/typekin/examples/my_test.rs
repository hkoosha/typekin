mod calc {
    #![allow(dead_code)]

    trait MyToString {
        fn my_to_string(&self) -> String;
    }

    impl<T: Debug> MyToString for T {
        fn my_to_string(&self) -> String {
            let it = format!("{self:?}");
            let it = it.split("(").nth(1).unwrap();
            let it = it.split(")").nth(0).unwrap();
            return it.to_string();
        }
    }

    use std::fmt::Formatter;
    use std::fmt::{Debug, Display};
    use std::ops::Add;
    use std::ops::BitAnd;
    use std::ops::BitOr;
    use std::ops::BitXor;
    use std::ops::Div;
    use std::ops::Mul;
    use std::ops::Not;
    use std::ops::Rem;
    use std::ops::Sub;

    macro_rules! calc {
    (
        $conv:expr,
        $p:tt $op:tt $q:tt,
        $pr:tt $op_r:tt $qr:tt
        $(,)?
    ) => {
        {
            debug_assert_eq!(
                stringify!($op),
                stringify!($op_r),
            );
            Produced::bin(
                $p,
                stringify!($op),
                $q,
                $p $op $q,
                $pr $op_r $qr,
                (($conv)($p $op $q)).my_to_string() == ($pr $op_r $qr).my_to_string(),
            )
        }
    };
}

    trait Any: Sized + Debug + Copy + Clone + Eq + Ord {}

    impl<T> Any for T where T: Sized + Debug + Copy + Clone + Eq + Ord {}

    trait Math<Q, R = Self>
    where
        Q: Any,
        Self: Any
            + Add<Q, Output = R>
            + Sub<Q, Output = R>
            + Mul<Q, Output = R>
            + Div<Q, Output = R>
            + Rem<Q, Output = R>,
    {
    }

    impl<T, Q, R> Math<Q, R> for T
    where
        Q: Any,
        T: Any
            + Add<Q, Output = R>
            + Sub<Q, Output = R>
            + Mul<Q, Output = R>
            + Div<Q, Output = R>
            + Rem<Q, Output = R>,
    {
    }

    trait Bit<Q, R = Self>
    where
        Q: Any,
        Self: Any
            + BitAnd<Q, Output = R>
            + BitOr<Q, Output = R>
            + BitXor<Q, Output = R>
            + Not<Output = R>,
    {
    }

    impl<T, Q, R> Bit<Q, R> for T
    where
        Q: Any,
        T: Any
            + BitAnd<Q, Output = R>
            + BitOr<Q, Output = R>
            + BitXor<Q, Output = R>
            + Not<Output = R>,
    {
    }

    #[derive(Debug, Clone)]
    struct Op<P: Any, Q: Any> {
        op: String,
        p: P,
        q: Q,
    }

    impl<P: Any, Q: Any> Op<P, Q> {
        #[must_use]
        fn bin(
            p: P,
            op: impl AsRef<str>,
            q: Q,
        ) -> Self {
            return Self {
                op: op.as_ref().my_to_string(),
                p,
                q,
            };
        }
    }

    impl<P: Any, Q: Any> Display for Op<P, Q> {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            let p = self.p.my_to_string();
            let q = self.q.my_to_string();
            write!(f, "{} {} {}", p, self.op, q)
        }
    }

    #[derive(Debug, Clone)]
    struct Produced<P: Any, Q: Any, R: Any, RR: Any> {
        op: Op<P, Q>,
        actual: R,
        expected: RR,
        is_ok: bool,
    }

    impl<P: Any, Q: Any, R: Any, RR: Any> Produced<P, Q, R, RR> {
        fn bin(
            p: P,
            op: impl AsRef<str>,
            q: Q,
            actual: R,
            expected: RR,
            ok: bool,
        ) -> Self {
            return Self {
                op: Op::bin(p, op, q),
                actual,
                expected,
                is_ok: ok,
            };
        }
    }

    impl<P: Any, Q: Any, R: Any, RR: Any> Display for Produced<P, Q, R, RR> {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            let actual = self.actual.my_to_string();
            let expected = self.expected.my_to_string();
            write!(
                f,
                "Produced{}[{}] {} <=> {}, ok={}{}",
                '{', self.op, actual, expected, self.is_ok, '}'
            )
        }
    }

    #[derive(Debug, Clone)]
    struct Demo<P, Q, R, RR, C>
    where
        P: Any,
        Q: Any,
        R: Any,
        RR: Any,
        C: Copy + Fn(P) -> RR,
    {
        is_ok: bool,
        conv: C,
        p: P,
        q: Q,
        pr: RR,
        qr: RR,
        results: Vec<Produced<P, Q, R, RR>>,
    }

    impl<P, Q, RR, C> Demo<P, Q, P, RR, C>
    where
        P: Any,
        Q: Any,
        RR: Any,
        C: Copy + Fn(P) -> RR,
    {
        #[must_use]
        fn bin(
            p: P,
            q: Q,
            pr: RR,
            qr: RR,
            conv: C,
        ) -> Self
        where
            P: Math<Q, P>,
            RR: Math<RR>,
        {
            let results = vec![
                calc!(conv, p - q, pr - qr),
                calc!(conv, p * q, pr * qr),
                calc!(conv, p / q, pr / qr),
                calc!(conv, p % q, pr % qr),
            ];
            return Self {
                is_ok: results.iter().all(|it| it.is_ok),
                conv,
                p,
                q,
                pr,
                qr,
                results,
            };
        }

        #[must_use]
        fn bit(
            p: P,
            q: Q,
            pr: RR,
            qr: RR,
            conv: C,
        ) -> Self
        where
            P: Bit<Q, P>,
            RR: Bit<RR>,
        {
            let results = vec![
                calc!(conv, p & q, pr & qr),
                calc!(conv, p | q, pr | qr),
                calc!(conv, p ^ q, pr ^ qr),
            ];

            return Self {
                is_ok: results.iter().all(|it| it.is_ok),
                conv,
                p,
                q,
                pr,
                qr,
                results,
            };
        }

        #[must_use]
        fn all(
            p: P,
            q: Q,
            pr: RR,
            qr: RR,
            conv: C,
        ) -> Self
        where
            P: Math<Q, P> + Bit<Q, P>,
            RR: Math<RR> + Bit<RR>,
        {
            let bin = Self::bin(p, q, pr, qr, conv);
            let bit = Self::bit(p, q, pr, qr, conv);

            let mut results = bin.results;
            results.extend(bit.results);

            return Self {
                is_ok: bit.is_ok && bit.is_ok,
                conv,
                p,
                q,
                pr,
                qr,
                results,
            };
        }

        #[must_use]
        pub fn print(&self) -> String {
            let mut text = String::new();

            let lhs_max = self
                .results
                .iter()
                .map(|it| (self.conv)(it.op.p))
                .map(|it| it.my_to_string())
                .max_by_key(|it| it.len())
                .map(|it| it.len())
                .unwrap_or(0);

            let rhs_max = self
                .results
                .iter()
                .map(|it| it.op.q.my_to_string())
                .max_by_key(|it| it.len())
                .map(|it| it.len())
                .unwrap_or(0);

            let op_max = self
                .results
                .iter()
                .map(|it| it.op.op.len())
                .max()
                .unwrap_or(0);

            let expected_max = self
                .results
                .iter()
                .map(|it| it.expected.my_to_string().len())
                .max()
                .unwrap_or(0);

            let actual_max = self
                .results
                .iter()
                .map(|it| (self.conv)(it.actual))
                .map(|it| it.my_to_string().len())
                .max()
                .unwrap_or(0);

            for it in &self.results {
                let (marker, eq, expected) = match it.is_ok {
                    true => ("", "", String::new()),
                    _ => (">", " != ", it.expected.my_to_string()),
                };

                let line = format!(
                    "{marker:<2} {:<lhs_max$} {:<op_max$} {:<rhs_max$} = {:<actual_max$} {} {:<expected_max$}",
                    (self.conv)(it.op.p).my_to_string(),
                    it.op.op,
                    it.op.q.my_to_string(),
                    (self.conv)(it.actual).my_to_string(),
                    eq,
                    expected,
                    lhs_max = lhs_max,
                    rhs_max = rhs_max,
                    op_max = op_max,
                    expected_max = expected_max,
                    actual_max = actual_max,
                );
                text.push_str(line.as_str());
                text.push('\n');
            }

            let _ = text.pop();

            return text;
        }
    }

    impl<P, Q, R, RR, C> Display for Demo<P, Q, R, RR, C>
    where
        P: Any,
        Q: Any,
        R: Any,
        RR: Any,
        C: Copy + Fn(P) -> RR,
    {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            for it in &self.results {
                writeln!(f, "{}", it)?;
            }

            Ok(())
        }
    }

    #[allow(private_interfaces)]
    #[allow(private_bounds)]
    pub fn demo_u128<P, Q, C>(
        p: P,
        q: Q,
        conv: C,
    ) -> Demo<P, Q, P, u128, C>
    where
        P: Any + Math<Q, P> + Bit<Q, P> + Into<u128>,
        Q: Any + TryInto<u128>,
        C: Copy + Fn(P) -> u128,
    {
        let pr: u128 = p.into();
        let qr: u128 = match q.try_into() {
            Ok(it) => it,
            Err(_) => panic!("bad q, cannot convert into u128: {:?}", q),
        };

        let math = Demo::all(p, q, pr, qr, conv);
        return math;
    }

    #[allow(private_interfaces)]
    #[allow(private_bounds)]
    pub fn demo_i128<P, Q, C>(
        p: P,
        q: Q,
        conv: C,
    ) -> Demo<P, Q, P, i128, C>
    where
        P: Any + Math<Q, P> + Bit<Q, P> + Into<i128>,
        Q: Any + TryInto<i128>,
        C: Copy + Fn(P) -> i128,
    {
        let pr: i128 = p.into();
        let qr: i128 = match q.try_into() {
            Ok(it) => it,
            Err(_) => panic!("bad q, cannot convert into i128: {:?}", q),
        };

        let math = Demo::all(p, q, pr, qr, conv);
        return math;
    }

    #[test]
    fn test_u128() {
        assert!(Demo::all(5u32, 2u32, 5u32, 2u32, |it| it).is_ok)
    }

    #[test]
    fn test_i128() {
        assert!(Demo::all(5i32, 2i32, 5i32, 2i32, |it| it).is_ok)
    }
}

#[cfg(test)]
mod test_u8 {
    #[typekin::integral(friends = [u8(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(u8);

    #[test]
    fn test() {
        let lhs = 0b1101u8;
        let rhs = 0b0110u8;

        let demo = super::calc::demo_u128(Subject::of(lhs), rhs, |it| {
            it.raw() as u128
        });
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_u16 {
    #[typekin::integral(friends = [u16(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(u16);

    #[test]
    fn test() {
        let lhs = 0b1101u16;
        let rhs = 0b0110u16;

        let demo = super::calc::demo_u128(Subject::of(lhs), rhs, |it| {
            it.raw() as u128
        });
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_u32 {
    #[typekin::integral(friends = [u32(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(u32);

    #[test]
    fn test() {
        let lhs = 0b1101u32;
        let rhs = 0b0110u32;

        let demo = super::calc::demo_u128(Subject::of(lhs), rhs, |it| {
            it.raw() as u128
        });
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_u64 {
    #[typekin::integral(friends = [u64(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(u64);

    #[test]
    fn test() {
        let lhs = 0b1101u64;
        let rhs = 0b0110u64;

        let demo = super::calc::demo_u128(Subject::of(lhs), rhs, |it| {
            it.raw() as u128
        });
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_u128 {
    #[typekin::integral(friends = [u128(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(u128);

    #[test]
    fn test() {
        let lhs = 0b1101u128;
        let rhs = 0b0110u128;

        let demo = super::calc::demo_u128(Subject::of(lhs), rhs, |it| it.raw());
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_i8 {
    #[typekin::integral(friends = [i8(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(i8);

    #[test]
    fn test() {
        let lhs = 0b1101i8;
        let rhs = 0b0110i8;

        let demo = super::calc::demo_i128(Subject::of(lhs), rhs, |it| {
            it.raw() as i128
        });
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_i16 {
    #[typekin::integral(friends = [i16(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(i16);

    #[test]
    fn test() {
        let lhs = 0b1101i16;
        let rhs = 0b0110i16;

        let demo = super::calc::demo_i128(Subject::of(lhs), rhs, |it| {
            it.raw() as i128
        });
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_i32 {
    #[typekin::integral(friends = [i32(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(i32);

    #[test]
    fn test() {
        let lhs = 0b1101i32;
        let rhs = 0b0110i32;

        let demo = super::calc::demo_i128(Subject::of(lhs), rhs, |it| {
            it.raw() as i128
        });
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_i64 {
    #[typekin::integral(friends = [i64(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(i64);

    #[test]
    fn test() {
        let lhs = 0b1101i64;
        let rhs = 0b0110i64;

        let demo = super::calc::demo_i128(Subject::of(lhs), rhs, |it| {
            it.raw() as i128
        });
        println!("{}", demo.print());
    }
}

#[cfg(test)]
mod test_i128 {
    #[typekin::integral(friends = [i128(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    struct Subject(i128);

    #[test]
    fn test() {
        let lhs = 0b1101i128;
        let rhs = 0b0110i128;

        let demo = super::calc::demo_i128(Subject::of(lhs), rhs, |it| it.raw());
        println!("{}", demo.print());
    }
}

fn main() {
    panic!("do not run, instead run tests.")
}
