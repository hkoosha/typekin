use crate::fx_marker_tree::*;
use crate::*;

#[cfg(test)]
#[cfg(feature = "test_fast")]
mod tests {
    use super::*;

    #[test]
    fn exported_tree_nodes_implement_their_visible_hierarchy() {
        accepts_integral_marker::<IntegralIn>();
        accepts_bitflag_marker::<BitflagIn>();
        accepts_text_marker::<TextIn>();
    }

    #[test]
    fn tree_attributes_leave_construction_available() {
        let number = TreeIntegral::make(7);
        assert_eq!(number.raw(), 7);

        let flags = TreeBitflag::Read.into_value();
        assert_eq!(flags.bits(), 1);

        let text = TreeText::try_make(String::from("tree"));
        assert!(text.is_ok());
        let text = text.unwrap();
        assert_eq!(text.as_str(), "tree");
    }

    #[test]
    fn branches_extend_their_ancestors() {
        root_in_extends_its_ancestors();
    }

    #[test]
    fn leaf_markers_are_zero_sized() {
        assert_eq!(root_in_size(), 0);
        assert_eq!(size_of::<IntegralIn>(), 0);
    }

    #[test]
    fn nodes_inherit_the_root_visibility() {
        root_out_has_the_root_visibility();
    }

    #[test]
    fn nodes_inherit_the_branch_visibility() {
        inbound_has_the_branch_visibility();
    }

    #[test]
    fn embedded_tree_honors_explicit_node_visibility() {
        forward_has_the_route_visibility();
    }
}
