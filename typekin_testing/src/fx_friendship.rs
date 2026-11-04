use crate::*;

#[typekin::friends(
    relation = DocumentParts,
    friend = [
        document_parts(Self) -> [],
        reader_parts(Reader) -> [DocumentInspector],
        writer_parts(Writer) -> [DocumentInspector],
    ],
)]
pub struct Document {
    id: u32,
    access: u8,
}

impl Document {
    pub fn of_parts(parts: DocumentParts) -> Self {
        return Self {
            id: parts.id,
            access: parts.access,
        };
    }
}

pub(crate) fn make_document(
    id: u32,
    access: u8,
) -> Document {
    return Document { id, access };
}

pub(crate) fn document_id(document: &Document) -> u32 {
    return document.id;
}

pub(crate) fn document_access(document: &Document) -> u8 {
    return document.access;
}

#[typekin::friends(
    relation = (u32, u8),
    mod = pub declared_document_protocol,
    extra_caps = [DocumentDeclared],
)]
pub struct DocumentProtocol;

#[allow(dead_code)]
impl DocumentProtocol {
    pub fn of_parts(_: (u32, u8)) -> Self {
        return Self;
    }
}

#[typekin::friends(
    relation = DirectParts,
    scope = pub direct_friendship,
    friend = [],
)]
pub struct Direct;

impl Direct {
    pub fn of_parts(_: DirectParts) -> Self {
        return Self;
    }
}

#[derive(Copy, Clone)]
pub struct DirectParts;

#[derive(Copy, Clone)]
pub struct DocumentParts {
    pub(crate) id: u32,
    pub(crate) access: u8,
}

pub fn document_parts(document: Document) -> DocumentParts {
    return DocumentParts {
        id: document.id,
        access: document.access,
    };
}

pub struct Reader {
    pub(crate) id: u32,
}

pub struct Writer {
    pub(crate) id: u32,
}

pub fn reader_parts(reader: Reader) -> DocumentParts {
    return DocumentParts {
        id: reader.id,
        access: 0b001,
    };
}

pub fn writer_parts(writer: Writer) -> DocumentParts {
    return DocumentParts {
        id: writer.id,
        access: 0b010,
    };
}

impl declared_document_protocol::Seal for Reader {}

impl declared_document_protocol::DocumentDeclared for Reader {
    fn convert(self) -> (u32, u8) {
        let parts = reader_parts(self);
        (parts.id, parts.access)
    }
}

// ==============================================================================

#[typekin::friends(
    relation = ::core::primitive::u128,
    friend = [
        make_parts(CapabilitySpecificSource) -> Make,
        bit_parts(CapabilitySpecificSource) -> Bit,
    ],
    scope = capability_specific_protocol,
)]
pub struct CapabilitySpecificTarget(u128);

pub struct CapabilitySpecificSource(pub(crate) u128);

pub fn make_parts(source: CapabilitySpecificSource) -> u128 {
    return source.0 / 4;
}

pub fn bit_parts(source: CapabilitySpecificSource) -> u128 {
    return source.0 / 2;
}

impl CapabilitySpecificTarget {
    pub fn of_parts(value: u128) -> Self {
        return Self(value);
    }
}

pub(crate) fn capability_specific_bit(
    source: CapabilitySpecificSource
) -> u128 {
    return capability_specific_protocol::Bit::to_u128(source);
}

pub(crate) fn capability_specific_value(
    target: &CapabilitySpecificTarget
) -> u128 {
    return target.0;
}

pub(crate) mod qualified_sources {
    pub(crate) struct Source(pub(crate) u8);
}

pub(crate) fn qualified_make(source: qualified_sources::Source) -> u8 {
    return source.0 + 1;
}

pub(crate) fn qualified_bit(source: qualified_sources::Source) -> u8 {
    return source.0 + 2;
}

#[typekin::friends(
    relation = u8,
    friend = [
        qualified_make(qualified_sources::Source) -> Make,
        qualified_bit(qualified_sources::Source) -> Bit,
    ],
    scope = qualified_friend_protocol,
)]
pub struct QualifiedTarget(u8);

impl QualifiedTarget {
    fn of_parts(value: u8) -> Self {
        return Self(value);
    }
}

pub(crate) fn qualified_friend_bit(source: qualified_sources::Source) -> u8 {
    return qualified_friend_protocol::Bit::to_u8(source);
}

pub(crate) fn qualified_value(target: &QualifiedTarget) -> u8 {
    return target.0;
}

// ==============================================================================

#[typekin::friends(
    relation = ScopedDocumentParts,
    scope = pub scoped_document_protocol,
    friend = scoped_document_parts(Self) -> [],
)]
pub struct ScopedDocument;

pub struct ScopedDocumentParts;

impl ScopedDocument {
    pub fn of_parts(_: ScopedDocumentParts) -> Self {
        return Self;
    }
}

pub fn scoped_document_parts(_: ScopedDocument) -> ScopedDocumentParts {
    return ScopedDocumentParts;
}

// ==============================================================================

#[typekin::friends(relation = ConstDocumentParts)]
pub struct ConstDocument;

pub struct ConstDocumentParts;

impl ConstDocument {
    pub fn of_parts(_: ConstDocumentParts) -> Self {
        return Self;
    }
}

// ==============================================================================

#[typekin::friends(
    relation = String,
    mod = pub blind_trust_title_protocol,
    friend = title_conversions::owned_title(OwnedTitle,) -> unsafe,
)]
pub struct BlindTrustDocument {
    title: String,
}

pub struct OwnedTitle(pub(crate) String);

pub(crate) mod title_conversions {
    pub(crate) fn owned_title(source: super::OwnedTitle) -> String {
        source.0
    }
}

impl BlindTrustDocument {
    pub fn of_parts(title: String) -> Self {
        assert!(!title.is_empty(), "a document must have a title");
        Self {
            title: title.to_uppercase(),
        }
    }
}

pub(crate) fn blind_trust_title(document: &BlindTrustDocument) -> &str {
    return &document.title;
}

#[allow(private_bounds)]
pub(crate) fn construct_blind_trust_title<T>(source: T) -> BlindTrustDocument
where
    T: blind_trust_title_protocol::Make + blind_trust_title_protocol::Unsafe,
{
    BlindTrustDocument::of(source)
}

// ==============================================================================

pub mod inspector_conversions {
    pub fn inspector_text(source: super::InspectorSource) -> String {
        let super::InspectorSource {
            text,
            lifetime: _lifetime,
        } = source;
        text
    }
}

#[typekin::friends(
    relation = String,
    mod = pub inspector_protocol,
    extra_caps = [ReservedScalar, ReservedFirst, ReservedSecond],
    friend = [
        inspector_conversions::inspector_text(InspectorSource) -> [Inspect, unsafe],
    ],
)]
pub struct Inspectable;

#[allow(dead_code)]
impl Inspectable {
    fn of_parts(_: String) -> Self {
        return Self;
    }
}

const _: () = {
    // Do not remove this unless you are adding a direct usage of this
    // struct somewhere, else rust will warn about unused struct.
    let _ = Inspectable;
};

pub struct InspectorSource {
    pub(crate) text: String,
    pub(crate) lifetime: SourceLifetime,
}

pub struct SourceLifetime(pub(crate) std::rc::Rc<std::cell::Cell<usize>>);

impl Drop for SourceLifetime {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

impl inspector_protocol::ReservedScalar for InspectorSource {
    fn to_string(self) -> String {
        return inspector_conversions::inspector_text(self);
    }
}

impl inspector_protocol::ReservedFirst for InspectorSource {
    fn to_string(self) -> String {
        return inspector_conversions::inspector_text(self);
    }
}

impl inspector_protocol::ReservedSecond for InspectorSource {
    fn to_string(self) -> String {
        return inspector_conversions::inspector_text(self);
    }
}

#[allow(private_bounds)]
pub(crate) fn inspect<T>(source: T) -> String
where
    T: inspector_protocol::Inspect
        + inspector_protocol::Unsafe
        + inspector_protocol::ReservedScalar
        + inspector_protocol::ReservedFirst
        + inspector_protocol::ReservedSecond,
{
    inspector_protocol::Inspect::to_string(source)
}

#[allow(private_bounds)]
pub(crate) fn construct_declared<T>(source: T) -> Document
where
    T: declared_document_protocol::DocumentDeclared,
{
    let (id, access) =
        declared_document_protocol::DocumentDeclared::convert(source);
    Document::of(DocumentParts { id, access })
}

// ==============================================================================

#[typekin::friends(
        relation = SelfScopedParts,
        seal = SelfScopedSeal,
        scope = self,
        friend = [],
)]
pub struct SelfScoped;

pub struct SelfScopedParts;

impl SelfScoped {
    pub fn of_parts(_: SelfScopedParts) -> Self {
        return Self;
    }
}

#[allow(private_bounds)]
pub(crate) fn requires_self_scoped_seal<T: SelfScopedSeal>(_: T) {}

// ==============================================================================

pub mod rooted_sources {
    pub struct Source(pub u8);

    pub fn convert(source: Source) -> u8 {
        return source.0 * 3;
    }
}

#[typekin::friends(
    relation = u8,
    scope = rooted_friend_protocol,
    friend = crate::fx_friendship::rooted_sources::convert(
        crate::fx_friendship::rooted_sources::Source
    ) -> Inspect,
)]
pub struct RootedTarget(u8);

impl RootedTarget {
    fn of_parts(value: u8) -> Self {
        return Self(value);
    }
}

pub(crate) fn rooted_friend_inspect(source: rooted_sources::Source) -> u8 {
    return rooted_friend_protocol::Inspect::to_u8(source);
}

pub(crate) fn rooted_value(target: &RootedTarget) -> u8 {
    return target.0;
}

// ==============================================================================

pub type Reading = u16;

pub struct Sensor(pub(crate) u16);

pub(crate) fn sensor_reading(sensor: Sensor) -> u16 {
    return sensor.0;
}

#[typekin::friends(
    relation = Reading,
    value = u16,
    conversion = into_value,
    scope = sensor_protocol,
    friend = sensor_reading(Sensor) -> Inspect,
)]
pub struct Gauge(u16);

impl Gauge {
    fn of_parts(reading: u16) -> Self {
        return Self(reading);
    }
}

pub(crate) fn sensor_into_value() -> fn(Sensor) -> u16 {
    return <Sensor as sensor_protocol::Inspect>::into_value;
}

pub(crate) fn gauge_value(gauge: &Gauge) -> u16 {
    return gauge.0;
}

// ==============================================================================

pub mod constructor_sources {
    pub struct Stub(pub u32);

    pub struct Pass(pub u32);

    pub fn stub_number(stub: Stub) -> u32 {
        return stub.0;
    }

    pub fn pass_number(pass: Pass) -> u32 {
        return pass.0 + 100;
    }
}

pub(crate) mod self_scoped_constructor {
    #[typekin::constructor(
        friend = crate::fx_friendship::constructor_sources::Stub,
        relationship = Self::new,
        scope = self,
    )]
    #[typekin::friendship(
        relation = u32,
        friend = crate::fx_friendship::constructor_sources::stub_number(
            crate::fx_friendship::constructor_sources::Stub
        ) -> Inspect,
    )]
    pub struct Ticket(pub(crate) u32);

    impl Ticket {
        fn new(number: u32) -> Self {
            return Self(number);
        }
    }
}

#[typekin::constructor(
    relationship = Self::new,
    friend = constructor_sources::Stub,
    maker = from_source,
    scope = _,
)]
#[typekin::friendship(
    relation = u32,
    friend = constructor_sources::stub_number(
        constructor_sources::Stub
    ) -> Inspect,
)]
pub struct MadeTicket(u32);

impl MadeTicket {
    fn new(number: u32) -> Self {
        return Self(number);
    }
}

pub(crate) fn made_ticket_value(ticket: &MadeTicket) -> u32 {
    return ticket.0;
}

pub fn construct_routed(number: u32) -> RoutedTicket {
    return RoutedTicket(number * 2);
}

#[typekin::constructor(
    relationship = crate::fx_friendship::construct_routed,
    maker = from_source,
    friend = constructor_sources::Stub,
    scope = pub(crate) routed_constructors,
)]
#[typekin::friendship(
    relation = u32,
    friend = constructor_sources::stub_number(
        constructor_sources::Stub
    ) -> Inspect,
)]
pub struct RoutedTicket(u32);

pub(crate) fn routed_inspect(source: constructor_sources::Stub) -> u32 {
    return routed_constructors::Inspect::to_u32(source);
}

pub(crate) fn routed_ticket_value(ticket: &RoutedTicket) -> u32 {
    return ticket.0;
}

pub(crate) mod multi_friend_constructor {
    use super::constructor_sources::{
        Pass,
        Stub,
        pass_number,
        stub_number,
    };

    #[typekin::constructor(
        relationship = Self::new,
        friend = [Stub, Pass],
        maker = from_source,
        scope = self,
    )]
    #[typekin::friendship(
        relation = u32,
        friend = [
            stub_number(Stub) -> Inspect,
            pass_number(Pass) -> Inspect,
        ],
    )]
    pub struct Gate(pub(crate) u32);

    impl Gate {
        fn new(number: u32) -> Self {
            return Self(number);
        }
    }
}

#[typekin::constructor(
    relationship = Self::new,
    friend = [],
    maker = from_source,
    scope = booth_protocol,
)]
#[typekin::friendship(
    relation = u32,
    friend = constructor_sources::stub_number(
        constructor_sources::Stub
    ) -> Inspect,
)]
pub struct Booth(u32);

impl Booth {
    fn new(number: u32) -> Self {
        return Self(number);
    }
}

pub(crate) fn booth_inspect(source: constructor_sources::Stub) -> u32 {
    return booth_protocol::Inspect::to_u32(source);
}

pub(crate) fn booth_value(booth: &Booth) -> u32 {
    return booth.0;
}

pub(crate) fn assert_stub_is_not_booth_make() {
    assert_not_impl!(constructor_sources::Stub: booth_protocol::Make);
}
