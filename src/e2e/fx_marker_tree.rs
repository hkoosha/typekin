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

#[typekin::tree(
    pub(crate) Side {
        Left,
        Right,
    },
)]
trait SideSeal {}

#[typekin::tree(
    without = [default, hash, partial_ord, ord],
    pub(crate) Lean {
        Port,
        Starboard,
    },
)]
trait LeanSeal {}

#[typekin::integral(
    konst = none,
    tree = [
        without = [
            clone,
            copy,
            debug,
            default,
            partial_eq,
            eq,
            hash,
            partial_ord,
            ord,
        ],
        pub(crate) Plain {
            PlainLeaf,
        },
    ],
)]
struct TreeWithoutDerives(u8);

/// Copies `marker` and builds a default one through `Side` bounds alone.
pub(crate) fn copy_and_default_by_branch<T: Side>(marker: T) -> [T; 3] {
    return [
        marker,
        marker,
        T::default(),
    ];
}

/// Copies `marker` through `Lean` bounds alone.
pub(crate) fn copy_by_lean<T: Lean>(marker: T) -> [T; 2] {
    return [
        marker,
        marker,
    ];
}

/// Copies the root tree's `In` leaf through its nested `Direction` branch,
/// which inherits the marker traits from the top-level `Transport`.
pub(crate) fn root_in_copies_through_its_nested_branch() {
    fn copy<T: root_visibility::Direction>(marker: T) -> [T; 2] {
        return [
            marker,
            marker,
        ];
    }

    let _ = copy(root_visibility::In);
}

#[typekin::tree(
    without = [hash, partial_ord, ord],
    /// Where traffic flows.
    pub(crate) Flow {
        /// The opposite flow.
        type Rev = Ingress <-> Egress,
    },
)]
trait FlowSeal {}

#[typekin::tree(
    pub(crate) Turn {
        type Rev = [Clockwise <-> Counter, Still <-> Still],
    },
)]
trait TurnSeal {}

#[typekin::tree(
    pub(crate) Hand {
        LeftHand { type Other = RightHand; type Grip = u8; },
        RightHand { type Other = LeftHand; type Grip = u8; },
        BothHands { type Other = Self; type Grip = u16; },
    },
)]
trait HandSeal {}

#[typekin::tree(
    pub(crate) Network {
        pub(crate) Lan {
            Wired { type Peer = Wireless; },
            Wireless { type Peer = Wired; },
        },
    },
)]
trait NetworkSeal {}

#[typekin::tree(
    pub(crate) Unit {
        type Base: Unit,
        Meter { type Base = Self; },
        pub(crate) Derived {
            Kilometer { type Base = Meter; },
        },
    },
)]
trait UnitSeal {}

/// Crate-private, as every binding's target may be: the embedded tree keeps
/// the declared reach.
#[derive(Debug, PartialEq)]
pub(crate) struct FastCost;

#[typekin::integral(
    konst = none,
    tree = pub(crate) Mode {
        Fast { type Cost = FastCost; },
        Slow { type Cost = self::FastCost; },
    },
)]
struct TreeWithCost(u8);

#[typekin::integral(
    konst = none,
    hide_in = none,
    tree = pub(crate) Pace {
        Quick { type Cost = self::FastCost; },
    },
)]
struct TreeWithCostInPlace(u8);

/// Reverses a flow twice through `Flow` bounds alone: it compiles only when
/// `Rev` is declared as an involution.
pub(crate) fn reverse_twice<D: Flow>(
    flow: <<D as Flow>::Rev as Flow>::Rev
) -> D {
    return flow;
}

/// Uses `Other` through `Hand` bounds alone: it compiles only when the
/// inferred declaration is bounded by `Hand`.
pub(crate) fn other_hand<H: Hand>(other: H::Other) -> impl Hand {
    return other;
}

/// Uses `Peer` through `Lan` bounds alone: it compiles only when the inferred
/// declaration is bounded by `Lan`, the branch above both peers.
pub(crate) fn lan_peer<T: Lan>(peer: T::Peer) -> impl Lan {
    return peer;
}
