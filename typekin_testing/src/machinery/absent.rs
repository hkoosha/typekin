//! Compile-time assertions that something does not exist. A violation is a
//! compile error. Fixture crates include this file verbatim, so it holds
//! only these macros.

/// Compiles only when `$ty` does not implement `$trait`. With an impl, both
/// blanket impls of `AmbiguousIfImpl` apply and `_` cannot be inferred.
#[macro_export]
macro_rules! assert_not_impl {
    ($ty:ty: $trait:path) => {{
        trait AmbiguousIfImpl<A> {
            fn some_item() {}
        }
        impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
        struct Implemented;
        impl<T: ?Sized + $trait> AmbiguousIfImpl<Implemented> for T {}
        let _ = <$ty as AmbiguousIfImpl<_>>::some_item;
    }};
}

/// Compiles only when `$ty` has no inherent method `$method`. Inherent
/// methods win method resolution, so an existing one replaces the fallback
/// and its result (or arity) mismatches `Absent`.
#[macro_export]
macro_rules! assert_no_method {
    ($ty:ty, $method:ident) => {{
        struct Absent;
        trait Fallback {
            fn $method(&self) -> Absent {
                return Absent;
            }
        }
        impl<T: ?Sized> Fallback for T {}
        fn check(value: $ty) -> Absent {
            return value.$method();
        }
    }};
}

/// Compiles only when `$ty` has no inherent associated function `$name`;
/// `assert_no_method` for functions without a receiver.
#[macro_export]
macro_rules! assert_no_associated_fn {
    ($ty:ty, $name:ident) => {{
        struct Absent;
        trait Fallback {
            fn $name() -> Absent {
                return Absent;
            }
        }
        impl<T: ?Sized> Fallback for T {}
        fn check() -> Absent {
            return <$ty>::$name();
        }
    }};
}
