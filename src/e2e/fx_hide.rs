use crate::*;

// Private items of the defining module, used from inside the hidden module.
const fn is_even(raw: u8) -> bool {
    return raw % 2 == 0;
}

struct Source(u8);

impl Source {
    fn into_raw(self) -> u8 {
        return self.0;
    }
}

#[typekin::integral(
    konst = none,
    valid = is_even,
    friend = Source::into_raw(Source) -> Make,
)]
pub struct Even(u8);

/// Converts `raw` through `Even`'s friend, the private `Source::into_raw`.
pub(crate) fn even_of_source(raw: u8) -> Even {
    return Even::of(Source(raw));
}

#[typekin::integral(
    konst = none,
    produce = Self::into_wide(_) -> Wide,
)]
pub(crate) struct Narrow(u8);

impl Narrow {
    fn into_wide(self) -> Wide {
        return Wide::make(u16::from(self.raw()));
    }
}

#[typekin::integral(konst = none)]
pub(crate) struct Wide(u16);

#[typekin::integral(
    konst = none,
    tree = Unit {
        Meters,
        Feet,
    },
)]
pub(crate) struct Length<U: Unit>(u32);

#[typekin::integral(konst = none, hide_in = custom_module)]
pub(crate) struct Renamed(u8);

/// Makes a `Renamed` through its `hide_in` module, private to this module.
pub(crate) fn renamed_in_custom_module(raw: u8) -> Renamed {
    return custom_module::Renamed::make(raw);
}

#[typekin::integral(konst = none, hide_in = r#none)]
pub(crate) struct RawNone(u8);

/// Makes a `RawNone` through its `hide_in` module, private to this module.
pub(crate) fn raw_none_in_none_module(raw: u8) -> RawNone {
    return none::RawNone::make(raw);
}

#[typekin::integral(konst = none, hide_in = none)]
pub(crate) struct Exposed(u8);

/// Builds an `Exposed` with the tuple constructor, reachable only here.
pub(crate) fn exposed_from_constructor(raw: u8) -> Exposed {
    return Exposed(raw);
}

/// Reads the value field of `exposed`, reachable only here.
pub(crate) fn exposed_field(exposed: &Exposed) -> u8 {
    return exposed.0;
}

#[typekin::integral(konst = none)]
pub(crate) struct PageHeader(u8);

/// Makes a `PageHeader` through its default `hide_in` module, private to this
/// module.
pub(crate) fn page_header_in_default_module(raw: u8) -> PageHeader {
    return __page_header::PageHeader::make(raw);
}

#[typekin::integral(konst = none)]
pub(crate) struct HTTPCode(u16);

/// Makes an `HTTPCode` through its default `hide_in` module, private to this
/// module.
pub(crate) fn http_code_in_default_module(raw: u16) -> HTTPCode {
    return __http_code::HTTPCode::make(raw);
}

#[typekin::text(konst = none)]
pub(crate) struct r#Type(String);

/// Parses a `Type` through its default `hide_in` module, private to this
/// module.
pub(crate) fn type_in_default_module(raw: &str) -> Result<r#Type, ()> {
    return __type::Type::try_from_str(raw);
}

#[typekin::integral(konst = none, valid = self::is_even)]
pub(crate) struct SelfChecked(u8);

#[typekin::integral(konst = none, valid = crate::fx_hide::is_even)]
pub(crate) struct CrateChecked(u8);

#[typekin::bitflag(repr = u8)]
pub(crate) enum Access {
    Read = 0b01,
    Write = 0b10,
}

#[typekin::text(domain = {"draft", "final"})]
pub(crate) struct Stage(String);

pub(crate) mod nested {
    pub(super) const fn is_small(raw: u8) -> bool {
        return raw < 10;
    }

    #[typekin::integral(konst = none, valid = super::nested::is_small)]
    pub(super) struct Small(u8);

    /// Validates `raw` as a `Small`, which is visible only in the parent.
    pub(crate) fn try_make_small(raw: u8) -> Result<u8, u8> {
        return Small::try_make(raw).map(Small::raw);
    }

    #[typekin::integral(
        konst = none,
        friend = super::Source::into_raw(super::Source) -> Make,
    )]
    pub(super) struct Relayed(u8);

    /// Converts `raw` through `Relayed`'s friend, the parent's private
    /// `Source::into_raw`, and reads the result back.
    pub(crate) fn relayed_of_source(raw: u8) -> u8 {
        return Relayed::of(super::Source(raw)).raw();
    }

    /// Items restricted by a path that resolves only from `deeper` itself:
    /// `super::super::nested` is this module, but re-anchored one module
    /// too many it names a missing `crate::nested`.
    pub(crate) mod deeper {
        #[typekin::integral(
            konst = none,
            tree = pub(in super::super::nested) Reach {
                Far,
            },
        )]
        pub(super) struct Reached(u8);

        #[typekin::packed(repr = u8)]
        pub(super) struct Shelf {
            #[bits(0..8)]
            pub(in super::super::nested) item: u8,
        }
    }

    fn accepts_reach<T: deeper::Reach>() {}

    /// Names `Far` here, where `Reach`'s restriction reaches.
    pub(crate) fn far_has_the_reach_visibility() {
        accepts_reach::<deeper::Far>();
    }

    /// Stores and reads `raw` through `Shelf`'s field, whose restriction
    /// reaches here.
    pub(crate) fn shelf_item(raw: u8) -> u8 {
        return deeper::Shelf::make(deeper::ShelfValues { item: raw }).item();
    }
}
