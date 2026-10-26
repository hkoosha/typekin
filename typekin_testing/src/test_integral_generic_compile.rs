use crate::*;

#[cfg(feature = "test_slow_only")]
mod tests {
    use super::*;

    fn reject(
        name: &str,
        source: &str,
    ) {
        let output = compile(name, "", source, false);
        assert!(!output.status.success(), "{name}: {output:?}");
    }

    fn reject_tcp(
        name: &str,
        operation: &str,
    ) {
        let source = format!(
            "{}
fn forbidden() {{
    {operation}
}}
",
            include_str!("../../examples/my_tcp.rs")
        );
        reject(name, &source);
    }

    #[test]
    fn outgoing_point_cannot_add_incoming_offset() {
        let operation = "
            let point = SeqPoint::<Out>::make(1);
            let offset = SeqOffset::<In>::make(2);
            let _ = point + offset;
        ";
        reject_tcp("outgoing-point-incoming-add", operation);
    }

    #[test]
    fn incoming_point_cannot_add_outgoing_offset() {
        let operation = "
            let point = SeqPoint::<In>::make(1);
            let offset = SeqOffset::<Out>::make(2);
            let _ = point + offset;
        ";
        reject_tcp("incoming-point-outgoing-add", operation);
    }

    #[test]
    fn outgoing_point_cannot_subtract_incoming_offset() {
        let operation = "
            let point = SeqPoint::<Out>::make(1);
            let offset = SeqOffset::<In>::make(2);
            let _ = point - offset;
        ";
        reject_tcp("outgoing-point-incoming-sub", operation);
    }

    #[test]
    fn incoming_point_cannot_subtract_outgoing_offset() {
        let operation = "
            let point = SeqPoint::<In>::make(1);
            let offset = SeqOffset::<Out>::make(2);
            let _ = point - offset;
        ";
        reject_tcp("incoming-point-outgoing-sub", operation);
    }

    #[test]
    fn outgoing_point_cannot_subtract_incoming_point() {
        let operation = "
            let outgoing = SeqPoint::<Out>::make(1);
            let incoming = SeqPoint::<In>::make(2);
            let _ = outgoing - incoming;
        ";
        reject_tcp("outgoing-minus-incoming-point", operation);
    }

    #[test]
    fn incoming_point_cannot_subtract_outgoing_point() {
        let operation = "
            let incoming = SeqPoint::<In>::make(1);
            let outgoing = SeqPoint::<Out>::make(2);
            let _ = incoming - outgoing;
        ";
        reject_tcp("incoming-minus-outgoing-point", operation);
    }

    #[test]
    fn outgoing_offset_cannot_add_incoming_offset() {
        let operation = "
            let outgoing = SeqOffset::<Out>::make(1);
            let incoming = SeqOffset::<In>::make(2);
            let _ = outgoing + incoming;
        ";
        reject_tcp("outgoing-incoming-offset-add", operation);
    }

    #[test]
    fn incoming_offset_cannot_add_outgoing_offset() {
        let operation = "
            let incoming = SeqOffset::<In>::make(1);
            let outgoing = SeqOffset::<Out>::make(2);
            let _ = incoming + outgoing;
        ";
        reject_tcp("incoming-outgoing-offset-add", operation);
    }

    #[test]
    fn outgoing_offset_cannot_subtract_incoming_offset() {
        let operation = "
            let outgoing = SeqOffset::<Out>::make(1);
            let incoming = SeqOffset::<In>::make(2);
            let _ = outgoing - incoming;
        ";
        reject_tcp("outgoing-incoming-offset-sub", operation);
    }

    #[test]
    fn incoming_offset_cannot_subtract_outgoing_offset() {
        let operation = "
            let incoming = SeqOffset::<In>::make(1);
            let outgoing = SeqOffset::<Out>::make(2);
            let _ = incoming - outgoing;
        ";
        reject_tcp("incoming-outgoing-offset-sub", operation);
    }

    #[test]
    fn points_cannot_be_added() {
        let operation = "
            let lhs = SeqPoint::<Out>::make(1);
            let rhs = SeqPoint::<Out>::make(2);
            let _ = lhs + rhs;
        ";
        reject_tcp("point-plus-point", operation);
    }

    #[test]
    fn offsets_cannot_add_points() {
        let operation = "
            let offset = SeqOffset::<Out>::make(1);
            let point = SeqPoint::<Out>::make(2);
            let _ = offset + point;
        ";
        reject_tcp("offset-plus-point", operation);
    }

    #[test]
    fn offsets_cannot_subtract_points() {
        let operation = "
            let offset = SeqOffset::<Out>::make(1);
            let point = SeqPoint::<Out>::make(2);
            let _ = offset - point;
        ";
        reject_tcp("offset-minus-point", operation);
    }

    #[test]
    fn friend_construction_cannot_change_direction() {
        let operation = "
            let incoming = SeqPoint::<In>::make(1);
            let _ = SeqPoint::<Out>::of(incoming);
        ";
        reject_tcp("friend-direction-identity", operation);
    }

    #[test]
    fn const_arguments_are_part_of_operator_identity() {
        let source = "
            #[typekin::integral]
            struct Tagged<const SPACE: u8>(u32);

            fn forbidden() {
                let lhs = Tagged::<7>::make(1);
                let rhs = Tagged::<9>::make(2);
                let _ = lhs + rhs;
            }
        ";
        reject("const-argument-identity", source);
    }

    #[test]
    fn owning_markers_propagate_send_constraints() {
        let source = "
            #[typekin::integral]
            struct Tagged<T>(u32);

            fn require_send<T: Send>() {}
            fn forbidden() {
                require_send::<Tagged<std::rc::Rc<()>>>();
            }
        ";
        reject("generic-marker-not-send", source);
    }

    #[test]
    fn owning_markers_propagate_sync_constraints() {
        let source = "
            #[typekin::integral]
            struct Tagged<T>(u32);

            fn require_sync<T: Sync>() {}
            fn forbidden() {
                require_sync::<Tagged<std::cell::Cell<u8>>>();
            }
        ";
        reject("generic-marker-not-sync", source);
    }

    #[test]
    fn lifetime_markers_cannot_escape_their_borrow() {
        let source = "
            #[typekin::integral]
            struct Borrowed<'a>(u32);

            fn tied_to<'a>(_: &'a str) -> Borrowed<'a> {
                return Borrowed::make(1);
            }

            fn forbidden() -> Borrowed<'static> {
                let owner = String::from(\"local\");
                return tied_to(&owner);
            }
        ";
        reject("generic-marker-borrow-escape", source);
    }

    #[test]
    fn generic_integrals_compile_in_no_std_on_stable() {
        let source = "
            #![no_std]

            #[typekin::integral(
                profile = wrapping {},
                with = [serde, math_xor],
            )]
            pub struct Stored<'de, T: ?Sized, const SPACE: u8>(u32);

            pub fn transform<'a, T: ?Sized, const SPACE: u8>(raw: u32) -> u32 {
                let initial = Stored::<'a, T, SPACE>::make(raw);
                let parsed: Stored<'a, T, SPACE> = \"2\".parse().unwrap();
                let mapped = initial.map(|value| value.wrapping_add(1));
                return (mapped + parsed).raw();
            }
        ";
        let dependencies =
            "serde = { version = \"1\", default-features = false }";
        let output = compile("generic-no-std", dependencies, source, false);
        assert!(output.status.success(), "{output:?}");
    }

    #[test]
    fn enum_const_parameters_preserve_their_declared_type() {
        let source = "
            #![feature(adt_const_params)]
            use core::marker::ConstParamTy;

            #[derive(PartialEq, Eq, ConstParamTy)]
            enum Mode { Incoming, Outgoing }

            #[typekin::integral]
            struct Tagged<const MODE: Mode>(u32);

            const VALUE: Tagged<{ Mode::Outgoing }> = match Tagged::try_make(7) {
                Ok(value) => value,
                Err(_) => panic!(\"valid generic value rejected\"),
            };

            fn use_value() -> u32 {
                return VALUE.raw();
            }
        ";
        let output = compile("generic-enum-const", "", source, true);
        assert!(output.status.success(), "{output:?}");
    }

    #[test]
    fn const_parameter_types_keep_their_generic_scope() {
        let source = "
            #![feature(adt_const_params)]
            #![feature(generic_const_parameter_types)]
            #![feature(const_param_ty_trait)]
            use core::marker::ConstParamTy_;

            #[typekin::integral]
            struct Tagged<T: ConstParamTy_, const VALUE: T>(u32);

            const VALUE: Tagged<u8, 7> = match Tagged::try_make(9) {
                Ok(value) => value,
                Err(_) => panic!(\"valid generic value rejected\"),
            };
            const _: () = assert!(VALUE.raw() == 9);
        ";
        let output = compile("generic-dependent-const", "", source, true);
        assert!(output.status.success(), "{output:?}");
    }
}
