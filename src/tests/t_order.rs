#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::ToTokens as _;
    use syn::{
        File,
        Item,
        ItemEnum,
        ItemStruct,
    };

    fn items_of(expansion: syn::Result<TokenStream>) -> Vec<Item> {
        assert!(
            expansion.is_ok(),
            "{:?}",
            expansion.as_ref().err().map(ToString::to_string)
        );
        let file = syn::parse2::<File>(expansion.unwrap());
        assert!(
            file.is_ok(),
            "{:?}",
            file.as_ref().err().map(ToString::to_string)
        );
        return file.unwrap().items;
    }

    fn integral(
        config: &str,
        source: &str,
    ) -> Vec<Item> {
        let config = syn::parse_str::<crate::integral_cfg::Cfg>(config);
        assert!(config.is_ok(), "{:?}", config.as_ref().err());
        let item = syn::parse_str::<ItemStruct>(source);
        assert!(item.is_ok(), "{source}");
        return items_of(crate::integral::ekran(
            Box::new(config.unwrap()),
            item.unwrap(),
        ));
    }

    /// The items of the module `name`, which must be the first item.
    fn first_module_items<'a>(
        items: &'a [Item],
        name: &str,
    ) -> &'a [Item] {
        let first = items.first();
        let Some(Item::Mod(module)) = first
        else {
            panic!("the first item is not a module: {}", tokens_of(first));
        };
        assert_eq!(module.ident, name);
        let content = module.content.as_ref();
        assert!(content.is_some(), "module {name} has a body");
        return &content.unwrap().1;
    }

    fn assert_struct(
        item: Option<&Item>,
        name: &str,
    ) {
        let Some(Item::Struct(it)) = item
        else {
            panic!("expected struct {name}, found {}", tokens_of(item));
        };
        assert_eq!(it.ident, name);
    }

    fn tokens_of(item: Option<&Item>) -> String {
        return item.map_or_else(String::new, |it| {
            return it.to_token_stream().to_string();
        });
    }

    #[test]
    fn hidden_integral_module_comes_first_and_starts_with_the_item() {
        let config = "konst = none, tree = Unit { Byte, Word }";
        let items = integral(config, "pub struct Port(u16);");

        let module = first_module_items(&items, "__port");
        assert_struct(module.first(), "Port");
    }

    #[test]
    fn unhidden_integral_item_precedes_the_tree() {
        let config = "konst = none, hide_in = none, tree = Unit { Byte }";
        let items = integral(config, "pub struct Port(u16);");

        assert_struct(items.first(), "Port");
    }

    #[test]
    fn unhidden_integral_item_precedes_const_markers() {
        let config = "konst = none, hide_in = none";
        let source = "pub struct Slot<const N: usize>(u16);";
        let items = integral(config, source);

        assert_struct(items.first(), "Slot");
    }

    #[test]
    fn hidden_text_module_comes_first_and_starts_with_the_item() {
        let config = "tree = Kind { Short }";
        let config = syn::parse_str::<crate::text::Cfg>(config);
        assert!(config.is_ok(), "{:?}", config.as_ref().err());
        let source = "pub struct Name(String);";
        let item = syn::parse_str::<ItemStruct>(source);
        assert!(item.is_ok(), "{source}");
        let items =
            items_of(crate::text::ekran(config.unwrap(), item.unwrap()));

        let module = first_module_items(&items, "__name");
        assert_struct(module.first(), "Name");
    }

    #[test]
    fn hidden_bitflag_module_comes_first_and_starts_with_the_enum() {
        let config = "konst = none, tree = Kind { Small }";
        let config = syn::parse_str::<crate::bitflag::Cfg>(config);
        assert!(config.is_ok(), "{:?}", config.as_ref().err());
        let source = "#[repr(u8)] pub enum Flag { A, B }";
        let item = syn::parse_str::<ItemEnum>(source);
        assert!(item.is_ok(), "{source}");
        let items = items_of(crate::bitflag::ekran(
            Box::new(config.unwrap()),
            item.unwrap(),
        ));

        let module = first_module_items(&items, "__flag");
        let first = module.first();
        let Some(Item::Enum(it)) = first
        else {
            panic!("expected enum Flag, found {}", tokens_of(first));
        };
        assert_eq!(it.ident, "Flag");
    }
}
