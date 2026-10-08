#[cfg(test)]
mod tests {
    #[typekin::friends(
        relation = DocumentParts,
        friends = [
            document_parts(Self) -> [],
            reader_parts(Reader) -> [DocumentInspector],
            writer_parts(Writer) -> [DocumentInspector],
        ],
    )]
    pub struct Document {
        id: u32,
        access: u8,
    }

    #[typekin::friends(
        relation = (u32, u8),
        mod = pub(crate) declared_document_protocol,
        extra_caps = [DocumentDeclared],
    )]
    struct DocumentProtocol;

    #[typekin::friends(
        relation = DirectParts,
        scope = pub(crate) direct_friendship,
        friends = [],
    )]
    pub struct Direct;
    #[derive(Copy, Clone)]
    struct DirectParts;

    #[typekin::friends(
        relation = SelfScopedParts,
        seal = SelfScopedSeal,
        scope = self,
        friends = [],
    )]
    struct SelfScoped;

    struct SelfScopedParts;

    impl Direct {
        fn of_parts(_: DirectParts) -> Self {
            return Self;
        }
    }

    #[allow(dead_code)]
    impl DocumentProtocol {
        fn of_parts(_: (u32, u8)) -> Self {
            return Self;
        }
    }

    impl SelfScoped {
        fn of_parts(_: SelfScopedParts) -> Self {
            return Self;
        }
    }

    #[derive(Copy, Clone)]
    pub struct DocumentParts {
        id: u32,
        access: u8,
    }

    impl Document {
        fn of_parts(parts: DocumentParts) -> Self {
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

    impl declared_document_protocol::Seal for Reader {}

    impl declared_document_protocol::DocumentDeclared for Reader {
        fn convert(self) -> (u32, u8) {
            let parts = reader_parts(self);
            (parts.id, parts.access)
        }
    }

    #[test]
    fn standalone_marker_gates_owned_document_construction() {
        fn construct_declared<T>(source: T) -> Document
        where
            T: declared_document_protocol::DocumentDeclared,
        {
            let (id, access) =
                declared_document_protocol::DocumentDeclared::convert(source);
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

        let document = Document::of(Writer { id: 13 });

        assert_eq!(document.id, 13);
        assert_eq!(document.access, 0b010);
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

    #[typekin::friends(
        relation = ::core::primitive::u128,
        friends = [
            make_parts(CapabilitySpecificSource) -> [Make],
            bit_parts(CapabilitySpecificSource) -> [Bit],
        ],
        scope = capability_specific_protocol,
    )]
    pub struct CapabilitySpecificTarget(u128);

    struct CapabilitySpecificSource(u128);

    fn make_parts(source: CapabilitySpecificSource) -> u128 {
        return source.0 / 4;
    }

    fn bit_parts(source: CapabilitySpecificSource) -> u128 {
        return source.0 / 2;
    }

    impl CapabilitySpecificTarget {
        fn of_parts(value: u128) -> Self {
            return Self(value);
        }
    }

    #[test]
    fn capabilities_use_their_own_conversions() {
        assert_eq!(
            capability_specific_protocol::Bit::to_u128(
                CapabilitySpecificSource(12)
            ),
            6,
        );
        assert_eq!(
            CapabilitySpecificTarget::of(CapabilitySpecificSource(12)).0,
            3,
        );
    }

    #[test]
    fn friends_generate_constructor_for_the_relationship() {
        let _ = Direct::of(DirectParts);
    }

    #[test]
    fn self_scope_exposes_the_friendship_protocol() {
        fn requires_seal<T: SelfScopedSeal>(_: T) {}

        requires_seal(SelfScopedParts);
        let _ = SelfScoped::of(SelfScopedParts);
    }

    #[typekin::friends(
        relation = ScopedDocumentParts,
        scope = pub(crate) scoped_document_protocol,
        friends = scoped_document_parts(Self) -> [],
    )]
    pub struct ScopedDocument;

    pub struct ScopedDocumentParts;

    impl ScopedDocument {
        fn of_parts(_: ScopedDocumentParts) -> Self {
            return Self;
        }
    }

    fn scoped_document_parts(_: ScopedDocument) -> ScopedDocumentParts {
        return ScopedDocumentParts;
    }

    #[test]
    fn constructor_module_contains_friendship_and_constructor_implementations()
    {
        let _ = ScopedDocument::of(ScopedDocumentParts);
    }

    #[typekin::friends(
        relation = ConstDocumentParts,
        scope = _,
        friends = [],
    )]
    struct ConstDocument;

    struct ConstDocumentParts;

    impl ConstDocument {
        fn of_parts(_: ConstDocumentParts) -> Self {
            return Self;
        }
    }

    #[test]
    fn constructor_const_scope_contains_its_implementations() {
        let _ = ConstDocument::of(ConstDocumentParts);
    }

    #[typekin::friends(
        relation = ::std::string::String,
        mod = pub(crate) trusted_title_protocol,
        friends = title_conversions::owned_title(OwnedTitle,) -> Trust,
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
        fn of_parts(title: String) -> Self {
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
        assert_panics(|| construct_trusted_title(OwnedTitle(String::new())));
    }

    mod trusted_inspector {
        use std::cell::Cell;
        use std::rc::Rc;

        #[typekin::friends(
            relation = ::std::string::String,
            mod = pub(crate) inspector_protocol,
            extra_caps = [ReservedScalar, ReservedFirst, ReservedSecond],
            friends = [
                inspector_conversions::inspector_text(InspectorSource) -> [Inspect, Trust],
            ],
        )]
        struct Inspectable;

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
            pub(super) fn inspector_text(
                source: super::InspectorSource
            ) -> String {
                let super::InspectorSource {
                    text,
                    lifetime: _lifetime,
                } = source;
                text
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

        fn inspect<T>(source: T) -> String
        where
            T: inspector_protocol::Inspect
                + inspector_protocol::Trust
                + inspector_protocol::ReservedScalar
                + inspector_protocol::ReservedFirst
                + inspector_protocol::ReservedSecond,
        {
            inspector_protocol::Inspect::to_string(source)
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

    pub fn assert_panics<T>(f: impl FnOnce() -> T + std::panic::UnwindSafe) {
        std::panic::set_hook(Box::new(move |_| {}));
        let result = std::panic::catch_unwind(f);
        let _ = std::panic::take_hook();
        assert!(result.is_err());
    }
}
