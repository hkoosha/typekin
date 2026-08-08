use crate::value_type::N;
use proc_macro2::TokenStream;
use std::str::FromStr;

pub(crate) enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    Xor,
    And,
    Or_,

    Shr,
    Shl,
}

impl BinOp {
    pub(crate) fn repr(&self) -> (&'static str, &'static str) {
        return match self {
            Self::Add => ("add", "+"),
            Self::Sub => ("sub", "-"),
            Self::Mul => ("mul", "*"),
            Self::Div => ("div", "/"),
            Self::Rem => ("rem", "%"),
            Self::Xor => ("xor", "^"),
            Self::And => ("and", "&"),
            Self::Or_ => ("or", "|"),
            Self::Shr => ("shr", ">>"),
            Self::Shl => ("shl", "<<"),
        };
    }

    pub(crate) fn stream(&self) -> TokenStream {
        return TokenStream::from_str(self.repr().1)
            .expect("invalid operation");
    }

    pub(crate) fn name(&self) -> &'static str {
        return self.repr().0;
    }

    pub(crate) fn predefined_arg_size(&self) -> Option<N> {
        return match self {
            Self::Shl => Some(N::USIZ),
            Self::Shr => Some(N::USIZ),
            _ => None,
        };
    }
}

#[derive(Copy, Clone)]
pub(crate) enum UnaryOp {
    Not,
}

impl UnaryOp {
    pub(crate) fn stream(self) -> TokenStream {
        return TokenStream::from_str(self.repr().1)
            .expect("invalid operation");
    }

    pub(crate) fn name(&self) -> &'static str {
        return self.repr().0;
    }

    pub(crate) fn repr(self) -> (&'static str, &'static str) {
        return match self {
            Self::Not => ("not", "!"),
        };
    }
}
