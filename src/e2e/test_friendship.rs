use crate::fx_friendship::*;
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standalone_marker_gates_owned_document_construction() {
        let _ = DocumentProtocol;
        let document = construct_declared(Reader { id: 5 });
        assert_eq!(
            (document_id(&document), document_access(&document)),
            (5, 0b001)
        );
    }

    #[test]
    fn constructs_from_friend_with_constructor_capability() {
        let document = Document::of(Reader { id: 7 });

        assert_eq!(document_id(&document), 7);
        assert_eq!(document_access(&document), 0b001);

        let document = Document::of(Writer { id: 13 });

        assert_eq!(document_id(&document), 13);
        assert_eq!(document_access(&document), 0b010);
    }

    #[test]
    fn constructs_from_its_relation() {
        let document = Document::of(DocumentParts {
            id: 11,
            access: 0b100,
        });

        assert_eq!(
            (document_id(&document), document_access(&document)),
            (11, 0b100)
        );
    }

    #[test]
    fn constructor_self_alias_grants_target() {
        let document = Document::of(make_document(17, 0b011));

        assert_eq!(
            (document_id(&document), document_access(&document)),
            (17, 0b011)
        );
    }

    #[test]
    fn friendship_consumes_its_friend() {
        let reader = Reader { id: 23 };

        let document = Document::of(reader);

        assert_eq!(
            (document_id(&document), document_access(&document)),
            (23, 0b001)
        );
    }

    #[test]
    fn capabilities_use_their_own_conversions() {
        assert_eq!(capability_specific_bit(CapabilitySpecificSource(12)), 6,);
        assert_eq!(
            capability_specific_value(&CapabilitySpecificTarget::of(
                CapabilitySpecificSource(12)
            )),
            3,
        );
    }

    #[test]
    fn qualified_friend_capabilities_share_source_identity() {
        let value = 4;
        let source = qualified_sources::Source(value);
        let constructed = QualifiedTarget::of(source);
        assert_eq!(qualified_value(&constructed), value + 1);

        let source = qualified_sources::Source(value);
        let converted = qualified_friend_bit(source);
        assert_eq!(converted, value + 2);
    }

    #[test]
    fn friends_generate_constructor_for_the_relationship() {
        let _ = Direct::of(DirectParts);
    }

    #[test]
    fn constructor_module_contains_friendship_and_constructor_implementations()
    {
        let _ = ScopedDocument::of(ScopedDocumentParts);
    }

    #[test]
    fn constructor_const_scope_contains_its_implementations() {
        let _ = ConstDocument::of(ConstDocumentParts);
    }

    #[test]
    fn blind_trust_constructor_consumes_friend_and_still_calls_of_relation() {
        let source = OwnedTitle(String::from("owned title"));
        let document = construct_blind_trust_title(source);
        assert_eq!(blind_trust_title(&document), "OWNED TITLE");
        let relation = String::from("owned relation");
        assert_eq!(
            blind_trust_title(&BlindTrustDocument::of(relation)),
            "OWNED RELATION"
        );
        assert_panics(|| {
            construct_blind_trust_title(OwnedTitle(String::new()))
        });
    }

    #[test]
    fn wildcard_capability_bounds_preserve_owned_qualified_conversion() {
        let drops = std::rc::Rc::new(std::cell::Cell::new(0));
        let source = InspectorSource {
            text: String::from("owned inspector title"),
            lifetime: SourceLifetime(std::rc::Rc::clone(&drops)),
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

    #[test]
    fn self_scope_exposes_the_friendship_protocol() {
        requires_self_scoped_seal(SelfScopedParts);
        let _ = SelfScoped::of(SelfScopedParts);
    }

    #[test]
    fn crate_rooted_friend_paths_resolve_from_the_protocol_module() {
        let value = 5;

        let converted = rooted_friend_inspect(rooted_sources::Source(value));
        let constructed = RootedTarget::of(rooted_sources::Source(value));

        assert_eq!(converted, value * 3, "{value}");
        assert_eq!(rooted_value(&constructed), value * 3, "{value}");
    }

    #[test]
    fn conversion_names_the_capability_method() {
        let reading = 42;

        let converted = sensor_into_value()(Sensor(reading));

        assert_eq!(converted, reading, "{reading}");
    }

    #[test]
    fn value_is_the_capability_conversion_result() {
        let reading = 42;
        let convert: fn(Sensor) -> u16 = sensor_into_value();

        let converted = convert(Sensor(reading));
        let gauge = Gauge::of(Sensor(reading));

        assert_eq!(converted, reading, "{reading}");
        assert_eq!(gauge_value(&gauge), reading, "{reading}");
    }

    #[test]
    fn constructor_builds_friends_through_its_relationship() {
        let number = 7;

        let ticket = self_scoped_constructor::Ticket::of(
            constructor_sources::Stub(number),
        );

        assert_eq!(ticket.0, number, "{number}");
    }

    #[test]
    fn constructor_maker_names_the_constructor() {
        let number = 9;

        let ticket = MadeTicket::from_source(constructor_sources::Stub(number));

        assert_eq!(made_ticket_value(&ticket), number, "{number}");
        assert_no_associated_fn!(MadeTicket, of);
    }

    #[test]
    fn constructor_relationship_can_be_a_free_function() {
        let number = 11;

        let ticket =
            RoutedTicket::from_source(constructor_sources::Stub(number));

        assert_eq!(routed_ticket_value(&ticket), number * 2, "{number}");
    }

    #[test]
    fn constructor_scope_module_holds_the_protocol() {
        let number = 13;

        let converted = routed_inspect(constructor_sources::Stub(number));

        assert_eq!(converted, number, "{number}");
    }

    #[test]
    fn constructor_accepts_each_listed_friend() {
        let number = 3;

        let from_stub = multi_friend_constructor::Gate::from_source(
            constructor_sources::Stub(number),
        );
        let from_pass = multi_friend_constructor::Gate::from_source(
            constructor_sources::Pass(number),
        );

        assert_eq!(from_stub.0, number, "{number}");
        assert_eq!(from_pass.0, number + 100, "{number}");
    }

    #[test]
    fn empty_constructor_friends_construct_only_from_the_relation() {
        let number = 17;

        let booth = Booth::from_source(number);
        let converted = booth_inspect(constructor_sources::Stub(number));

        assert_eq!(booth_value(&booth), number, "{number}");
        assert_eq!(converted, number, "{number}");
        assert_stub_is_not_booth_make();
    }
}
