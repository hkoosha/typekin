//! The `domain` expression language (integral and float subjects).
//!
//! A domain is a set of values of the field's type, written as a set
//! expression: `domain = (1..12 | 87..) & !{10} & !mul(3)`.
//!
//! Parsing is syntax only; the value type is known later, in `resolve`, which
//! types constants, folds them per pointer width, normalizes (never adding
//! comparisons), probes zero exactly, and emits one expression or a
//! per-width `cfg` block. Integers resolve in `zz_domain_int`, floats in
//! `zz_domain_float`.
//!
//! # Grammar
//!
//! EBNF over Rust tokens. This grammar is the source of truth; the parser below
//! implements it rule by rule.
//!
//! ```text
//! domain        = union ;
//! union         = { closed_inter , "|" } , inter ;
//! inter         = { closed , "&" } , unary ;
//! closed_inter  = closed , { "&" , closed } ;
//! unary         = closed | image ;
//! closed        = "!" , closed | primary ;
//! primary       = range | roster | multiples | predicate
//!               | "(" , union , ")" ;
//! image         = lens , [ "!" ] , "in" , value_unary ;
//!
//! value_union   = value_inter , { "|" , value_inter } ;
//! value_inter   = value_unary , { "&" , value_unary } ;
//! value_unary   = "!" , value_unary | value_primary ;
//! value_primary = range | roster | multiples | "(" , value_union , ")" ;
//!
//! range         = [ constant ] , ".." , [ constant ]
//!               | [ constant ] , "..=" , constant ;
//! roster        = "{" , constant , { "," , constant } , [ "," ] , "}" ;
//! multiples     = "mul" , "(" , constant , ")" ;
//! lens          = name , [ "(" , constant , { "," , constant } , ")" ] ;
//! predicate     = name ;
//! name          = IDENT - ( "mul" | "MIN" | "MAX" ) ;
//! constant      = [ "-" ] , ( INTEGER | FLOAT | int_bound | float_const ) ;
//! int_bound     = INT_TYPE , "::" , ( "MIN" | "MAX" ) ;
//! float_const   = FLOAT_TYPE , "::" , IDENT - "NAN"
//!               | [ ( "core" | "std" ) , "::" ] , FLOAT_TYPE , "::" ,
//!                 "consts" , "::" , IDENT ;
//! ```
//!
//! - `|` is union, `&` intersection, and `!` complement within the type (NaN
//!   included for floats). `!` applies to the unary right after it, and `&`
//!   binds tighter than `|`, as in Rust; that is the grammar's only precedence
//!   rule.
//! - `range` is a Rust range over the type, `roster` its listed values, and
//!   `multiples` the values `v` with `v % c == 0`; none of them holds NaN.
//! - `predicate` holds the values whose method returns true
//!   (`is_power_of_two`, `is_nan`); `image` holds the values whose lens result
//!   (`abs`, `rem_euclid(16)`) lies in `value_unary`, a set of lens results,
//!   and with `!in` the values whose lens result does not.
//! - A `closed` unary does not end in an image. Only it may precede `|` or `&`,
//!   which could extend either the image's values or the surrounding set, and
//!   only it may follow `!`, which would read as complementing the lens.
//! - INTEGER and FLOAT are Rust literals; an INTEGER suffix must name the
//!   operand type. INT_TYPE and FLOAT_TYPE are primitive type names. Names are
//!   checked against the whitelist once the value type is known.
//!
//! # In plain words
//!
//! An unofficial summary for reading and writing domains. It may simplify,
//! omit, or fall behind the EBNF above, which decides every disagreement.
//!
//! A domain is the set of values a wrapper accepts, and you write that set
//! directly:
//!
//! - `1..12`, `87..`, `..=100`: Rust ranges.
//! - `{10}`, `{1, 3, 7}`: exactly these values.
//! - `mul(3)`: the multiples of 3.
//! - `is_power_of_two`, `is_nan`: the values for which that method returns
//!   `true`.
//! - `abs in ..=1000`: the values whose `abs()` lies in `..=1000`, and
//!   `abs !in ..=1000` the values whose `abs()` does not. After `in` comes a
//!   set of the function's results, built like any other set but without
//!   functions: `count_ones in {1}`, `rem_euclid(16) in {3, 5}`,
//!   `abs in (..5 | 10..)`.
//!
//! Sets combine like Rust's `HashSet` operators:
//!
//! - `!a` is every value of the type that is not in `a`;
//! - `a & b` holds the values in both;
//! - `a | b` holds the values in either.
//!
//! So `(1..12 | 87..) & !{10} & !mul(3)` accepts 1 to 11 and 87 upward, except
//! 10 and the multiples of 3. One precedence rule applies: `&` binds tighter
//! than `|`, as in Rust and in logic, so `..5 | 10.. & ..20` is
//! `..5 | (10.. & ..20)`. Wherever else two readings are possible, the domain is
//! rejected with a request for parentheses instead of picking one: an `in` set
//! followed by `|` or `&`. `abs in ..5 | 10..` could mean "`abs` below 5 or
//! from 10" or "`abs` below 5, or any value from 10"; write
//! `abs in (..5 | 10..)` or `(abs in ..5) | 10..`.
//!
//! `!` takes the set right after it, so `!1..5`, `!{0} & 5..`, and
//! `{0} | abs in 5..10` need no parentheses. A function's set is complemented
//! with `!in`: `abs !in ..3`. A leading `!`, as in `!abs in ..3`, would read as
//! complementing `abs` and is rejected.
//!
//! Constants are literals, the type's `MIN`/`MAX` (`u32::MAX`), and float
//! constants such as `f64::INFINITY` or `f64::consts::PI`. They are never
//! computed: `1..10 - 2` is an error, not `1..8`. A lone value is not a set;
//! write `{5}`, not `5`.
//!
//! For floats, ranges, `{…}`, and `mul(…)` never contain NaN, while
//! complements do: `!{0.5}` accepts NaN, `0.0..` rejects it. There is no `>`;
//! `0.0.. & !{0.0}` reads "positive" and compiles to the single comparison
//! `> 0.0`.

use std::collections::BTreeMap;

use proc_macro2::{
    Delimiter,
    Ident,
    Span,
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    quote,
};
use syn::buffer::Cursor;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::{
    LitFloat,
    LitInt,
    Token,
    token,
};

use crate::zz_lib::Named as _;
use crate::zz_runner::MkErr;
use crate::zz_values::{
    F,
    FnName,
    Int,
    N,
};

/// The integer bound constants a domain may name (`u32::MAX`).
pub(crate) const MIN: &str = "MIN";
pub(crate) const MAX: &str = "MAX";
/// The multiples set, `mul(3)`.
const MUL: &str = "mul";

// =============================================================================
// Syntax

/// A parsed, untyped domain expression.
#[derive(Clone)]
pub(crate) struct Domain {
    pub(crate) root: Node,
    pub(crate) span: Span,
    /// The expression as written, for generated documentation.
    text: String,
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Node {
    Or(Vec<Node>),
    And(Vec<Node>),
    Not(Box<Node>),
    Atom(Atom),
}

#[derive(Clone)]
pub(crate) struct Atom {
    pub(crate) lens: Option<Lens>,
    pub(crate) test: Test,
    pub(crate) span: Span,
}

#[derive(Clone)]
pub(crate) struct Lens {
    pub(crate) name: Ident,
    /// `name` as a known function; `None` is reported when typed.
    pub(crate) function: Option<FnName>,
    pub(crate) args: Vec<Const>,
}

#[derive(Clone)]
pub(crate) enum Test {
    Range {
        lo: Option<Const>,
        hi: Option<Const>,
        inclusive: bool,
    },
    /// `{c}`, or its complement `!{c}`.
    Point {
        value: Const,
        negated: bool,
    },
    /// `mul(c)`, or its complement `!mul(c)`.
    Divisible {
        divisor: Const,
        negated: bool,
    },
    Predicate,
}

#[derive(Clone)]
pub(crate) struct Const {
    pub(crate) tokens: TokenStream,
    pub(crate) span: Span,
    pub(crate) kind: ConstKind,
}

#[derive(Clone)]
pub(crate) enum ConstKind {
    Int {
        value: Int,
        suffix: String,
    },
    Bound {
        ty: N,
        max: bool,
        negated: bool,
    },
    Float {
        digits: String,
        suffix: String,
        negated: bool,
    },
    /// `f64::MAX`, `f64::INFINITY`, `core::f64::consts::PI`, …
    FloatPath {
        ty: F,
        name: String,
        consts: bool,
        negated: bool,
    },
}

/// Where a set expression stands: the domain, or the values after `in`, which
/// hold no lens or predicate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Scope {
    Domain,
    Values,
}

/// The binary set operators; `&` binds tighter than `|`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SetOp {
    Union,
    Inter,
}

impl SetOp {
    /// `|` or `&`, but not `||` or `&&`.
    fn peek(input: ParseStream) -> Option<Self> {
        return if input.peek(Token![|]) && !input.peek(Token![||]) {
            Some(Self::Union)
        }
        else if input.peek(Token![&]) && !input.peek(Token![&&]) {
            Some(Self::Inter)
        }
        else {
            None
        };
    }

    fn parse(
        self,
        input: ParseStream,
    ) -> syn::Result<()> {
        match self {
            Self::Union => drop(input.parse::<Token![|]>()?),
            Self::Inter => drop(input.parse::<Token![&]>()?),
        }
        return Ok(());
    }

    fn symbol(self) -> &'static str {
        return match self {
            Self::Union => "|",
            Self::Inter => "&",
        };
    }
}

impl Parse for Domain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let begin = input.cursor();
        let root = parse_union(input, Scope::Domain)?;
        if !input.is_empty() && !input.peek(Token![,]) {
            return Err(stray(input));
        }
        let text = spelled(begin, input.cursor());
        return Ok(Self { root, span, text });
    }
}

/// The tokens from `begin` up to `end`, a later cursor of the same buffer.
fn tokens_between(
    begin: Cursor,
    end: Cursor,
) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut cursor = begin;
    while cursor != end {
        let Some((token, next)) = cursor.token_tree()
        else {
            break;
        };
        tokens.extend([token]);
        cursor = next;
    }
    return tokens;
}

/// The tokens from `begin` up to `end`, spelled like the doc text.
fn spelled(
    begin: Cursor,
    end: Cursor,
) -> String {
    let mut text = String::new();
    spell(tokens_between(begin, end), &mut text);
    return text;
}

impl Domain {
    /// The expression as written.
    pub(crate) fn text(&self) -> &str {
        return &self.text;
    }
}

/// The written tokens, spaced like `(1..12 | 87..) & !{10}`, `{1, 3}`, and
/// `abs in ..=1000`.
fn spell(
    tokens: TokenStream,
    text: &mut String,
) {
    let mut tokens = tokens.into_iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Group(group) => {
                let (open, close) = match group.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::None => ("", ""),
                };
                text.push_str(open);
                spell(group.stream(), text);
                text.push_str(close);
            }
            TokenTree::Punct(punct) => match punct.as_char() {
                '|' => text.push_str(" | "),
                '&' => text.push_str(" & "),
                ',' if tokens.peek().is_some() => text.push_str(", "),
                // `in` is the grammar's only keyword.
                '!' if matches!(
                    tokens.peek(),
                    Some(TokenTree::Ident(it)) if it == "in"
                ) =>
                {
                    tokens.next();
                    text.push_str(" !in ");
                }
                other => text.push(other),
            },
            TokenTree::Ident(ident) if ident == "in" => text.push_str(" in "),
            other => text.push_str(&other.to_string()),
        }
    }
}

fn stray(input: ParseStream) -> syn::Error {
    let span = input.span();
    let message = if input.peek(Token![+])
        || input.peek(Token![-])
        || input.peek(Token![*])
        || input.peek(Token![/])
    {
        "domains have no arithmetic; combine sets with |, &, and !"
    }
    else {
        "expected | or & between domain sets"
    };
    return syn::Error::new(span, message);
}

/// A name, its optional arguments, and `in` or `!in`.
fn starts_image(input: ParseStream) -> bool {
    let fork = input.fork();
    let Ok(name) = fork.parse::<Ident>()
    else {
        return false;
    };
    if name == MUL || fork.peek(Token![::]) {
        return false;
    }
    if fork.peek(token::Paren) && fork.parse::<TokenTree>().is_err() {
        return false;
    }
    return fork.peek(Token![in])
        || (fork.peek(Token![!]) && fork.peek2(Token![in]));
}

/// Applies `lens` to every atom of a values-scope set: the image of a union,
/// intersection, or complement is the union, intersection, or complement of
/// the images.
fn with_lens(
    node: Node,
    lens: &Lens,
) -> Node {
    return match node {
        Node::Or(items) => {
            Node::Or(items.into_iter().map(|it| with_lens(it, lens)).collect())
        }
        Node::And(items) => {
            Node::And(items.into_iter().map(|it| with_lens(it, lens)).collect())
        }
        Node::Not(inner) => Node::Not(Box::new(with_lens(*inner, lens))),
        Node::Atom(atom) => Node::Atom(Atom {
            lens: Some(lens.clone()),
            ..atom
        }),
    };
}

/// `union`, and `value_union`.
fn parse_union(
    input: ParseStream,
    scope: Scope,
) -> syn::Result<Node> {
    let mut items = vec![parse_inter(input, scope)?];
    while SetOp::peek(input) == Some(SetOp::Union) {
        SetOp::Union.parse(input)?;
        items.push(parse_inter(input, scope)?);
    }
    return Ok(match items.len() {
        1 => items.pop().unwrap(),
        _ => Node::Or(items),
    });
}

/// `inter`, and `value_inter`.
fn parse_inter(
    input: ParseStream,
    scope: Scope,
) -> syn::Result<Node> {
    let mut items = vec![parse_unary(input, scope)?];
    while SetOp::peek(input) == Some(SetOp::Inter) {
        SetOp::Inter.parse(input)?;
        items.push(parse_unary(input, scope)?);
    }
    return Ok(match items.len() {
        1 => items.pop().unwrap(),
        _ => Node::And(items),
    });
}

/// `unary = closed | image`, and `value_unary`.
fn parse_unary(
    input: ParseStream,
    scope: Scope,
) -> syn::Result<Node> {
    if scope == Scope::Domain && starts_image(input) {
        return parse_image(input);
    }
    return parse_closed(input, scope);
}

/// `closed = "!" , closed | primary`: `!` never precedes an image, where it
/// would read as complementing the lens.
fn parse_closed(
    input: ParseStream,
    scope: Scope,
) -> syn::Result<Node> {
    if !input.peek(Token![!]) {
        return parse_primary(input, scope);
    }
    let begin = input.cursor();
    input.parse::<Token![!]>()?;
    if scope == Scope::Domain && starts_image(input) {
        let fork = input.fork();
        let image = parse_image_parts(&fork)?;
        let flipped = Image {
            excluded: !image.excluded,
            ..image
        };
        return Err(syn::Error::new_spanned(
            tokens_between(begin, fork.cursor()),
            format!(
                "a leading `!` reads as complementing `{}`; write `{}`",
                flipped.lens,
                flipped.text()
            ),
        ));
    }
    return Ok(complement(parse_closed(input, scope)?));
}

/// An image and its spelled parts, for diagnostics.
struct Image {
    node: Node,
    /// `rem_euclid(16)`
    lens: String,
    /// Written with `!in`; `node` is already complemented.
    excluded: bool,
    /// `{3, 5}`
    values: String,
}

impl Image {
    fn keyword(&self) -> &'static str {
        return match self.excluded {
            true => "!in",
            false => "in",
        };
    }

    /// `rem_euclid(16) !in {3, 5}`
    fn text(&self) -> String {
        return format!("{} {} {}", self.lens, self.keyword(), self.values);
    }
}

/// `lens [!]in value_unary`.
fn parse_image_parts(input: ParseStream) -> syn::Result<Image> {
    let begin = input.cursor();
    let name = input.parse::<Ident>()?;
    let lens = parse_lens(input, name)?;
    let lens_text = spelled(begin, input.cursor());
    let excluded = input.peek(Token![!]);
    if excluded {
        input.parse::<Token![!]>()?;
    }
    input.parse::<Token![in]>()?;
    let values_begin = input.cursor();
    let values = parse_unary(input, Scope::Values)?;
    let values_text = spelled(values_begin, input.cursor());
    let image = with_lens(values, &lens);
    return Ok(Image {
        node: match excluded {
            true => complement(image),
            false => image,
        },
        lens: lens_text,
        excluded,
        values: values_text,
    });
}

/// An image, which no `|` or `&` may follow: it could extend the values or
/// combine the image with the surrounding set.
fn parse_image(input: ParseStream) -> syn::Result<Node> {
    let begin = input.cursor();
    let image = parse_image_parts(input)?;
    let Some(op) = SetOp::peek(input)
    else {
        return Ok(image.node);
    };
    let mut tokens = tokens_between(begin, input.cursor());
    if let Some((punct, _)) = input.cursor().punct() {
        tokens.extend([TokenTree::Punct(punct)]);
    }
    let op = op.symbol();
    let (lens, keyword, values) = (&image.lens, image.keyword(), &image.values);
    return Err(syn::Error::new_spanned(
        tokens,
        format!(
            "ambiguous `{op}` after `{}`; write `{lens} {keyword} ({values} \
            {op} …)` to extend the function's results, or `({}) {op} …` to \
            combine sets",
            image.text(),
            image.text(),
        ),
    ));
}

/// `!node`. `{c}` and `mul(c)` have complement atoms, which De Morgan carries
/// through lists of them (`!{1, 2}` is `!{1} & !{2}`); a complement of a
/// complement is the set itself; anything else is wrapped.
fn complement(node: Node) -> Node {
    return match node {
        Node::Atom(Atom {
            lens,
            test: Test::Point { value, negated },
            span,
        }) => Node::Atom(Atom {
            lens,
            test: Test::Point {
                value,
                negated: !negated,
            },
            span,
        }),
        Node::Atom(Atom {
            lens,
            test: Test::Divisible { divisor, negated },
            span,
        }) => Node::Atom(Atom {
            lens,
            test: Test::Divisible {
                divisor,
                negated: !negated,
            },
            span,
        }),
        Node::Not(inner) => *inner,
        Node::Or(items) if items.iter().all(flips) => {
            Node::And(items.into_iter().map(complement).collect())
        }
        Node::And(items) if items.iter().all(flips) => {
            Node::Or(items.into_iter().map(complement).collect())
        }
        other => Node::Not(Box::new(other)),
    };
}

/// `complement` leaves no `Node::Not` wrapper.
fn flips(node: &Node) -> bool {
    return match node {
        Node::Atom(atom) => {
            matches!(atom.test, Test::Point { .. } | Test::Divisible { .. })
        }
        Node::Not(_) => true,
        Node::Or(items) | Node::And(items) => items.iter().all(flips),
    };
}

/// `primary`, and `value_primary` in the values scope.
fn parse_primary(
    input: ParseStream,
    scope: Scope,
) -> syn::Result<Node> {
    let span = input.span();
    if starts_range(input) {
        return parse_range(input);
    }
    if input.peek(token::Paren) {
        if is_parenthesized_constant(input) {
            return span.fail("constants take no parentheses; write 1..5");
        }
        let content;
        syn::parenthesized!(content in input);
        let node = parse_union(&content, scope)?;
        if !content.is_empty() {
            return Err(stray(&content));
        }
        return Ok(node);
    }
    if input.peek(token::Brace) {
        return parse_roster(input);
    }
    if input.peek(syn::Ident) {
        let name = input.parse::<Ident>()?;
        if name == MUL {
            return parse_multiples(input, name);
        }
        let lens = parse_lens(input, name)?;
        if scope == Scope::Values {
            return lens.name.fail(
                "after `in` come the function's results: a range, {…}, \
                mul(…), or (…)",
            );
        }
        return Ok(Node::Atom(Atom {
            lens: Some(lens),
            test: Test::Predicate,
            span,
        }));
    }
    return span.fail(match scope {
        Scope::Domain => "expected a range, {…}, mul(…), a function, or (…)",
        Scope::Values => "expected a range, {…}, mul(…), or (…)",
    });
}

/// `(1)` or `(-u8::MAX)`: a group holding only a constant (D17).
fn is_parenthesized_constant(input: ParseStream) -> bool {
    use syn::parse::Parser as _;

    let Ok(TokenTree::Group(group)) = input.fork().parse::<TokenTree>()
    else {
        return false;
    };
    let lone_constant = |content: ParseStream| -> syn::Result<()> {
        if !starts_const(content) {
            return content.span().fail("not a constant");
        }
        content.parse::<Const>()?;
        return Ok(());
    };
    return lone_constant.parse2(group.stream()).is_ok();
}

/// `{1, 3}`: one point atom per value.
fn parse_roster(input: ParseStream) -> syn::Result<Node> {
    let span = input.span();
    let content;
    syn::braced!(content in input);
    let mut items = Vec::new();
    while !content.is_empty() {
        let value = content.parse::<Const>()?;
        if content.peek(Token![..]) {
            return content.span().fail(
                "{…} lists single values; write ranges outside braces: \
                {1} | 3..5",
            );
        }
        items.push(Node::Atom(Atom {
            lens: None,
            span: value.span,
            test: Test::Point {
                value,
                negated: false,
            },
        }));
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
    }
    return match items.len() {
        0 => span.fail("write at least one value: {1}"),
        1 => Ok(items.pop().unwrap()),
        _ => Ok(Node::Or(items)),
    };
}

/// `mul(c)`, after its name.
fn parse_multiples(
    input: ParseStream,
    name: Ident,
) -> syn::Result<Node> {
    if !input.peek(token::Paren) {
        return name.span().fail("write mul(3) for the multiples of 3");
    }
    let content;
    syn::parenthesized!(content in input);
    let divisor = content.parse::<Const>()?;
    if !content.is_empty() {
        return content.span().fail("mul takes one constant: mul(3)");
    }
    return Ok(Node::Atom(Atom {
        lens: None,
        test: Test::Divisible {
            divisor,
            negated: false,
        },
        span: name.span(),
    }));
}

/// A function name and its optional constant arguments.
fn parse_lens(
    input: ParseStream,
    name: Ident,
) -> syn::Result<Lens> {
    if name == MIN || name == MAX {
        return Err(bare_bound_error(&name));
    }
    let mut args = Vec::new();
    if input.peek(token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        while !content.is_empty() {
            args.push(content.parse::<Const>()?);
            if content.is_empty() {
                break;
            }
            content.parse::<Token![,]>()?;
        }
    }
    let function = FnName::of(&name.to_string());
    return Ok(Lens {
        name,
        function,
        args,
    });
}

fn starts_const(input: ParseStream) -> bool {
    return input.peek(LitInt)
        || input.peek(LitFloat)
        || input.peek(Token![-])
        || input.peek(Token![::])
        || (input.peek(syn::Ident) && input.peek2(Token![::]));
}

/// `..` also peeks `..=`.
fn starts_range(input: ParseStream) -> bool {
    return input.peek(Token![..]) || starts_const(input);
}

fn parse_range(input: ParseStream) -> syn::Result<Node> {
    let span = input.span();
    let lo = match input.peek(Token![..]) {
        true => None,
        false => Some(input.parse::<Const>()?),
    };
    if let Some(lo) = &lo
        && !input.peek(Token![..])
    {
        return lo.span.fail("a value alone is not a set; write it as {…}");
    }
    let inclusive = if input.peek(Token![..=]) {
        input.parse::<Token![..=]>()?;
        true
    }
    else {
        input.parse::<Token![..]>()?;
        false
    };
    let hi = starts_const(input)
        .then(|| input.parse::<Const>())
        .transpose()?;
    if inclusive && hi.is_none() {
        return input.span().fail("..= needs an upper bound");
    }
    return Ok(Node::Atom(Atom {
        lens: None,
        test: Test::Range { lo, hi, inclusive },
        span,
    }));
}

impl Parse for Const {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        if input.peek(token::Paren) {
            return span.fail("constants take no parentheses; write 1..5");
        }

        let negated = input.peek(Token![-]);
        let minus = negated.then(|| input.parse::<Token![-]>()).transpose()?;

        let (kind, tokens) = if input.peek(LitInt) {
            parse_int_literal(input, negated)?
        }
        else if input.peek(LitFloat) {
            parse_float_literal(input, negated)?
        }
        else if input.peek(Token![::])
            || (input.peek(syn::Ident) && input.peek2(Token![::]))
        {
            parse_const_path(input, negated, span)?
        }
        else if input.peek(syn::Ident) {
            let name = input.parse::<Ident>()?;
            return match name == MIN || name == MAX {
                true => Err(bare_bound_error(&name)),
                false => name.span().fail("need a constant"),
            };
        }
        else {
            return span.fail("need a constant");
        };

        reject_arithmetic(input)?;

        let tokens = quote! { #minus #tokens };
        return Ok(Self { tokens, span, kind });
    }
}

fn parse_int_literal(
    input: ParseStream,
    negated: bool,
) -> syn::Result<(ConstKind, TokenStream)> {
    let literal = input.parse::<LitInt>()?;
    let digits = literal
        .base10_digits()
        .parse::<u128>()
        .map_err(|_| literal.errorful("integer constant out of range"))?;
    let value = match negated {
        true => Int::NonNegative(digits).negate(),
        false => Int::NonNegative(digits),
    };
    let kind = ConstKind::Int {
        value,
        suffix: literal.suffix().to_owned(),
    };
    return Ok((kind, literal.to_token_stream()));
}

fn parse_float_literal(
    input: ParseStream,
    negated: bool,
) -> syn::Result<(ConstKind, TokenStream)> {
    let literal = input.parse::<LitFloat>()?;
    let kind = ConstKind::Float {
        digits: literal.base10_digits().to_owned(),
        suffix: literal.suffix().to_owned(),
        negated,
    };
    return Ok((kind, literal.to_token_stream()));
}

/// Includes the `::core::…` rewrite of float paths.
fn parse_const_path(
    input: ParseStream,
    negated: bool,
    span: Span,
) -> syn::Result<(ConstKind, TokenStream)> {
    let path = input.parse::<syn::Path>()?;
    let kind = classify_const_path(&path, negated)?;
    // `std::f64::consts::PI` would not resolve in a no_std consumer.
    let tokens = match &kind {
        ConstKind::FloatPath {
            ty, name, consts, ..
        } => {
            let ty = Ident::new(ty.name(), span);
            let name = Ident::new(name, span);
            match consts {
                true => quote! { ::core::#ty::consts::#name },
                false => quote! { ::core::primitive::#ty::#name },
            }
        }
        _ => path.to_token_stream(),
    };
    return Ok((kind, tokens));
}

fn classify_const_path(
    path: &syn::Path,
    negated: bool,
) -> syn::Result<ConstKind> {
    let names = path
        .segments
        .iter()
        .map(|it| it.ident.to_string())
        .collect::<Vec<_>>();
    let names = names.iter().map(String::as_str).collect::<Vec<_>>();
    let invalid = "need a constant such as u32::MAX or a literal";
    return match names.as_slice() {
        [
            ty,
            bound,
        ] if N::of(ty).is_some() && path.leading_colon.is_none() => {
            let max = match *bound {
                MAX => true,
                MIN => false,
                _ => return path.fail(invalid),
            };
            Ok(ConstKind::Bound {
                ty: N::of(ty).unwrap(),
                max,
                negated,
            })
        }
        [
            ty,
            name,
        ] if F::of(ty).is_some() && path.leading_colon.is_none() => {
            if *name == "NAN" {
                return path
                    .fail("NaN never compares equal; use is_nan or !is_nan");
            }
            Ok(ConstKind::FloatPath {
                ty: F::of(ty).unwrap(),
                name: (*name).to_owned(),
                consts: false,
                negated,
            })
        }
        [
            ty,
            "consts",
            name,
        ]
        | [
            "core" | "std",
            ty,
            "consts",
            name,
        ] if F::of(ty).is_some() => Ok(ConstKind::FloatPath {
            ty: F::of(ty).unwrap(),
            name: (*name).to_owned(),
            consts: true,
            negated,
        }),
        _ => path.fail(invalid),
    };
}

fn reject_arithmetic(input: ParseStream) -> syn::Result<()> {
    if input.peek(Token![+])
        || input.peek(Token![*])
        || input.peek(Token![/])
        || (input.peek(Token![-]) && !input.peek(Token![->]))
    {
        return input
            .span()
            .fail("need a constant; arithmetic is not supported");
    }
    return Ok(());
}

/// "need a constant; write u32::MAX …", shared with `parse_lens`.
fn bare_bound_error(name: &Ident) -> syn::Error {
    return name.span().errorful(format!(
        "need a constant; write u32::{name} with the operand's type"
    ));
}

// =============================================================================
// Untyped analysis (parse time)

/// Three-valued result of evaluating a domain before the value type is known.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Tri {
    True,
    False,
    Unknown,
}

impl From<Option<bool>> for Tri {
    fn from(verdict: Option<bool>) -> Self {
        return match verdict {
            Some(true) => Self::True,
            Some(false) => Self::False,
            None => Self::Unknown,
        };
    }
}

// =============================================================================
// Shared by the integer and float stages

/// What normalization groups by: the value, or one lens application.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum OperandKey {
    Subject,
    /// The function and its argument token texts.
    Lens(FnName, Vec<String>),
}

impl OperandKey {
    pub(crate) fn of(
        function: FnName,
        args: &[Const],
    ) -> Self {
        return Self::Lens(
            function,
            args.iter().map(|it| it.tokens.to_string()).collect(),
        );
    }

    /// `rem_euclid(16)`, for diagnostics; empty for the value itself.
    pub(crate) fn text(&self) -> String {
        let Self::Lens(function, args) = self
        else {
            return String::new();
        };
        if args.is_empty() {
            return function.name().to_owned();
        }
        let args = args
            .iter()
            .map(|it| it.replace(" :: ", "::"))
            .collect::<Vec<_>>()
            .join(", ");
        return format!("{}({args})", function.name());
    }
}

/// Kleene `||` (`or`) or `&&`; stops at the first deciding verdict.
pub(crate) fn kleene(
    verdicts: impl Iterator<Item = Option<bool>>,
    or: bool,
) -> Option<bool> {
    let mut result = Some(!or);
    for it in verdicts {
        match it {
            Some(it) if it == or => return Some(or),
            Some(_) => {}
            None => result = None,
        }
    }
    return result;
}

/// The item order, `Ok(key)` holding a group's place, and the groups.
pub(crate) type Grouped<K, T> = (Vec<Result<K, T>>, BTreeMap<K, Vec<T>>);

/// Items in first-appearance order; `Ok(key)` holds a group's place.
pub(crate) fn group_in_order<T, K: Ord + Clone>(
    items: Vec<T>,
    key: impl Fn(&T) -> Option<&K>,
) -> Grouped<K, T> {
    let mut groups = BTreeMap::<K, Vec<T>>::new();
    let mut order = Vec::<Result<K, T>>::new();
    for item in items {
        let Some(group) = key(&item).cloned()
        else {
            order.push(Err(item));
            continue;
        };
        if !groups.contains_key(&group) {
            order.push(Ok(group.clone()));
        }
        groups.entry(group).or_default().push(item);
    }
    return (order, groups);
}

/// `empty` for no item, the item for one, `list(items)` otherwise.
pub(crate) fn collapse<T>(
    mut items: Vec<T>,
    empty: T,
    list: fn(Vec<T>) -> T,
) -> T {
    return match items.len() {
        0 => empty,
        1 => items.pop().unwrap(),
        _ => list(items),
    };
}

/// `functions`' names, for diagnostics.
pub(crate) fn listing(functions: &[FnName]) -> String {
    return functions
        .iter()
        .map(|it| it.name())
        .collect::<Vec<_>>()
        .join(", ");
}
