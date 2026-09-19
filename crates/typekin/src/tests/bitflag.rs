#[cfg(test)]
mod test {
    use crate::bitflag::BitflagCfg;

    #[test]
    fn accepts_explicit_nested_konst() {
        let config = syn::parse_str::<BitflagCfg>("integral = [konst = false]")
            .expect("explicit nested konst should parse");
        assert!(!config.int.konst);
    }

    #[test]
    fn rejects_missing_konst() {
        let error = match syn::parse_str::<BitflagCfg>("friends = [u8]") {
            Ok(_) => panic!("konst must be explicit"),
            Err(error) => error,
        };
        assert_eq!(
            error.to_string(),
            "missing required `konst` argument in `integral = [...]`",
        );
    }
}
