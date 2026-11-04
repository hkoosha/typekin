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
        let this = configuration.expand_embedded(&owner, &visibility);
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
        let this = configuration.expand_embedded(&owner, &visibility);
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
}
