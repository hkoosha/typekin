use std::fmt::Debug;
use std::hash::Hash;

pub fn assert_panics<T>(f: impl FnOnce() -> T + std::panic::UnwindSafe) {
    std::panic::set_hook(Box::new(move |_| {}));
    let result = std::panic::catch_unwind(f);
    let _ = std::panic::take_hook();
    assert!(result.is_err());
}

pub fn unsafe_assert_panics<T>(f: impl FnOnce() -> T) {
    assert_panics(std::panic::AssertUnwindSafe(f))
}

pub fn accepts_copy_clone<T: Copy + Clone>() {}

pub fn accepts_ord<T: Ord>() {}

pub fn accepts_eq<T: Eq>() {}

pub fn accepts_hash<T: Hash>() {}

pub fn accepts_copy<T: Copy>() {}

pub fn accepts_clone<T: Clone>() {}

pub fn accepts_debug<T: Debug>() {}

pub use slow_asserts::*;

pub mod slow_asserts {
    use crate::machinery::asm;
    use crate::{
        AsmCase,
        Build,
        Operation,
        OptLevel,
        build,
    };

    /// What a case's raw/wrapped probe pairs must show at one opt-level.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Expect {
        /// Every pair is identical.
        Match,
        /// Every pair differs: the per-operation negative control.
        DiffEach,
        /// At least one pair differs: the per-case negative control, for
        /// cases whose single inline layer can vanish even at level 0.
        DiffAny,
    }

    /// Runs `case` from the host build at `opt_level` and compares its
    /// `operations`.
    pub fn assert_ok(
        case: &'static AsmCase,
        opt_level: OptLevel,
        expect: Expect,
        operations: &[Operation],
    ) {
        let build = build(case, None, opt_level);
        build.run(case);
        assert_expectation(&build, case, expect, operations);
    }

    /// `assert_ok` for a cross-target object, which is compiled but never
    /// linked or run.
    pub fn assert_cross(
        case: &'static AsmCase,
        target: &'static str,
        opt_level: OptLevel,
        expect: Expect,
        operations: &[Operation],
    ) {
        let build = build(case, Some(target), opt_level);
        assert_expectation(&build, case, expect, operations);
    }

    fn assert_expectation(
        build: &Build,
        case: &AsmCase,
        expect: Expect,
        operations: &[Operation],
    ) {
        build.assert_isolated(case, operations);
        let mut matches = Vec::new();
        let mut diffs = Vec::new();
        for it in operations {
            let raw = build.disassembly(it.raw);
            let wrapped = build.disassembly(it.wrapped);
            if raw == wrapped {
                matches.push(asm::assembly_match(it, &raw));
            }
            else {
                diffs.push(asm::text_diff(it, &raw, &wrapped));
            }
        }

        let (expected, unexpected) = match expect {
            Expect::Match => (matches, diffs),
            Expect::DiffEach => (diffs, matches),
            Expect::DiffAny if diffs.is_empty() => (diffs, matches),
            Expect::DiffAny => (diffs, Vec::new()),
        };
        if !expected.is_empty() {
            eprintln!("expected found:\n\n{}", expected.join("\n"));
        }
        assert!(
            unexpected.is_empty(),
            "case `{}` ({build}), expected {expect:?}; unexpected:\n\n{}",
            case.name,
            unexpected.join("\n")
        );
    }

    /// Each `wrapped` probe of `case` has at most as many instructions as
    /// its `raw` counterpart (typekin's output is never longer than the
    /// reference).
    pub fn assert_not_longer(
        case: &'static AsmCase,
        opt_level: OptLevel,
        operations: &[Operation],
    ) {
        let build = build(case, None, opt_level);
        build.run(case);
        build.assert_isolated(case, operations);

        let longer = operations
            .iter()
            .filter_map(|it| {
                let reference = build.disassembly(it.raw).lines;
                let wrapped = build.disassembly(it.wrapped).lines;
                return (wrapped.len() > reference.len()).then(|| {
                    format!(
                        "{}: {} has {} instructions, {} has {}\n{}",
                        it.name,
                        it.wrapped,
                        wrapped.len(),
                        it.raw,
                        reference.len(),
                        wrapped.join("\n"),
                    )
                });
            })
            .collect::<Vec<_>>();

        assert!(
            longer.is_empty(),
            "case `{}` ({build}), longer than written:\n\n{}",
            case.name,
            longer.join("\n\n")
        );
    }
}
