use crate::fx_marker_tree::*;
use crate::*;

#[cfg(test)]
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

#[cfg(test)]
mod derives {
    use super::*;

    #[test]
    fn leaves_derive_the_marker_traits_by_default() {
        let left = Left;
        let copied = left;
        let sides = std::collections::HashSet::from([
            left,
            copied,
        ]);

        assert_eq!(left, Left::default());
        assert_eq!(left.cmp(&copied), core::cmp::Ordering::Equal);
        assert_eq!(format!("{left:?}"), "Left");
        assert_eq!(sides.len(), 1);
    }

    #[test]
    fn branch_bounds_imply_the_marker_traits() {
        let markers = copy_and_default_by_branch(Right);

        assert_eq!(
            markers,
            [
                Right,
                Right,
                Right
            ]
        );
    }

    #[test]
    fn nested_branches_inherit_the_marker_traits() {
        root_in_copies_through_its_nested_branch();
    }

    #[test]
    fn without_removes_leaf_derives() {
        assert_not_impl!(Port: core::default::Default);
        assert_not_impl!(Port: core::hash::Hash);
        assert_not_impl!(Port: core::cmp::PartialOrd);
        assert_not_impl!(Port: core::cmp::Ord);
    }

    #[test]
    fn without_keeps_the_remaining_derives() {
        let markers = copy_by_lean(Starboard);

        assert_eq!(
            markers,
            [
                Starboard,
                Starboard
            ]
        );
        assert_eq!(format!("{:?}", markers[0]), "Starboard");
    }

    #[test]
    fn embedded_tree_without_every_derive_has_a_dyn_compatible_branch() {
        let leaf: &dyn Plain = &PlainLeaf;

        assert_not_impl!(PlainLeaf: core::clone::Clone);
        assert_not_impl!(PlainLeaf: core::cmp::PartialEq);
        assert_eq!(size_of_val(leaf), 0);
    }
}

#[cfg(test)]
mod associated_types {
    use super::*;

    #[test]
    fn pairs_bind_each_leaf_to_the_other() {
        let reversed: <Ingress as Flow>::Rev = Egress;
        let back: <Egress as Flow>::Rev = Ingress;

        assert_eq!((reversed, back), (Egress, Ingress));
    }

    #[test]
    fn pairs_declare_an_involution() {
        let flow = reverse_twice::<Ingress>(Ingress);

        assert_eq!(flow, Ingress);
    }

    #[test]
    fn pair_lists_bind_each_pair_and_fixed_points() {
        let counter: <Clockwise as Turn>::Rev = Counter;
        let clockwise: <Counter as Turn>::Rev = Clockwise;
        let still: <Still as Turn>::Rev = Still;

        assert_eq!((counter, clockwise, still), (Counter, Clockwise, Still));
    }

    #[test]
    fn leaf_bindings_infer_a_declaration_bounded_by_their_branch() {
        let other: <LeftHand as Hand>::Other = RightHand;
        let itself: <BothHands as Hand>::Other = BothHands;
        let _ = other_hand::<LeftHand>(other);

        assert_eq!(itself, BothHands);
    }

    #[test]
    fn leaf_bindings_to_other_types_infer_an_unbounded_declaration() {
        let grip: <LeftHand as Hand>::Grip = 7;
        let wide: <BothHands as Hand>::Grip = 700;

        assert_eq!((grip, wide), (7_u8, 700_u16));
    }

    #[test]
    fn nested_leaves_infer_a_declaration_on_their_branch() {
        let peer: <Wired as Lan>::Peer = Wireless;
        let _ = lan_peer::<Wired>(peer);
    }

    #[test]
    fn leaves_bind_a_declaration_of_an_ancestor() {
        let base: <Kilometer as Unit>::Base = Meter;
        let own: <Meter as Unit>::Base = Meter;

        assert_eq!((base, own), (Meter, Meter));
    }

    #[test]
    fn embedded_tree_bindings_resolve_in_the_owners_scope() {
        let cost: <Fast as Mode>::Cost = FastCost;
        let path: <Slow as Mode>::Cost = FastCost;
        let in_place: <Quick as Pace>::Cost = FastCost;

        assert_eq!((cost, path, in_place), (FastCost, FastCost, FastCost));
    }
}
