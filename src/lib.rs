extern crate core;

use std::panic::UnwindSafe;

use syn::parse::{
    Parse,
    Parser as _,
};
use syn::{
    Item,
    ItemEnum,
    ItemStruct,
    ItemTrait,
};

use crate::zz_lib::Named as _;
use crate::zz_profile_integral::ProfileKind;
use crate::zz_values::F;

pub(crate) mod bitflag;
pub(crate) mod constructor;
pub(crate) mod float;
pub(crate) mod friendship;
pub(crate) mod integral;
pub(crate) mod integral_cfg;
pub(crate) mod integral_compute;
pub(crate) mod integral_forward;
pub(crate) mod integral_friend;
pub(crate) mod integral_impls;
pub(crate) mod integral_items;
pub(crate) mod integral_proof;
pub(crate) mod integral_within;
pub(crate) mod marker_tree;
pub(crate) mod packed;
pub(crate) mod text;
pub(crate) mod zz_conv;
pub(crate) mod zz_domain;
pub(crate) mod zz_domain_closure;
pub(crate) mod zz_domain_float;
pub(crate) mod zz_domain_int;
pub(crate) mod zz_domain_within;
pub(crate) mod zz_domain_within_const;
pub(crate) mod zz_flags;
pub(crate) mod zz_forward;
pub(crate) mod zz_friend;
pub(crate) mod zz_hide;
pub(crate) mod zz_lib;
pub(crate) mod zz_lit;
pub(crate) mod zz_names;
pub(crate) mod zz_profile_integral;
pub(crate) mod zz_runner;
pub(crate) mod zz_values;

// Never compiled into the library: `src/e2e/mod.rs` is the root of the `e2e`
// test target (Cargo.toml). The declaration only makes `cargo fmt` reach it.
#[cfg(any())]
mod e2e;
#[cfg(test)]
mod unit;

/// Generates a transparent primitive-integer newtype and its configured API.
///
/// # Inputs
///
/// - **Required item:** a tuple struct with exactly one primitive integer field.
///   Generic integral structs require that value field to be private.
/// - All configuration arguments below are optional.
///
/// # Configuration
///
/// - `konst = none | stable | nightly`
/// - `valid = PATH | [PATH, ...]`, `domain = SET`, and `error = ()`. `SET`
///   combines ranges, rosters (`{1, 3}`), multiples (`mul(3)`), and integer
///   functions (`abs in ..=3`, `abs !in ..=3`, `count_ones in {1}`,
///   `is_power_of_two`) with `!`, `&`, `|`, and parentheses:
///   `domain = (1..12 | 87..) & !{10} & !mul(3)`.
/// - `friend = SPEC | [SPEC, ...]`, where `SPEC` is
///   `conversion(Source) -> CAPABILITY | [CAPABILITY, ...]`
/// - `with = FLAG | [FLAG, ...]` and `without = FLAG | [FLAG, ...]`
/// - `get_raw = PATH`
/// - `produce = CALLBACK(_) -> Type`
/// - `profile = PROFILE | [PROFILE, ...]`
/// - `facet = FACET | [FACET, ...]`
/// - `tree = NODE | [NODE | without = FLAGS, ...]`
/// - `hide_in = MODULE | none`
/// - `panic_at = anything | arithmetic | overflow | nothing`
/// - `prove = OP | [OP, ...]`: operations that must be proven closed over the
///   domain, each an operator flag (`math_add`, `math_add(Rhs)`) or a
///   `core_int` method (`abs`)
/// - `lit_name = IDENT` and `lit_path = crate::PATH::Type` for the literal
///   macro
///
/// `PROFILE` is `int`, `circular`, `delta`, `sequential`, `wrapping`, `point`,
/// or `nominal`. A profile body accepts `with` and `without`; only `circular`
/// also accepts `precedes = bool`. `FACET` is one profile other than `nominal`
/// with an optional body that accepts `with`, `without`, `friend`, and
/// `produce`. A facet slot whose output is `Self` also gets its assignment
/// operator (`+=` for a `math_add` slot) while its facet has the `assign_*`
/// flag on; facets inherit the root's, and shift assignments stay
/// wrapper-wide. Every binary operator has its own assignment flag
/// (`assign_xor` for `^=`), on by default and independent of the operator's
/// `math_*` flag. `TREE` uses the [`tree`] node grammar, and a bracketed list
/// also takes the tree's `without`. Its private `MarkerTreeRoot` seal stays
/// generated, while its nodes are re-exported beside the annotated item.
///
/// By default the item and its generated code live in a private module named
/// `__` plus the snake-cased type name (`hide_in = MODULE` picks the name) and
/// are re-exported at the call site, so the tuple constructor and a private
/// field stay unreachable even from the defining module. `hide_in = none`
/// disables this; `get_raw` and a custom `_or_panic` require it.
///
/// `panic_at` says what generated code may still panic on: `anything` (the
/// default), `arithmetic` (only Rust's division by zero and overflow),
/// `overflow` (only Rust's `overflow-checks`, off in release builds), or
/// `nothing`. Below `anything`, every operator returns
/// `Result<Output, Option<Raw>>`: `Err(Some(raw))` for a result that fails
/// validation, `Err(None)` when there is no result (`sequential` overflow;
/// division by zero and `MIN / -1` from `overflow`; any overflow at
/// `nothing`). Every result is validated; domain proofs are not used.
/// Functions keep their signatures, so those that would have to panic are
/// left out: assignment operators, `of` (`try_of` returns
/// `Result<Self, Raw>`), `map` (`try_map` remains), and the `core_int`,
/// `core_float`, and `float_math` functions returning a validated `Self`
/// (`checked_*` and reads remain). `Sum` and `Product` collect into the
/// operators' `Result`. `unsafe` friends and `with_blind_trust` are errors.
///
/// `FLAG` is an integral generation, operation, friendship, storage, or
/// `konst_*` flag. `Math` expands to `Make` and `Bit` for integral friends.
///
/// `_(Source) -> Within` constructs without validation once rustc has
/// proven at compile time that every value of `Source` (an integral wrapper,
/// a bitflag value type, or a primitive integer) lies in the domain; a
/// failed proof is a compile error naming a counterexample. `Within` implies
/// `Make`, converts through the source's raw value, and takes no explicit
/// conversion or `unsafe`. Sources publish what the proof reads unless
/// `without = within_source`; generic targets prove where they are
/// instantiated, so `cargo build` reports their failures.
///
/// ```
/// #[typekin::integral(domain = ..=100)]
/// struct Percent(u8);
///
/// #[typekin::integral(domain = ..=255, friend = _(Percent) -> Within)]
/// struct Byte(u16);
///
/// assert_eq!(Byte::of(Percent::try_make(42).unwrap()).raw(), 42);
/// ```
///
/// `without = fn_NAME` removes one generated function that forwards to
/// `core` (`fn_count_ones`, `fn_checked_add`), so the caller can define their
/// own; it is accepted only in `without`, and naming a function the type does
/// not generate is an error. A profile body may remove functions too; a facet
/// may not.
///
/// Signed integers implement `Neg` following the profile's arithmetic;
/// unsigned ones have none. Integers convert to `f32`/`f64` with `into_*`
/// where every value is exact and with checked `try_into_*` otherwise.
/// `Neg` and `abs` skip validation when the domain makes their result valid
/// (`domain = -10..=10`).
///
/// Pointer-sized conversions and implicit friends follow the consumer target's
/// pointer width, including when cross-compiling:
///
/// ```
/// #[typekin::integral(konst = none)]
/// struct Wide(u64);
///
/// let value = Wide::make(1u64 << 32);
/// #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
/// assert_eq!(value.try_into_usize(), Err(()));
/// #[cfg(target_pointer_width = "64")]
/// assert_eq!(value.into_usize(), (1u64 << 32) as usize);
/// ```
///
/// An explicit empty capability list adds no capabilities. Any supplied
/// callback is still checked as `fn(Source) -> Primitive` without being called.
/// Const friendship and const facet operators also require a const callback.
///
/// Incorrect input and output types are rejected:
///
/// ```compile_fail
/// struct Source;
/// fn convert(_: u16) -> u32 { 0 }
/// #[typekin::integral(konst = none, friend = convert(Source) -> [])]
/// struct Value(u32);
/// # fn main() {}
/// ```
///
/// ```compile_fail
/// struct Source;
/// fn convert(_: Source) -> u16 { 0 }
/// #[typekin::integral(konst = none, friend = convert(Source) -> [])]
/// struct Value(u32);
/// # fn main() {}
/// ```
///
/// Const friendship also rejects a non-const callback:
///
/// ```compile_fail
/// #![feature(const_trait_impl)]
/// struct Source;
/// fn convert(_: Source) -> u32 { 0 }
/// #[typekin::integral(
///     konst = none,
///     with = konst_friendship,
///     friend = convert(Source) -> [],
/// )]
/// struct Value(u32);
/// # fn main() {}
/// ```
#[proc_macro_attribute]
pub fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return expand_hideable(
        attr,
        item,
        syn::parse2::<integral_cfg::Cfg>,
        |attr, item: ItemStruct| integral::ekran(Box::new(attr), item),
    );
}

/// Generates a transparent `f32`/`f64` newtype and its configured API.
///
/// # Inputs
///
/// - **Required item:** a tuple struct with exactly one `f32` or `f64` field.
///   Generic float structs require that value field to be private.
/// - Every configuration argument of [`integral`], all optional, plus `math`.
///
/// Floats share the integral configuration. Differences:
///
/// - `profile` and facets take only `int` and `point`.
/// - `prove` is rejected, and so are the integer-only flags (`proven`,
///   `non_zero`, `range`, `core_int`, `bit_access`, the bitwise operators,
///   binary/octal/hex formatting).
/// - `domain` uses float constants (`0.0..=1.0 & !{0.5} & is_finite`).
///   Ranges, rosters, and multiples reject NaN; their complements (`!{0.5}`)
///   admit it. A domain that rejects NaN makes the type NaN-free: it then gets
///   `Eq`, `Ord`, `Hash`, and `_cmp` when its value field is private. Other
///   types have none of them, and asking for them is an error.
/// - `math = core | std | nightly` selects where `floor`, `sqrt`, … come from
///   in domains and the `float_math` API: `std` uses inherent methods,
///   `nightly` uses `core::fNN::math` (no_std, `#![feature(core_float_math)]`).
/// - `Math` friends grant `Make` and arithmetic only; `Bit` and `Within`
///   friends are rejected.
/// - Floats add `Neg`, `LowerExp`/`UpperExp`, `Sum`/`Product` (one validation
///   per call), `core_float` methods, and exact float-to-integer `try_into_*`.
/// - `without = fn_NAME` removes one `core_float` or `float_math` function
///   (`fn_round_ties_even`, `fn_is_nan`), as for integers.
///
/// ```
/// #[typekin::float(konst = none)]
/// struct Meters(f64);
///
/// let width = Meters::make(2.5);
/// assert_eq!((-width * 2.0).raw(), -5.0);
/// assert_eq!(Meters::make(f64::NAN).partial_cmp(&width), None);
/// ```
///
/// A NaN-free domain is totally ordered:
///
/// ```
/// #[typekin::float(konst = none, domain = 0.0..=1.0)]
/// struct Unit(f64);
///
/// assert!(Unit::try_make(f64::NAN).is_err());
/// let half = Unit::try_make(0.5).unwrap();
/// let zero = Unit::try_make(0.0).unwrap();
/// assert_eq!(half.max(zero), half);
/// assert_eq!(Unit::try_make(-0.0).unwrap(), zero);
/// ```
///
/// Conversions to integers are checked; `as` would round or saturate:
///
/// ```
/// #[typekin::float(konst = none)]
/// struct Meters(f64);
///
/// assert_eq!(Meters::make(4.0).try_into_u8(), Ok(4));
/// assert_eq!(Meters::make(4.5).try_into_u8(), Err(()));
/// assert_eq!(Meters::make(256.0).try_into_u8(), Err(()));
/// assert_eq!(Meters::make(f64::NAN).try_into_i64(), Err(()));
/// ```
///
/// `Ord` needs a private value field: a public one could store NaN.
///
/// ```compile_fail
/// #[typekin::float(konst = none, domain = !is_nan, with = ord)]
/// pub struct Exposed(pub f64);
/// # fn main() {}
/// ```
#[proc_macro_attribute]
pub fn float(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parser = |attr: proc_macro2::TokenStream| {
        return integral_cfg::Cfg::parse_float.parse2(attr);
    };

    return expand_hideable(attr, item, parser, |attr, item: ItemStruct| {
        integral::ekran(Box::new(attr), item)
    });
}

/// Generates an integral newtype with the fixed `int` profile.
///
/// Equivalent to `#[typekin::integral(profile = int, ...)]`: it takes the item
/// and every configuration argument of [`integral`] except `profile`.
#[proc_macro_attribute]
pub fn int(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Int, attr, item);
}

/// Generates an integral newtype with the fixed `circular` profile.
///
/// Equivalent to `#[typekin::integral(profile = circular, ...)]`: it takes the
/// item and every configuration argument of [`integral`] except `profile`,
/// plus `precedes = bool`, which configures the selected circular profile
/// directly.
#[proc_macro_attribute]
pub fn circular(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Circular, attr, item);
}

/// Generates an integral newtype with the fixed `delta` profile.
///
/// Equivalent to `#[typekin::integral(profile = delta, ...)]`: it takes the
/// item and every configuration argument of [`integral`] except `profile`.
#[proc_macro_attribute]
pub fn delta(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Delta, attr, item);
}

/// Generates an integral newtype with the fixed `sequential` profile.
///
/// Equivalent to `#[typekin::integral(profile = sequential, ...)]`: it takes
/// the item and every configuration argument of [`integral`] except `profile`.
#[proc_macro_attribute]
pub fn sequential(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Sequential, attr, item);
}

/// Generates an integral newtype with the fixed `wrapping` profile.
///
/// Equivalent to `#[typekin::integral(profile = wrapping, ...)]`: it takes the
/// item and every configuration argument of [`integral`] except `profile`.
#[proc_macro_attribute]
pub fn wrapping(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Wrapping, attr, item);
}

/// Generates an integral or float newtype with the fixed `point` profile.
///
/// Equivalent to `#[typekin::integral(profile = point, ...)]`, or on an
/// `f32`/`f64` field to `#[typekin::float(profile = point, ...)]`: it takes
/// the item and every configuration argument of [`integral`] (of [`float`] for
/// a float field) except `profile`.
///
/// Float points also omit `Neg`, `Sum`, `Product`, and the
/// `core_float`/`float_math` methods.
#[proc_macro_attribute]
pub fn point(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Point, attr, item);
}

/// Generates an integral newtype with the fixed `nominal` profile: a value
/// that is only told apart from others, such as an identifier, a port, or a
/// header field.
///
/// Equivalent to `#[typekin::integral(profile = nominal, ...)]`: it takes the
/// item and every configuration argument of [`integral`] except `profile`.
///
/// A nominal value keeps construction, `raw()`, `Eq`, `Hash`, `Debug`, `Copy`,
/// and `Clone`, and drops every operator, linear ordering, conversion, radix
/// formatting, `AsRef`, `FromStr`, `map`, and its `Within` source constants.
/// Like `point` it selects no arithmetic, so `profile = [circular, nominal]`
/// keeps circular arithmetic and `precedes` for the operators `with`
/// re-enables.
///
/// ```
/// #[typekin::nominal(with = display)]
/// struct Port(u16);
///
/// let http = Port::make(80);
/// assert_eq!(http, Port::make(80));
/// assert_eq!(http.raw(), 80);
/// assert_eq!(http.to_string(), "80");
/// ```
#[proc_macro_attribute]
pub fn nominal(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Nominal, attr, item);
}

/// Generates a sealed hierarchy of marker traits and zero-sized leaf structs.
///
/// # Inputs
///
/// - **Required item:** a non-generic, safe, empty trait without supertraits.
///   It is the root seal.
/// - **Required attribute:** one or more marker nodes.
/// - **Optional attribute:** `without = FLAG | [FLAG, ...]`, anywhere among the
///   nodes.
///
/// A node is `[visibility] Name` for a leaf or `[visibility] Name { ENTRY, ...
/// }` for a branch, where an entry is a child node or an associated type.
/// Omitted visibility inherits from its parent; top-level nodes inherit the
/// root seal's visibility. Branches extend their parent and the tree seal,
/// while leaves implement every ancestor trait. Names must be unique within
/// the tree, and every branch needs at least one child. Nodes accept doc
/// comments and no other attributes.
///
/// Leaves derive `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq`,
/// `Hash`, `PartialOrd`, and `Ord`, and top-level branches require them, so
/// code generic over any branch can copy, compare, or print its markers, and
/// derives on types generic over them need no extra bounds. `without` removes
/// any of them: `clone`, `copy`, `debug`, `default`, `partial_eq`, `eq`,
/// `hash`, `partial_ord`, `ord`. A trait another one needs must be removed with
/// it (`copy` needs `clone`). These supertraits make branches
/// non-`dyn`-compatible; removing all but `debug` restores it.
///
/// # Associated types
///
/// A branch declares `type Name` or `type Name: BOUNDS` among its children.
/// A leaf binds every type its ancestors declare, impl-style:
/// `Name { type Kind = Other; }`. A type leaves bind that no ancestor declares
/// is declared on their parent branch, bounded by the deepest node above every
/// leaf it is bound to (`Self` is the binding leaf), or unbounded when a bound
/// type is not a leaf of the tree. A name is declared once along a path.
///
/// `type Rev = A <-> B`, or `type Rev = [A <-> B, C <-> C, ...]`, in a branch
/// is shorthand for `type Rev: Branch<Rev = Self>` and the leaves
/// `A { type Rev = B; }` and `B { type Rev = A; }`; `C <-> C` declares one
/// leaf bound to itself. The bound makes `Rev` an involution: generic code
/// sees `<D::Rev as Branch>::Rev` as `D`. Pair members take a visibility and
/// doc comments; their leaves bind only `Rev`.
///
/// ```text
/// #[typekin::tree(
///     without = [hash, partial_ord, ord],
///     /// Which way traffic flows.
///     pub Direction {
///         /// The opposite direction.
///         type Rev = pub In <-> pub Out,
///     },
/// )]
/// trait MakerSeal {}
/// ```
///
/// Keep the root trait private to seal public branches. The macro allows public
/// branches with private root bounds and suppresses the corresponding lint.
#[proc_macro_attribute]
pub fn tree(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return expand::<marker_tree::Cfg, ItemTrait>(
        attr,
        item,
        marker_tree::expand,
    );
}

/// Generates a unit-variant bitflag enum and its `{Enum}Value` wrapper.
///
/// # Inputs
///
/// - **Required item:** a non-generic enum with only unit variants.
/// - **Required representation:** exactly one numeric representation from
///   `repr = TYPE` or `#[repr(TYPE)]` on the enum.
/// - All other configuration arguments are optional.
///
/// # Configuration
///
/// - `konst = none | stable | nightly`
/// - `repr = TYPE`, `suffix = "Ident"`, and `value_name = "Ident"`
/// - `friend = SPEC | [SPEC, ...]`
/// - `with = FLAG | [FLAG, ...]` and `without = FLAG | [FLAG, ...]`
/// - `integral = [INTEGRAL_CONFIGURATION]`
/// - `tree = NODE | [NODE | without = FLAGS, ...]`
/// - `hide_in = MODULE | none` (root only; hides the enum and value type)
///
/// `SPEC` uses the friend conversion grammar. The bracketed `integral`
/// configuration accepts default-profile integral validation, friend, flag, and
/// accessor settings; it rejects `profile`, `facet`, `produce`, `tree`, and
/// nonzero storage.
/// Pointer-sized conversions follow the target width, as with [`integral`].
#[proc_macro_attribute]
pub fn bitflag(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return expand_hideable(
        attr,
        item,
        syn::parse2::<bitflag::Cfg>,
        |attr, item: ItemEnum| bitflag::ekran(Box::new(attr), item),
    );
}

/// Generates a packed record: a transparent unsigned integer whose fields
/// are typed bit windows.
///
/// # Inputs
///
/// - **Required item:** a non-generic struct with named fields, each with
///   exactly one `#[bits(LO..HI)]`, `#[bits(LO..=HI)]`, or `#[bit(N)]`. Bit 0
///   is the least significant. Doc comments are the only other field
///   attributes.
/// - **Required argument:** `repr = u8 | u16 | u32 | u64 | u128`.
/// - All other configuration arguments are optional.
///
/// # Configuration
///
/// - `konst = none | stable | nightly`
/// - `hide_in = MODULE | none`
/// - `values_name = Ident` (default `{Type}Values`)
/// - `with = FLAG | [FLAG, ...]` and `without = FLAG | [FLAG, ...]`: `debug`,
///   `eq`, `hash`, and `values_struct` (on by default), `non_zero`, `ord`, and
///   `serde` (off by default)
///
/// A field is `bool` (one bit), a fixed-width primitive integer (exactly its
/// width), or a typekin integer type (an integral wrapper, a bitflag value
/// type, or another packed record) whose every value fits its window, which
/// rustc proves at compile time. Windows may not overlap; unclaimed bits are
/// always zero.
///
/// The record gets `make` from the generated values struct, `fields` back
/// to it, a getter and a `with_{field}` setter per field, and the integral
/// core: `raw`, `try_make`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, and
/// optionally `Ord` and Serde. `make` and the setters never validate; raw
/// ingress (`try_make`, deserialization) checks every field and the unclaimed
/// bits. `with = non_zero` selects `NonZero` storage once rustc proves that
/// some field excludes zero.
#[proc_macro_attribute]
pub fn packed(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return expand_hideable(
        attr,
        item,
        syn::parse2::<packed::Cfg>,
        |attr, item: packed::PackedItem| packed::ekran(Box::new(attr), item),
    );
}

/// Generates a validated `String` newtype.
///
/// # Warning: `text` is not a replacement for an enum
///
/// Always prefer an enum for a known set of values, unless there is a very
/// strong, valid, and verified reason not to. `domain = {"draft", "final"}`
/// accepts a closed set of strings, but the type still stores a heap-allocated
/// `String`. It is not `Copy`, matches compare strings, and a typo in a match
/// arm compiles. An enum names each value in the type system, matches
/// exhaustively, and costs no allocation; Serde can still read and write it
/// as a string. Use `text` for open-ended strings checked by rules (`valid`),
/// not as a stringly-typed enum.
///
/// # Inputs
///
/// - **Required item:** a non-generic tuple struct with exactly one unqualified
///   `String` field.
/// - All configuration arguments are optional.
///
/// # Configuration
///
/// - `std = bool` and `konst = none | stable | nightly`
/// - `valid = PATH | [PATH, ...]` or `domain = {"literal", ...}`
/// - `error = ()`
/// - `friend = SPEC | [SPEC, ...]`
/// - `with = FLAG | [FLAG, ...]` and `without = FLAG | [FLAG, ...]`
/// - `tree = NODE | [NODE | without = FLAGS, ...]`
/// - `hide_in = MODULE | none`
///
/// `valid` and `domain` are mutually exclusive. Text friends accept `Make`,
/// `unsafe`, and self-only `Rel`; `Make` needs a conversion callback. Set
/// `std = false` for an `alloc`-backed generated API.
///
/// `without = fn_NAME` removes one generated function that forwards to
/// `String` (`fn_len`, `fn_try_push`); naming a function the type does not
/// generate is an error. The generated traits do not use these functions.
#[proc_macro_attribute]
pub fn text(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return expand_hideable(attr, item, syn::parse2::<text::Cfg>, text::ekran);
}

/// Generates a capability protocol for a concrete relationship type.
///
/// # Inputs
///
/// - **Required item:** a non-generic struct, enum, or union.
/// - **Required argument:** `relation = Type`.
/// - **Required capability:** at least one capability supplied by `friend` or
///   `extra_caps`.
/// - All other configuration arguments are optional.
///
/// # Configuration
///
/// - `value = Type`, `seal = Ident`, and `conversion = Ident`
/// - `friend = SPEC | [SPEC, ...]`
/// - `extra_caps = Ident | [Ident, ...]`
/// - `scope = self | _ | [visibility] module_name`
/// - `mod = _ | [visibility] module_name`
///
/// `SPEC` is `conversion(Source) -> Capability | [Capability, ...]`. `scope`
/// and `mod` are mutually exclusive; omitting both emits the protocol in an
/// anonymous `const` scope.
#[proc_macro_attribute]
pub fn friends(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return expand::<friendship::Cfg, Item>(attr, item, friendship::ekran);
}

/// Adds an explicit constructor to a friendship protocol.
///
/// # Inputs
///
/// - **Required item:** a concrete item with a
///   `#[typekin::friendship(...)]` attribute immediately below this one.
/// - **Required arguments:** `relationship = PATH` and
///   `scope = self | _ | [visibility] module_name`.
/// - All other configuration arguments are optional.
///
/// # Configuration
///
/// - `maker = Ident`
/// - `friend = Type | [Type, ...]`
///
/// Each selected friend must be declared by the paired friendship attribute.
/// `relationship` receives the generated relation and must construct `Self`.
#[proc_macro_attribute]
#[doc(hidden)] // Experimental AND unfinished!
pub fn constructor(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return expand::<constructor::Cfg, Item>(attr, item, constructor::ekran);
}

fn profiled_integral(
    profile: ProfileKind,
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let kind = match profile_kind(profile, &item) {
        Ok(kind) => kind,
        Err(error) => return error.into_compile_error().into(),
    };

    let parser = move |attr: proc_macro2::TokenStream| {
        let parser = |input: syn::parse::ParseStream| {
            return integral_cfg::Cfg::parse_kind_with_profile(
                input,
                profile,
                kind,
            );
        };
        return parser.parse2(attr);
    };

    return expand_hideable(attr, item.into(), parser, |attr, item| {
        integral::ekran(Box::new(attr), item)
    });
}

/// The value kind a fixed-profile entry point expands `item` as.
///
/// The kind decides friend expansion and flag defaults, so it is read from
/// the value field before the configuration is parsed. Only `point` accepts
/// an `f32`/`f64` field.
fn profile_kind(
    profile: ProfileKind,
    item: &proc_macro2::TokenStream,
) -> syn::Result<integral_cfg::Kind> {
    return match (float_field(item), profile) {
        (None, _) => Ok(integral_cfg::Kind::Int),
        (Some(span), profile) if !profile.spec().float => Err(syn::Error::new(
            span,
            format!(
                "the {} profile requires an integer value field",
                profile.name()
            ),
        )),
        (Some(_), ProfileKind::Point) => Ok(integral_cfg::Kind::Float),
        (Some(span), _) => Err(syn::Error::new(
            span,
            "use #[typekin::float] for f32/f64 value fields",
        )),
    };
}

/// The span of the item's first field type when it is `f32` or `f64`.
fn float_field(item: &proc_macro2::TokenStream) -> Option<proc_macro2::Span> {
    let item = syn::parse2::<ItemStruct>(item.clone()).ok()?;
    let field = item.fields.iter().next()?;
    return match &field.ty {
        syn::Type::Path(path)
            if path
                .path
                .get_ident()
                .is_some_and(|it| F::of(&it.to_string()).is_some()) =>
        {
            Some(syn::spanned::Spanned::span(&field.ty))
        }
        _ => None,
    };
}

/// Parses `attr` and `item`, then expands them with panics caught.
fn expand<C, I>(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
    ekran: fn(C, I) -> syn::Result<proc_macro2::TokenStream>,
) -> proc_macro::TokenStream
where
    C: Parse + UnwindSafe,
    I: Parse + UnwindSafe,
{
    let attr = match syn::parse::<C>(attr) {
        Ok(it) => it,
        Err(error) => return error.into_compile_error().into(),
    };
    let item = match syn::parse::<I>(item) {
        Ok(it) => it,
        Err(error) => return error.into_compile_error().into(),
    };
    return zz_runner::catching(move || ekran(attr, item)).into();
}

/// Parses `attr` and `item`; when `hide_in` is on, parses them again with
/// `self::`/`super::` paths re-anchored for the private module. Then expands
/// them with panics caught.
fn expand_hideable<C, I>(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
    parse_attr: impl Fn(proc_macro2::TokenStream) -> syn::Result<C>,
    ekran: impl FnOnce(C, I) -> syn::Result<proc_macro2::TokenStream> + UnwindSafe,
) -> proc_macro::TokenStream
where
    C: zz_hide::HideCfg + UnwindSafe,
    I: zz_hide::Hideable + UnwindSafe,
{
    let attr = proc_macro2::TokenStream::from(attr);
    let item = proc_macro2::TokenStream::from(item);
    let parsed = parse_attr(attr.clone()).and_then(|cfg| {
        return Ok((cfg, syn::parse2::<I>(item.clone())?));
    });
    let (cfg, parsed_item) = match parsed {
        Ok(it) => it,
        Err(error) => return error.into_compile_error().into(),
    };
    let (cfg, item) = match cfg.hide_in().module_for(parsed_item.ident()) {
        None => (cfg, parsed_item),
        Some(_) => match zz_hide::reparse(attr, item, parse_attr) {
            Ok(it) => it,
            Err(error) => return error.into_compile_error().into(),
        },
    };
    return zz_runner::catching(move || ekran(cfg, item)).into();
}
