use crate::*;

#[typekin::integral(
    konst = none,
    tree = IntegralDirection {
        IntegralBranch {
            IntegralIn,
        },
    },
)]
pub struct TreeIntegral(u8);

#[repr(u8)]
#[typekin::bitflag(
    konst = none,
    tree = BitflagDirection {
        BitflagIn,
    },
)]
pub enum TreeBitflag {
    Read = 1,
}

#[typekin::text(
    konst = none,
    tree = [TextDirection {
        TextIn,
    }],
)]
pub struct TreeText(String);

// The `pub(super)` trees below reach only this module, so the helpers at the end
// of this file name their nodes for the tests.
pub(crate) mod root_visibility {
    #[typekin::tree(
        Transport {
            Direction {
                In,
                Out,
            },
        },
    )]
    pub(super) trait RootSeal {}
}

pub(crate) mod branch_visibility {
    #[typekin::tree(
        pub(super) Lane {
            Inbound,
        },
    )]
    trait BranchSeal {}
}

pub(crate) mod embedded_visibility {
    #[typekin::integral(
        konst = none,
        tree = pub(super) Route {
            Forward,
        },
    )]
    struct Packet(u8);
}

pub(crate) fn accepts_integral_marker<T>()
where
    T: IntegralDirection + IntegralBranch,
{
}

pub(crate) fn accepts_bitflag_marker<T>()
where
    T: BitflagDirection,
{
}

pub(crate) fn accepts_text_marker<T>()
where
    T: TextDirection,
{
}

fn accepts_root_seal<T>()
where
    T: root_visibility::RootSeal,
{
}

fn accepts_transport<T>()
where
    T: root_visibility::Transport,
{
}

fn accepts_direction<T>()
where
    T: root_visibility::Direction,
{
    accepts_transport::<T>();
    accepts_root_seal::<T>();
}

fn accepts_lane<T>()
where
    T: branch_visibility::Lane,
{
}

fn accepts_route<T>()
where
    T: embedded_visibility::Route,
{
}

/// Requires the root tree's `In` leaf to implement `Direction` and its
/// ancestors.
pub(crate) fn root_in_extends_its_ancestors() {
    accepts_direction::<root_visibility::In>();
}

/// The size of the root tree's `In` leaf.
pub(crate) fn root_in_size() -> usize {
    return size_of::<root_visibility::In>();
}

/// Names the root tree's `Out` leaf here, where `RootSeal`'s `pub(super)`
/// reaches, and requires it to implement `Direction` and its ancestors.
pub(crate) fn root_out_has_the_root_visibility() {
    accepts_direction::<root_visibility::Out>();
}

/// Names the `Inbound` leaf here, where `Lane`'s `pub(super)` reaches.
pub(crate) fn inbound_has_the_branch_visibility() {
    accepts_lane::<branch_visibility::Inbound>();
}

/// Names the `Forward` leaf here, where `Route`'s `pub(super)` reaches.
pub(crate) fn forward_has_the_route_visibility() {
    accepts_route::<embedded_visibility::Forward>();
}
