#[typekin::friendship(
    relation = DocumentParts,
    friends = [
        _(conv = reader_parts, cap = [DocumentReserved]),
        Self(conv = document_parts, cap = []),
        Reader(conv = reader_parts, cap = [DocumentInspector]),
        Writer(conv = writer_parts, cap = [DocumentInspector]),
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
    friends = [_(conv = reader_parts, cap = [DocumentDeclared])],
)]
struct DocumentProtocol;

#[typekin::constructor(
    of_relation = Self::from_parts,
    of_friend = from_friend,
)]
#[typekin::friendship(
    relation = DirectParts,
    mod = pub(crate) direct_friendship,
    friends = [_(cap = [])],
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

fn document_parts(document: &Document) -> DocumentParts {
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

fn reader_parts(reader: &Reader) -> DocumentParts {
    return DocumentParts {
        id: reader.id,
        access: 0b001,
    };
}

fn writer_parts(writer: &Writer) -> DocumentParts {
    return DocumentParts {
        id: writer.id,
        access: 0b010,
    };
}

#[test]
fn friendship_without_constructor_compiles() {
    let _ = DocumentProtocol;
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
fn forwards_friendship_to_shared_and_mutable_references() {
    let reader = Reader { id: 23 };
    let mut mutable_reader = Reader { id: 31 };

    let shared_document = Document::of(&reader);
    let mutable_document = Document::of(&mut mutable_reader);

    assert_eq!((shared_document.id, shared_document.access), (23, 0b001));
    assert_eq!((mutable_document.id, mutable_document.access), (31, 0b001),);
}

#[test]
fn constructor_dispatches_inner_friendship() {
    let _ = Direct::from_friend(DirectParts);
}
