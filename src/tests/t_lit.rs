#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::{
        Generics,
        ItemStruct,
        Visibility,
    };

    use crate::integral::ekran;
    use crate::integral_cfg::Cfg;
    use crate::zz_lit::LitMacro;

    fn expand(
        config: &str,
        source: &str,
    ) -> syn::Result<String> {
        let config = syn::parse_str::<Cfg>(config)?;
        let item = syn::parse_str::<ItemStruct>(source)?;
        return ekran(Box::new(config), item).map(|it| it.to_string());
    }

    fn private_lit(ty: &str) -> syn::Result<LitMacro> {
        return LitMacro::new(
            &format_ident!("{}", ty),
            &Generics::default(),
            &Visibility::Inherited,
            None,
            None,
        );
    }

    fn definition_of(source: &str) -> String {
        let item = syn::parse_str::<ItemStruct>(source);
        assert!(item.is_ok(), "{source}");
        let item = item.unwrap();
        let lit =
            LitMacro::new(&item.ident, &item.generics, &item.vis, None, None);
        assert!(lit.is_ok(), "{source}");

        return lit.unwrap().definition(&item.vis, false).to_string();
    }

    #[test]
    fn keyword_name_is_rejected() {
        let ty = "Ref";
        let lit = private_lit(ty);
        assert!(lit.is_err(), "{ty}");

        assert_eq!(
            lit.err().unwrap().to_string(),
            "`Ref` would generate `ref!`, which is a keyword; set `lit_name` \
            or add `without = lit_macro`"
        );
    }

    #[test]
    fn prelude_shadowing_name_is_rejected() {
        let ty = "CompileError";
        let lit = private_lit(ty);
        assert!(lit.is_err(), "{ty}");

        assert_eq!(
            lit.err().unwrap().to_string(),
            "`CompileError` would generate `compile_error!`, shadowing \
            `core::compile_error!`; set `lit_name` or add \
            `without = lit_macro`"
        );
    }

    #[test]
    fn lit_path_must_start_at_the_crate_root() {
        let config = "domain = 1.., lit_path = net::Port";
        let expansion = expand(config, "struct Port(u16);");
        assert!(expansion.is_err(), "{config}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "lit_path must be a crate-relative path such as `crate::net::Port`"
        );
    }

    #[test]
    fn lit_path_rejects_generic_arguments() {
        let config = "domain = 1.., lit_path = crate::net::Port<u8>";
        let expansion = expand(config, "struct Port(u16);");
        assert!(expansion.is_err(), "{config}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "lit_path cannot have generic arguments"
        );
    }

    #[test]
    fn generic_type_without_defaults_rejects_the_plain_form() {
        let source = "struct Tagged<D>(u32);";
        let definition = definition_of(source);

        assert!(
            definition.contains(
                "($ value : expr) => { :: core :: compile_error ! (\"`tagged!` \
                needs generic arguments"
            ),
            "{source}: {definition}"
        );
        assert!(
            definition.contains("([$ ($ args : tt) *] $ value : expr)"),
            "{source}: {definition}"
        );
    }

    #[test]
    fn definition_contains_no_unsafe() {
        let source = "pub struct Tagged<D>(u32);";
        let definition = definition_of(source);

        assert!(!definition.contains("unsafe"), "{source}: {definition}");
    }

    #[test]
    fn without_lit_macro_removes_the_guard() {
        let config = "domain = 1.., without = lit_macro";
        let expansion = expand(config, "struct Port(u16);");
        assert!(expansion.is_ok(), "{config}");
        let expansion = expansion.unwrap();

        assert!(
            !expansion.contains("__typekin_lit_"),
            "{config}: {expansion}"
        );
    }

    #[test]
    fn non_const_try_make_rejects_lit_macro() {
        let config = "domain = 1.., konst = none, with = lit_macro";
        let expansion = expand(config, "struct Port(u16);");
        assert!(expansion.is_err(), "{config}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`lit_macro` needs a const `try_make`; remove `konst = none` or \
            add `without = lit_macro`"
        );
    }

    #[test]
    fn lit_name_requires_the_macro() {
        let config = "domain = 1.., without = lit_macro, lit_name = port_no";
        let expansion = expand(config, "struct Port(u16);");
        assert!(expansion.is_err(), "{config}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`lit_name` needs the literal macro"
        );
    }
}
