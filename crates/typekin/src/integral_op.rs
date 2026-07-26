use crate::integral_types::N;
use proc_macro2::Punct;
use proc_macro2::Spacing;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

pub(crate) fn stream_op(repr: &str) -> TokenStream {
    let mut stream = TokenStream::new();

    let last = repr.len() - 1;
    for (idx, it) in repr.chars().enumerate() {
        let punct = Punct::new(
            it,
            if idx == last {
                Spacing::Alone
            }
            else {
                Spacing::Joint
            },
        );
        let tree = TokenTree::Punct(punct);
        let chunk = TokenStream::from(tree);
        stream.extend(chunk);
    }

    return stream;
}

// =============================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum NumBinOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    Xor,
    And,
    Or,

    Shr,
    Shl,
}

impl NumBinOperator {
    pub(crate) fn stream(self) -> TokenStream {
        return stream_op(self.repr());
    }

    pub(crate) fn repr(self) -> &'static str {
        match self {
            NumBinOperator::Add => "+",
            NumBinOperator::Sub => "-",
            NumBinOperator::Mul => "*",
            NumBinOperator::Div => "/",
            NumBinOperator::Rem => "%",
            NumBinOperator::Xor => "^",
            NumBinOperator::And => "&",
            NumBinOperator::Or => "|",
            NumBinOperator::Shr => ">>",
            NumBinOperator::Shl => "<<",
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            NumBinOperator::Add => "add",
            NumBinOperator::Sub => "sub",
            NumBinOperator::Mul => "mul",
            NumBinOperator::Div => "div",
            NumBinOperator::Rem => "rem",
            NumBinOperator::Xor => "xor",
            NumBinOperator::And => "and",
            NumBinOperator::Or => "or",
            NumBinOperator::Shr => "shr",
            NumBinOperator::Shl => "shl",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum NumBinArg {
    Variable,
    Predefined(N),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NumBinOp {
    pub(crate) op: NumBinOperator,
    pub(crate) arg: NumBinArg,
}

impl NumBinOp {
    pub(crate) fn variable(op: NumBinOperator) -> Self {
        return Self {
            op,
            arg: NumBinArg::Variable,
        };
    }

    pub(crate) fn usized(op: NumBinOperator) -> Self {
        return Self {
            op,
            arg: NumBinArg::Predefined(N::USIZ),
        };
    }
}

// =============================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum NumUnaryPrefixOperator {
    Not,
}

impl NumUnaryPrefixOperator {
    pub(crate) fn stream(self) -> TokenStream {
        return stream_op(self.repr());
    }

    pub(crate) fn repr(self) -> &'static str {
        match self {
            NumUnaryPrefixOperator::Not => "!",
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            NumUnaryPrefixOperator::Not => "not",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NumUnaryPrefixOp {
    pub(crate) op: NumUnaryPrefixOperator,
}

impl NumUnaryPrefixOp {
    pub(crate) fn new(op: NumUnaryPrefixOperator) -> Self {
        return Self { op };
    }
}

// =============================================================================

pub(crate) struct Op;

impl Op {
    pub(crate) fn add() -> NumBinOp {
        return NumBinOp::variable(NumBinOperator::Add);
    }

    pub(crate) fn sub() -> NumBinOp {
        return NumBinOp::variable(NumBinOperator::Sub);
    }

    pub(crate) fn mul() -> NumBinOp {
        return NumBinOp::variable(NumBinOperator::Mul);
    }

    pub(crate) fn div() -> NumBinOp {
        return NumBinOp::variable(NumBinOperator::Div);
    }

    pub(crate) fn rem() -> NumBinOp {
        return NumBinOp::variable(NumBinOperator::Rem);
    }

    pub(crate) fn xor() -> NumBinOp {
        return NumBinOp::variable(NumBinOperator::Xor);
    }

    pub(crate) fn and() -> NumBinOp {
        return NumBinOp::variable(NumBinOperator::And);
    }

    pub(crate) fn or() -> NumBinOp {
        return NumBinOp::variable(NumBinOperator::Or);
    }

    pub(crate) fn shr() -> NumBinOp {
        return NumBinOp::usized(NumBinOperator::Shr);
    }

    pub(crate) fn shl() -> NumBinOp {
        return NumBinOp::usized(NumBinOperator::Shl);
    }

    pub(crate) fn not() -> NumUnaryPrefixOp {
        return NumUnaryPrefixOp::new(NumUnaryPrefixOperator::Not);
    }
}
