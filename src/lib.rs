extern crate core;

use syn::parse::Parser as _;
use syn::{
    Item,
    ItemEnum,
    ItemStruct,
    ItemTrait,
    parse_macro_input,
};

pub(crate) mod bitflag;
pub(crate) mod constructor;
pub(crate) mod friendship;
pub(crate) mod integral;
pub(crate) mod marker_tree;
pub(crate) mod text;
pub(crate) mod zz;
pub(crate) mod zz_conv;
pub(crate) mod zz_profile;
pub(crate) mod zz_runner;
pub(crate) mod zz_value_type;

#[cfg(test)]
mod tests;

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
/// - `valid = PATH | [PATH, ...]`, `domain = RANGE | [RANGE, ...]`, and
///   `error = ()`
/// - `friend = SPEC | [SPEC, ...]`, where `SPEC` is
///   `conversion(Source) -> CAPABILITY | [CAPABILITY, ...]`
/// - `with = FLAG | [FLAG, ...]` and `without = FLAG | [FLAG, ...]`
/// - `get_raw = PATH`
/// - `produce = CALLBACK(_) -> Type`
/// - `profile = PROFILE | [PROFILE, ...]`
/// - `facet = FACET | [FACET, ...]`
///
/// `PROFILE` is `int`, `circular`, `delta`, `sequential`, `wrapping`, or
/// `point`. A profile body accepts `with` and `without`; only `circular` also
/// accepts `precedes = bool`. `FACET` is one profile with an optional body that
/// accepts `with`, `without`, `friend`, and `produce`.
///
/// `FLAG` is an integral generation, operation, friendship, storage, or
/// `konst_*` flag. `Math` expands to `Make` and `Bit` for integral friends.
#[proc_macro_attribute]
pub fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = Box::new(parse_macro_input!(attr as integral::Cfg));
    let item = parse_macro_input!(item as ItemStruct);

    return integral::integral(attr, item);
}

/// Generates an integral newtype with the fixed `int` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, and `facet`.
///
/// This is equivalent to `#[typekin::integral(profile = int, ...)]`; it does
/// not accept `profile`.
#[proc_macro_attribute]
pub fn int(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral("int", attr, item);
}

/// Generates an integral newtype with the fixed `circular` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, `facet`, and
///   `precedes = bool`.
///
/// This is equivalent to `#[typekin::integral(profile = circular, ...)]`.
/// `precedes` configures the selected circular profile directly; `profile` is
/// not accepted.
#[proc_macro_attribute]
pub fn circular(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral("circular", attr, item);
}

/// Generates an integral newtype with the fixed `delta` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, and `facet`.
///
/// This is equivalent to `#[typekin::integral(profile = delta, ...)]`; it does
/// not accept `profile` or `precedes`.
#[proc_macro_attribute]
pub fn delta(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral("delta", attr, item);
}

/// Generates an integral newtype with the fixed `sequential` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, and `facet`.
///
/// This is equivalent to `#[typekin::integral(profile = sequential, ...)]`; it
/// does not accept `profile` or `precedes`.
#[proc_macro_attribute]
pub fn sequential(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral("sequential", attr, item);
}

/// Generates an integral newtype with the fixed `wrapping` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, and `facet`.
///
/// This is equivalent to `#[typekin::integral(profile = wrapping, ...)]`; it
/// does not accept `profile` or `precedes`.
#[proc_macro_attribute]
pub fn wrapping(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral("wrapping", attr, item);
}

/// Generates an integral newtype with the fixed `point` profile.
///
/// # Inputs
///
/// - **Required item:** the same primitive-integer tuple struct accepted by
///   [`integral`].
/// - All configuration arguments are optional: `konst`, `valid`, `domain`,
///   `error`, `friend`, `with`, `without`, `get_raw`, `produce`, and `facet`.
///
/// This is equivalent to `#[typekin::integral(profile = point, ...)]`; it does
/// not accept `profile` or `precedes`.
#[proc_macro_attribute]
pub fn point(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return profiled_integral("point", attr, item);
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
/// for a branch. Branches become traits extending their parent; leaves become
/// unit structs implementing every ancestor trait. Names must be unique within
/// the tree, and every branch needs at least one child.
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
    let attr = parse_macro_input!(attr as marker_tree::Cfg);
    let item = parse_macro_input!(item as ItemTrait);

    return marker_tree::marker_tree(attr, item);
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
///
/// `SPEC` uses the friend conversion grammar. The bracketed `integral`
/// configuration accepts default-profile integral validation, friend, flag, and
/// accessor settings; it rejects `profile`, `facet`, `produce`, and nonzero
/// storage.
#[proc_macro_attribute]
pub fn bitflag(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = Box::new(parse_macro_input!(attr as bitflag::Cfg));
    let item = parse_macro_input!(item as ItemEnum);

    return bitflag::bitflag(attr, item);
}

/// Generates a validated `String` newtype.
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
/// - `valid = PATH | [PATH, ...]` or `domain = ["literal", ...]`
/// - `error = ()`
/// - `friend = SPEC | [SPEC, ...]`
/// - `with = FLAG | [FLAG, ...]` and `without = FLAG | [FLAG, ...]`
///
/// `valid` and `domain` are mutually exclusive. Text friends accept `Make`,
/// `unsafe`, and self-only `Rel`; `Make` needs a conversion callback. Set
/// `std = false` for an `alloc`-backed generated API.
#[proc_macro_attribute]
pub fn text(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as text::Cfg);
    let item = parse_macro_input!(item as ItemStruct);

    return text::text(attr, item);
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
    let attr = parse_macro_input!(attr as friendship::Cfg);
    let item = parse_macro_input!(item as Item);

    return friendship::ekran(attr, item);
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
    let attr = parse_macro_input!(attr as constructor::Cfg);
    let item = parse_macro_input!(item as Item);
    return constructor::ekran(attr, item);
}

fn profiled_integral(
    profile: &'static str,
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parser = |input: syn::parse::ParseStream| {
        return integral::Cfg::parse_with_profile(input, profile);
    };
    let attr = match parser.parse(attr) {
        Ok(attr) => Box::new(attr),
        Err(error) => return error.into_compile_error().into(),
    };
    let item = match syn::parse::<ItemStruct>(item) {
        Ok(item) => item,
        Err(error) => return error.into_compile_error().into(),
    };

    return integral::integral(attr, item);
}
