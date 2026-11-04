use std::sync::LazyLock;

use crate::fx_compile_fail::WITHIN_GENERIC_SOURCE;
use crate::*;

pub(crate) const LITERAL_PROVIDER: &str = r#"
pub mod net {
    #[typekin::integral(domain = 1..=1023)]
    pub struct Port(u16);

    #[typekin::integral(
        domain = ..=100,
        lit_path = crate::net::Percent,
    )]
    pub struct Percent(u8);
}
"#;

/// Depends on `literal_provider`, a sibling fixture directory.
pub(crate) const LITERAL_CONSUMER: Fixture = Fixture {
    name: "literal_consumer",
    toolchain: Toolchain::Nightly,
    target: None,
    header: "",
    dependencies: r#"literal_provider = { path = "../literal_provider" }"#,
};

pub(crate) const PUBLIC_LITERAL: Case = Case {
    name: "public_literal_is_usable_from_another_crate",
    source: r#"
use literal_provider::net::{Port, port};

pub const HTTPS: Port = port!(443);

pub fn percent() -> u8 {
    return literal_provider::net::percent!(42).raw();
}
"#,
};

pub(crate) static LITERAL: LazyLock<Checked> = LazyLock::new(|| {
    write_crate("literal_provider", "", LITERAL_PROVIDER);
    return check(
        &LITERAL_CONSUMER,
        &cases(&[
            &PUBLIC_LITERAL,
            &WITHIN_OTHER_CRATE_SOURCE,
            &WITHIN_GENERIC_UNINSTANTIATED,
        ]),
    );
});

pub(crate) const WITHIN_OTHER_CRATE_SOURCE: Case = Case {
    name: "within_accepts_a_source_from_another_crate",
    source: r#"
use literal_provider::net::Port;

#[typekin::integral(domain = 1.., friend = _(Port) -> Within)]
pub struct Positive(u16);

pub fn positive(port: Port) -> Positive {
    return Positive::of(port);
}
"#,
};

/// The negative control of `test_compile_fail`'s built case: `cargo check`
/// never instantiates the generic target, so it accepts the refuted edge.
pub(crate) const WITHIN_GENERIC_UNINSTANTIATED: Case = Case {
    name: "within_generic_target_proves_only_where_instantiated",
    source: WITHIN_GENERIC_SOURCE,
};

/// `generic_const_parameter_types` is incomplete; it stays out of the
/// testing crate itself.
pub(crate) const DEPENDENT_CONST: Fixture = Fixture {
    name: "dependent_const",
    toolchain: Toolchain::Nightly,
    target: None,
    header: "
#![feature(adt_const_params)]
#![feature(generic_const_parameter_types)]
#![feature(const_param_ty_trait)]
",
    dependencies: "",
};

pub(crate) const CONST_PARAMETER_TYPES: Case = Case {
    name: "const_parameter_types_keep_their_generic_scope",
    source: r#"
use core::marker::ConstParamTy_;

#[typekin::integral]
struct Tagged<T: ConstParamTy_, const VALUE: T>(u32);

const VALUE: Tagged<u8, 7> = match Tagged::try_make(9) {
    Ok(value) => value,
    Err(_) => panic!("valid generic value rejected"),
};
const _: () = assert!(VALUE.raw() == 9);
"#,
};

pub(crate) static DEPENDENT: LazyLock<Checked> = LazyLock::new(|| {
    return check(&DEPENDENT_CONST, &cases(&[&CONST_PARAMETER_TYPES]));
});
