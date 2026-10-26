#[cfg(test)]
mod tests {
    use syn::ItemTrait;

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
    fn nested_tree_implements_every_ancestor_for_leaves() {
        let configuration = "
            Transport {
                Direction {
                    In,
                    Out,
                },
            },
        ";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        assert!(this.is_ok(), "{configuration}");
        let output = this.unwrap().to_string();

        assert!(output.contains("trait Transport : MakerSeal"));
        assert!(output.contains("trait Direction : Transport"));
        assert!(output.contains("struct In ;"));
        assert!(output.contains("impl MakerSeal for In"));
        assert!(output.contains("impl Transport for In"));
        assert!(output.contains("impl Direction for In"));
        assert!(output.contains("impl MakerSeal for Out"));
        assert!(output.contains("impl Transport for Out"));
        assert!(output.contains("impl Direction for Out"));
    }

    #[test]
    fn tree_preserves_explicit_node_visibility() {
        let configuration = "pub Direction { pub In, }";
        let root = "trait MakerSeal {}";
        let this = expand_tree(configuration, root);
        assert!(this.is_ok(), "{configuration}");
        let output = this.unwrap().to_string();

        assert!(output.contains("pub trait Direction : MakerSeal"));
        assert!(output.contains("pub struct In ;"));
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
