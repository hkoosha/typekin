#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::{
        ItemTrait,
        parse_quote,
    };

    use crate::marker_tree::{
        Cfg,
        expand,
    };

    fn expand_tree(
        configuration: &str,
        root: &str,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let configuration = syn::parse_str::<Cfg>(configuration)?;
        let root = syn::parse_str::<ItemTrait>(root)?;

        return expand(configuration, root);
    }

    #[test]
    fn embedded_tree_uses_a_private_seal() {
        let configuration = "Direction { Transport { In, }, Out, }";
        let this = syn::parse_str::<Cfg>(configuration);
        assert!(this.is_ok(), "{configuration}");
        let configuration = this.unwrap();
        let owner = format_ident!("Packet");
        let visibility = parse_quote!(pub);
        let this = configuration.expand_embedded(&owner, &visibility, false);
        assert!(this.is_ok(), "{configuration:?}");
        let output = this.unwrap().to_string();

        assert!(output.contains("mod __typekin_marker_tree_for_Packet"));
        assert!(output.contains("trait MarkerTreeRoot"));
        assert!(output.contains("pub trait Direction : MarkerTreeRoot"));
        assert!(
            output
                .contains("pub trait Transport : Direction + MarkerTreeRoot",),
        );
    }

    #[test]
    fn embedded_tree_reserves_the_virtual_root_name() {
        let configuration = "MarkerTreeRoot";
        let this = syn::parse_str::<Cfg>(configuration);
        assert!(this.is_ok(), "{configuration}");
        let configuration = this.unwrap();
        let owner = format_ident!("Packet");
        let visibility = parse_quote!(pub);
        let this = configuration.expand_embedded(&owner, &visibility, false);
        let error = this.unwrap_err();

        assert!(error.to_string().contains("duplicated marker tree node"));
    }

    #[test]
    fn empty_tree_is_rejected() {
        let configuration = "";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert!(error.to_string().contains("at least one node"));
    }

    #[test]
    fn empty_branch_is_rejected() {
        let configuration = "Direction {}";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert!(error.to_string().contains("at least one child"));
    }

    #[test]
    fn duplicate_nodes_are_rejected() {
        let configuration = "Direction { In }, Other { In }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert!(error.to_string().contains("duplicated marker tree node"));
    }

    #[test]
    fn generic_root_is_rejected() {
        let configuration = "Direction { In }";
        let root = "trait MakerSeal<T> {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert!(error.to_string().contains("must not be generic"));
    }

    #[test]
    fn root_supertraits_are_rejected() {
        let configuration = "Direction { In }";
        let root = "trait MakerSeal: Copy {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert!(error.to_string().contains("must not have supertraits"));
    }

    #[test]
    fn removing_a_derive_another_needs_is_rejected() {
        let configuration = "without = clone, Direction { In }";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "`copy` needs `clone`; remove both");
    }

    #[test]
    fn removing_partial_eq_alone_is_rejected() {
        let configuration = "without = partial_eq, Direction { In }";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "`eq` needs `partial_eq`; remove both");
    }

    #[test]
    fn removing_a_derive_with_its_dependents_is_accepted() {
        let configuration = "without = [clone, copy], Direction { In }";
        let this = syn::parse_str::<Cfg>(configuration);

        assert!(this.is_ok(), "{configuration}");
    }

    #[test]
    fn with_is_rejected() {
        let configuration = "with = clone, Direction { In }";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn repeated_without_is_rejected() {
        let configuration = "without = hash, Direction { In }, without = ord";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "duplicated arg");
    }

    #[test]
    fn unknown_tree_flag_is_rejected() {
        let configuration = "without = display, Direction { In }";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "unknown flag");
    }

    #[test]
    fn missing_binding_is_rejected() {
        let configuration =
            "Direction { type Rev: Direction, In { type Rev = Out; }, Out }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "`Out` needs `type Rev`, declared by `Direction`",
        );
    }

    #[test]
    fn binding_without_a_branch_is_rejected() {
        let configuration = "In { type Rev = In; }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "`type Rev` needs a branch above `In` to declare it",
        );
    }

    #[test]
    fn redeclaring_an_ancestors_type_is_rejected() {
        let configuration =
            "A { type X: A, B { type X: B, L { type X = L; } } }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "`type X` is already declared by `A`");
    }

    #[test]
    fn pair_beside_a_written_declaration_is_rejected() {
        let configuration =
            "Direction { type Rev: Direction, type Rev = In <-> Out }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "duplicated `type Rev`");
    }

    #[test]
    fn duplicate_binding_is_rejected() {
        let configuration = "A { L { type X = L; type X = L; } }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "duplicated `type X`");
    }

    #[test]
    fn binding_beside_children_is_rejected() {
        let configuration = "A { type X = L; L }";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "a node holds either children or `type NAME = TYPE;` bindings",
        );
    }

    #[test]
    fn pair_member_beside_a_node_of_the_same_name_is_rejected() {
        let configuration = "Direction { In, type Rev = In <-> Out }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "duplicated marker tree node");
    }

    #[test]
    fn non_doc_node_attribute_is_rejected() {
        let configuration = "#[allow(dead_code)] Direction { In }";
        let this = syn::parse_str::<Cfg>(configuration);
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "marker tree nodes accept only doc comments",
        );
    }

    #[test]
    fn node_docs_are_kept() {
        let configuration = "
            /// Where traffic flows.
            Direction {
                /// Incoming.
                type Rev = In <-> Out,
            }
        ";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        assert!(this.is_ok(), "{configuration}");
        let expansion = this.unwrap().to_string();

        assert!(
            expansion.contains("# [doc = \" Where traffic flows.\"] # [allow"),
            "{expansion}"
        );
        assert!(
            expansion.contains("# [doc = \" Incoming.\"] type Rev"),
            "{expansion}"
        );
    }

    #[test]
    fn bindings_to_types_outside_the_tree_infer_no_bound() {
        let configuration =
            "Mode { Fast { type Cost = u8; }, Slow { type Cost = u16; } }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        assert!(this.is_ok(), "{configuration}");
        let expansion = this.unwrap().to_string();

        assert!(expansion.contains("type Cost ;"), "{expansion}");
    }
}
