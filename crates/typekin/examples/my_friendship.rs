mod folks {
    use std::fmt::{
        Display,
        Formatter,
    };

    const PRETTY: u64 =
        0b1000000000000000000000000000000000000000000000000000000000000000u64;

    #[derive(Debug, Clone, Copy)]
    pub struct Writer {
        pub id: u32,
    }

    impl Display for Writer {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "Writer[{:b}]", self.id)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ExecXX {
        pub id: u32,
    }

    impl Display for ExecXX {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "ExecXX[{:b}]", self.id)
        }
    }

    pub(super) fn executor_parts(it: &ExecXX) -> (u64, u8) {
        return (PRETTY | ((it.id as u64) << 32), 0b10_100);
    }
}

mod sample {
    use super::folks::ExecXX;
    use super::folks::Writer;
    use std::fmt::{
        Display,
        Formatter,
    };
    use std::ops::BitOr;

    const PRETTY: u64 =
        0b1000000000000000000000000000000000000000000000000000000000000000u64;

    #[typekin::friendship(
        relation = (u64, u8),
        friends = [
            _(conv = x, cap = [Haha]),
            Self(conv = Document::convert_me, cap = []),
            Reader(conv = Reader::parts, cap = [Make, Inspector]),
            ExecXX(conv = crate::folks::executor_parts, cap = []),
            Writer(conv = writer_parts, cap = [Make, Inspector]),
        ],
        mod = things,
    )]
    #[typekin::constructor(
        of_relation = Self::of_parts,
        friends = [ExecXX, Self]
    )]
    #[derive(Debug, Clone, Copy)]
    pub struct Document {
        #[allow(unused, dead_code)]
        pub(super) id: u64,
        #[allow(unused, dead_code)]
        pub(super) access: u8,
    }

    impl Default for Document {
        fn default() -> Self {
            return Self {
                id: PRETTY,
                access: 0b10000000u8,
            };
        }
    }

    impl Display for Document {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "Document[{:b}-{:b}]", self.id, self.access)
        }
    }

    impl Document {
        fn of_parts(it: (u64, u8)) -> Self {
            return Self {
                id: it.0,
                access: it.1,
            };
        }

        fn convert_me(it: &Self) -> (u64, u8) {
            return (it.id, it.access);
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Reader {
        pub id: u16,
    }

    impl Display for Reader {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "Reader[{:b}]", self.id)
        }
    }

    impl Reader {
        fn parts(&self) -> (u64, u8) {
            return (PRETTY | ((self.id as u64) << 16), 0b10_001);
        }
    }

    fn writer_parts(it: &Writer) -> (u64, u8) {
        return (PRETTY | it.id as u64, 0b10_010);
    }

    impl<T> BitOr<T> for Document
    where
        T: things::Make,
    {
        type Output = Self;

        fn bitor(
            self,
            rhs: T,
        ) -> Self::Output {
            let (id, access) = rhs.convert();
            return Self {
                id: self.id | id,
                access: self.access | access,
            };
        }
    }
}

fn main() {
    use folks::*;
    use sample::*;

    let reader = Reader {
        id: 0b10_100_10_00_01_01,
    };
    let writer = Writer {
        id: 0b10_010_10_10_00_01,
    };
    let exec_x = ExecXX {
        id: 0b10_001_00_10_01_01,
    };

    let r_doc = Document::of(reader);
    let w_doc = Document::of(writer);
    let e_doc = Document::of(exec_x);

    println!("{} => {}", reader, r_doc);
    println!("{} => {}", writer, w_doc);
    println!("{} => {}", exec_x, e_doc);

    println!(
        "{} &\n{} &\n{} =>\n{} &\n{} &\n{} =>\n{}",
        reader,
        writer,
        exec_x,
        r_doc,
        w_doc,
        e_doc,
        Document::default() | r_doc | w_doc | e_doc
    );
}
