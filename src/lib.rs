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

use crate::zz_profile_integral::ProfileKind;
use crate::zz_values::F;

pub(crate) mod bitflag;
pub(crate) mod constructor;
pub(crate) mod float;
pub(crate) mod friendship;
pub(crate) mod integral;
pub(crate) mod integral_cfg;
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
/// - `tree = NODE | [NODE, ...]`
/// - `hide_in = MODULE | none`
///
/// `PROFILE` is `int`, `circular`, `delta`, `sequential`, `wrapping`, or
/// `point`. A profile body accepts `with` and `without`; only `circular` also
/// accepts `precedes = bool`. `FACET` is one profile with an optional body that
/// accepts `with`, `without`, `friend`, and `produce`. `TREE` uses the
/// [`tree`] node grammar. Its private `MarkerTreeRoot` seal stays generated,
/// while its nodes are re-exported beside the annotated item.
///
/// By default the item and its generated code live in a private module named
/// `__` plus the snake-cased type name (`hide_in = MODULE` picks the name) and
/// are re-exported at the call site, so the tuple constructor and a private
/// field stay unreachable even from the defining module. `hide_in = none`
/// disables this; `get_raw` and a custom `_or_panic` require it.
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
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `math`, `error`, `friend`, `with`, `without`, `get_raw`, `produce`,
///   `profile` (`int` or `point`), `facet`, `tree`, and `hide_in`.
///
/// Floats share the integral configuration. Differences:
///
/// - `domain` uses float constants (`0.0..=1.0 & !{0.5} & is_finite`).
///   Ranges, rosters, and multiples reject NaN; their complements (`!{0.5}`)
///   admit it. A domain that rejects NaN makes the type NaN-free: it then gets
///   `Eq`, `Ord`, `Hash`, and `_cmp` when its value field is private. Other
///   types have none of them, and asking for them is an error.
/// - `math = core | std | nightly` selects where `floor`, `sqrt`, … come from
///   in domains and the `float_math` API: `std` uses inherent methods,
///   `nightly` uses `core::fNN::math` (no_std, `#![feature(core_float_math)]`).
/// - `Math` friends grant `Make` and arithmetic only; there are no bitwise
///   operations, `non_zero` storage, or `Step`.
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
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, `facet`,
///   `tree`, and `hide_in`.
///
/// This is equivalent to `#[typekin::integral(profile = int, ...)]`; it does
/// not accept `profile`.
#[proc_macro_attribute]
pub fn int(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Int, attr, item);
}

/// Generates an integral newtype with the fixed `circular` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, `facet`,
///   `tree`, `hide_in`, and `precedes = bool`.
///
/// This is equivalent to `#[typekin::integral(profile = circular, ...)]`.
/// `precedes` configures the selected circular profile directly; `profile` is
/// not accepted.
#[proc_macro_attribute]
pub fn circular(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Circular, attr, item);
}

/// Generates an integral newtype with the fixed `delta` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, `facet`,
///   `tree`, and `hide_in`.
///
/// This is equivalent to `#[typekin::integral(profile = delta, ...)]`; it does
/// not accept `profile` or `precedes`.
#[proc_macro_attribute]
pub fn delta(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Delta, attr, item);
}

/// Generates an integral newtype with the fixed `sequential` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, `facet`,
///   `tree`, and `hide_in`.
///
/// This is equivalent to `#[typekin::integral(profile = sequential, ...)]`; it
/// does not accept `profile` or `precedes`.
#[proc_macro_attribute]
pub fn sequential(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Sequential, attr, item);
}

/// Generates an integral newtype with the fixed `wrapping` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, `facet`,
///   `tree`, and `hide_in`.
///
/// This is equivalent to `#[typekin::integral(profile = wrapping, ...)]`; it
/// does not accept `profile` or `precedes`.
#[proc_macro_attribute]
pub fn wrapping(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Wrapping, attr, item);
}

/// Generates an integral or float newtype with the fixed `point` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`], or the `f32`/`f64` tuple struct accepted by [`float`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, `facet`,
///   `tree`, and `hide_in`; float fields also accept `math`.
///
/// This is equivalent to `#[typekin::integral(profile = point, ...)]` or
/// `#[typekin::float(profile = point, ...)]`; it does not accept `profile` or
/// `precedes`. Float points also omit `Neg`, `Sum`, `Product`, and the
/// `core_float`/`float_math` methods.
#[proc_macro_attribute]
pub fn point(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral(ProfileKind::Point, attr, item);
}

/// Generates a sealed hierarchy of marker traits and zero-sized leaf structs.
///
/// # Inputs
///
/// - **Required item:** a non-generic, safe, empty trait without supertraits.
///   It is the root seal.
/// - **Required attribute:** one or more marker nodes.
///
/// A node is `[visibility] Name` for a leaf or `[visibility] Name { CHILDREN }`
/// for a branch. Omitted visibility inherits from its parent; top-level nodes
/// inherit the root seal's visibility. Branches extend their parent and the
/// tree seal, while leaves implement every ancestor trait. Names must be unique
/// within the tree, and every branch needs at least one child.
///
/// ```text
/// #[typekin::tree(
///     pub Direction {
///         pub In,
///         pub Out,
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
/// - `tree = NODE | [NODE, ...]`
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
/// - `tree = NODE | [NODE, ...]`
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
        (Some(_), ProfileKind::Point) => Ok(integral_cfg::Kind::Float),
        (Some(span), ProfileKind::Int) => Err(syn::Error::new(
            span,
            "use #[typekin::float] for f32/f64 value fields",
        )),
        (Some(span), profile) => Err(syn::Error::new(
            span,
            format!(
                "the {} profile requires an integer value field",
                profile.name()
            ),
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
                .is_some_and(|it| F::of(it.to_string()).is_some()) =>
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
        Some(_) => {
            match zz_hide::reparse(attr, item, parsed_item, parse_attr) {
                Ok(it) => it,
                Err(error) => return error.into_compile_error().into(),
            }
        }
    };
    return zz_runner::catching(move || ekran(cfg, item)).into();
}
