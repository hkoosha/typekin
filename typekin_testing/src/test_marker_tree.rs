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

#[cfg(test)]
mod tests {
    use super::*;

    fn accepts_integral_marker<T>()
    where
        T: IntegralDirection + IntegralBranch,
    {
    }

    fn accepts_bitflag_marker<T>()
    where
        T: BitflagDirection,
    {
    }

    fn accepts_text_marker<T>()
    where
        T: TextDirection,
    {
    }

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
}
