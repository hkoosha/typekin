use std::cell::{
    Cell,
    RefCell,
};
use std::panic::AssertUnwindSafe;

use crate::*;

/// Validated: a result outside the domain panics.
#[typekin::integral(konst = none, domain = ..=100, friend = _(u16) -> Make)]
pub(crate) struct Percent(u8);

/// Unvalidated: only Rust's arithmetic and the primitive's functions
/// panic.
#[typekin::integral(konst = none)]
pub(crate) struct Count(u8);

/// How often `panic_location` runs its operation again when another test
/// replaced the panic hook in between.
const ATTEMPTS: usize = 1000;

thread_local! {
    /// Whether this thread's panic is the one `panic_location` runs.
    static CAPTURING: Cell<bool> = const { Cell::new(false) };
    /// Where that panic was reported.
    static LOCATION: RefCell<Option<(String, u32)>> =
        const { RefCell::new(None) };
}

/// The file and line that the panic of `operation` reports.
///
/// The panic hook is one per process and other tests replace it while
/// this one runs (`assert_panics`). So the hook writes to a slot of the
/// panicking thread, and `operation` runs again when another hook took
/// the panic. A panic of another thread is printed as the default hook
/// would.
pub(crate) fn panic_location<T>(operation: impl Fn() -> T) -> (String, u32) {
    for _ in 0..ATTEMPTS {
        std::panic::set_hook(Box::new(|info| {
            if !CAPTURING.get() {
                eprintln!("{info}");
                return;
            }
            let location =
                info.location().map(|it| (it.file().to_owned(), it.line()));
            LOCATION.set(location);
        }));
        CAPTURING.set(true);
        let result = std::panic::catch_unwind(AssertUnwindSafe(&operation));
        CAPTURING.set(false);
        let _ = std::panic::take_hook();

        assert!(result.is_err(), "the operation did not panic");
        if let Some(location) = LOCATION.take() {
            return location;
        }
    }
    panic!("another panic hook took the panic {ATTEMPTS} times");
}
