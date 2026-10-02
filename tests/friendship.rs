#[typekin::friendship(
    relation = DocumentParts,
    friends = [
        _ -> [DocumentReserved],
        document_parts(Self) -> [],
        reader_parts(Reader) -> [DocumentInspector],
        writer_parts(Writer) -> [DocumentInspector],
    ],
)]
#[typekin::constructor(
    of_relation = Self::from_parts,
    friends = [Self, Reader],
)]
pub struct Document {
    id: u32,
    access: u8,
}

#[typekin::friendship(
    relation = (u32, u8),
    mod = pub(crate) declared_document_protocol,
    friends = _ -> DocumentDeclared,
)]
struct DocumentProtocol;

#[typekin::constructor(
    of_relation = Self::from_parts,
    of_friend = from_friend,
)]
#[typekin::friendship(
    relation = DirectParts,
    mod = pub(crate) direct_friendship,
    friends = [_ -> []],
)]
pub struct Direct;
#[derive(Copy, Clone)]
struct DirectParts;

impl Direct {
    fn from_parts(_: DirectParts) -> Self {
        return Self;
    }
}

#[derive(Copy, Clone)]
pub struct DocumentParts {
    id: u32,
    access: u8,
}

impl Document {
    fn from_parts(parts: DocumentParts) -> Self {
        return Self {
            id: parts.id,
            access: parts.access,
        };
    }
}

fn document_parts(document: Document) -> DocumentParts {
    return DocumentParts {
        id: document.id,
        access: document.access,
    };
}

pub struct Reader {
    id: u32,
}

pub struct Writer {
    id: u32,
}

fn reader_parts(reader: Reader) -> DocumentParts {
    return DocumentParts {
        id: reader.id,
        access: 0b001,
    };
}

fn writer_parts(writer: Writer) -> DocumentParts {
    return DocumentParts {
        id: writer.id,
        access: 0b010,
    };
}

impl declared_document_protocol::Seal for Reader {
    fn convert(self) -> (u32, u8) {
        let parts = reader_parts(self);
        (parts.id, parts.access)
    }
}

impl declared_document_protocol::DocumentDeclared for Reader {}

#[test]
fn standalone_marker_gates_owned_document_construction() {
    fn construct_declared<T>(source: T) -> Document
    where
        T: declared_document_protocol::DocumentDeclared,
    {
        let (id, access) = declared_document_protocol::Seal::convert(source);
        Document::of(DocumentParts { id, access })
    }

    let _ = DocumentProtocol;
    let document = construct_declared(Reader { id: 5 });
    assert_eq!((document.id, document.access), (5, 0b001));
}

#[test]
fn constructs_from_friend_with_constructor_capability() {
    let document = Document::of(Reader { id: 7 });

    assert_eq!(document.id, 7);
    assert_eq!(document.access, 0b001);
}

#[test]
fn constructs_from_its_relation() {
    let document = Document::of(DocumentParts {
        id: 11,
        access: 0b100,
    });

    assert_eq!((document.id, document.access), (11, 0b100));
}

#[test]
fn constructor_self_alias_grants_target() {
    let document = Document::of(Document {
        id: 17,
        access: 0b011,
    });

    assert_eq!((document.id, document.access), (17, 0b011));
}

#[test]
fn friendship_consumes_its_friend() {
    let reader = Reader { id: 23 };

    let document = Document::of(reader);

    assert_eq!((document.id, document.access), (23, 0b001));
}

#[test]
fn constructor_dispatches_inner_friendship() {
    let _ = Direct::from_friend(DirectParts);
}

#[typekin::friendship(
    relation = ::std::string::String,
    mod = pub(crate) trusted_title_protocol,
    friends = title_conversions::owned_title(OwnedTitle,) -> Trust,
)]
#[typekin::constructor(
    of_relation = Self::from_title,
    friends = OwnedTitle,
)]
pub(crate) struct TrustDocument {
    title: String,
}

struct OwnedTitle(String);

mod title_conversions {
    pub(super) fn owned_title(source: super::OwnedTitle) -> String {
        source.0
    }
}

impl TrustDocument {
    fn from_title(title: String) -> Self {
        assert!(!title.is_empty(), "a document must have a title");
        Self {
            title: title.to_uppercase(),
        }
    }
}

fn construct_trusted_title<T>(source: T) -> TrustDocument
where
    T: trusted_title_protocol::Make + trusted_title_protocol::Trust,
{
    TrustDocument::of(source)
}

#[test]
fn trusted_constructor_consumes_friend_and_still_calls_of_relation() {
    let source = OwnedTitle(String::from("owned title"));
    let document = construct_trusted_title(source);
    assert_eq!(document.title, "OWNED TITLE");
    let relation = String::from("owned relation");
    assert_eq!(TrustDocument::of(relation).title, "OWNED RELATION");
    assert!(
        std::panic::catch_unwind(|| {
            construct_trusted_title(OwnedTitle(String::new()))
        })
        .is_err()
    );
}

mod trusted_inspector {
    use std::cell::Cell;
    use std::rc::Rc;

    #[typekin::friendship(
        relation = ::std::string::String,
        mod = pub(crate) inspector_protocol,
        friends = [
            _ -> ReservedScalar,
            _ -> [ReservedFirst, ReservedSecond],
            inspector_conversions::inspector_text(InspectorSource) -> [Inspect, Trust],
        ],
    )]
    struct Inspectable;

    const _: () = {
        // Do not remove this unless you are adding a direct usage of this
        // struct somewhere, else rust will warn about unused struct.
        let _ = Inspectable;
    };

    struct InspectorSource {
        text: String,
        lifetime: SourceLifetime,
    }

    struct SourceLifetime(Rc<Cell<usize>>);

    impl Drop for SourceLifetime {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    mod inspector_conversions {
        pub(super) fn inspector_text(source: super::InspectorSource) -> String {
            let super::InspectorSource {
                text,
                lifetime: _lifetime,
            } = source;
            text
        }
    }

    impl inspector_protocol::ReservedScalar for InspectorSource {}
    impl inspector_protocol::ReservedFirst for InspectorSource {}
    impl inspector_protocol::ReservedSecond for InspectorSource {}

    fn inspect<T>(source: T) -> String
    where
        T: inspector_protocol::Inspect
            + inspector_protocol::Trust
            + inspector_protocol::ReservedScalar
            + inspector_protocol::ReservedFirst
            + inspector_protocol::ReservedSecond,
    {
        inspector_protocol::Seal::to_string(source)
    }

    #[test]
    fn wildcard_capability_bounds_preserve_owned_qualified_conversion() {
        let drops = Rc::new(Cell::new(0));
        let source = InspectorSource {
            text: String::from("owned inspector title"),
            lifetime: SourceLifetime(Rc::clone(&drops)),
        };
        let pointer = source.text.as_ptr();
        let capacity = source.text.capacity();
        assert_eq!(drops.get(), 0);
        let title = inspect(source);
        assert_eq!(title, "owned inspector title");
        assert_eq!(title.as_ptr(), pointer);
        assert_eq!(title.capacity(), capacity);
        assert_eq!(drops.get(), 1);
    }
}
