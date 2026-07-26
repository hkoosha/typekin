#![allow(dead_code)]

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not};

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
                (($conv)($p $op $q)).to_string() == ($pr $op_r $qr).to_string(),
            )
        }
    };
}

pub trait Any: Sized + Debug + Display + Copy + Clone + Eq + Ord {}

impl<T> Any for T where T: Sized + Debug + Display + Copy + Clone + Eq + Ord {}

pub trait Math<Q, R = Self>
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

pub trait Bit<Q, R = Self>
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
pub struct Op<P: Any, Q: Any> {
    pub op: String,
    pub p: P,
    pub q: Q,
}

impl<P: Any, Q: Any> Op<P, Q> {
    #[must_use]
    pub fn bin(
        p: P,
        op: impl AsRef<str>,
        q: Q,
    ) -> Self {
        return Self {
            op: op.as_ref().to_string(),
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
        write!(f, "{} {} {}", self.p, self.op, self.q)
    }
}

#[derive(Debug, Clone)]
pub struct Produced<P: Any, Q: Any, R: Any, RR: Any> {
    pub op: Op<P, Q>,
    pub actual: R,
    pub expected: RR,
    pub is_ok: bool,
}

impl<P: Any, Q: Any, R: Any, RR: Any> Produced<P, Q, R, RR> {
    pub fn bin(
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
        write!(
            f,
            "Produced{}[{}] {} <=> {}, ok={}{}",
            '{', self.op, self.actual, self.expected, self.is_ok, '}'
        )
    }
}

#[derive(Debug, Clone)]
pub struct Demo<P, Q, R, RR, C>
where
    P: Any,
    Q: Any,
    R: Any,
    RR: Any,
    C: Copy + Fn(P) -> RR,
{
    pub is_ok: bool,
    pub conv: C,
    pub p: P,
    pub q: Q,
    pub pr: RR,
    pub qr: RR,
    pub results: Vec<Produced<P, Q, R, RR>>,
}

impl<P, Q, RR, C> Demo<P, Q, P, RR, C>
where
    P: Any,
    Q: Any,
    RR: Any,
    C: Copy + Fn(P) -> RR,
{
    #[must_use]
    pub fn bin(
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
    pub fn bit(
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
    pub fn all(
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
            .map(|it| it.to_string())
            .max_by_key(|it| it.len())
            .map(|it| it.len())
            .unwrap_or(0);

        let rhs_max = self
            .results
            .iter()
            .map(|it| it.op.q.to_string())
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
            .map(|it| it.expected.to_string().len())
            .max()
            .unwrap_or(0);

        let actual_max = self
            .results
            .iter()
            .map(|it| (self.conv)(it.actual))
            .map(|it| it.to_string().len())
            .max()
            .unwrap_or(0);

        for it in &self.results {
            let (marker, eq, expected) = match it.is_ok {
                true => ("", "", String::new()),
                _ => (">", " != ", it.expected.to_string()),
            };

            let line = format!(
                "{marker:<2} {:<lhs_max$} {:<op_max$} {:<rhs_max$} = {:<actual_max$} {} {:<expected_max$}",
                (self.conv)(it.op.p),
                it.op.op,
                it.op.q,
                (self.conv)(it.actual),
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

pub fn demo_i128<P, Q, R, RR, C, CC>(
    p: P,
    q: Q,
    conv: C,
) -> Demo<P, Q, P, i128, C>
where
    P: Any + Math<Q, P> + Bit<Q, P> + Into<i128>,
    Q: Any + TryInto<i128>,
    R: Any,
    RR: Any,
    C: Copy + Fn(P) -> i128,
{
    let pr: i128 = p.into();
    let qr: i128 = match q.try_into() {
        Ok(it) => it,
        Err(_) => panic!("bad q, cannot convert into i128: {}", q),
    };

    let math = Demo::all(p, q, pr, qr, conv);
    return math;
}

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
        Err(_) => panic!("bad q, cannot convert into u128: {}", q),
    };

    let math = Demo::all(p, q, pr, qr, conv);
    return math;
}

#[cfg(test)]
#[test]
fn test() {
    let demo = Demo::all(5u32, 2u32, 5u32, 2u32, |it| it);

    eprintln!("{}", demo.print());

    assert!(demo.is_ok)
}
