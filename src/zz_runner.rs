use std::collections::BTreeSet;
use std::fmt::Display;
use std::panic::UnwindSafe;
use std::sync::{
    Arc,
    Mutex,
};

use proc_macro2::{
    Span,
    TokenStream,
};
use syn::Error;
use syn::spanned::Spanned;

// Can be enabled on demand, but MUST NOT be ever checked into the code,
// released, or published as enabled.
const TYPEKIN_MACHINERY_DEBUG: bool = false;

pub(crate) fn typekin_catch_panics() -> bool {
    return TYPEKIN_MACHINERY_DEBUG;
}

pub(crate) fn catching(
    f: impl FnOnce() -> syn::Result<TokenStream> + UnwindSafe
) -> TokenStream {
    return match typekin_catch_panics() {
        false => f().unwrap_or_else(|it| it.to_compile_error()),

        true => {
            let err = Arc::new(Mutex::new(None));

            let err0 = err.clone();
            std::panic::set_hook(Box::new(move |it| {
                let text = format!(
                    "\n{}\n{}\n",
                    it,
                    std::backtrace::Backtrace::capture()
                );
                err0.lock().unwrap().replace(text);
            }));

            let result = std::panic::catch_unwind(|| {
                f().unwrap_or_else(|it| it.to_compile_error())
            });
            let _ = std::panic::take_hook();

            result
                .map_err(|_| err.lock().unwrap().take().unwrap())
                .map_err(|mut it| {
                    let remove = "disabled backtrace\n";
                    if it.ends_with(remove) {
                        it.truncate(it.len() - remove.len());
                    }
                    return it;
                })
                .map_err(|it| quote::quote! { compile_error!(#it); })
                .unwrap_or_else(|it| it)
        }
    };
}

// ============================================================================

pub(crate) fn snake_case_of(it: &str) -> String {
    // AI generated, whatever.
    let it = it.strip_prefix("r#").unwrap_or(it);
    let mut result = String::with_capacity(it.len());
    let chars = it.chars().vec();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !chars[i - 1].is_uppercase() {
                result.push('_');
            }
            else if i > 0
                && chars[i - 1].is_uppercase()
                && let Some(&next) = chars.get(i + 1)
                && next.is_lowercase()
            {
                result.push('_');
            }

            result.push(c.to_lowercase().next().unwrap());
        }
        else {
            result.push(c);
        }
    }

    result
}

// ============================================================================

pub(crate) trait Merge {
    #[must_use]
    fn merge(
        self,
        that: Self,
    ) -> Self;
}

impl Merge for TokenStream {
    fn merge(
        mut self,
        that: Self,
    ) -> Self {
        self.extend(that);
        return self;
    }
}

pub(crate) trait Merged {
    type Output;

    #[must_use]
    fn merged(self) -> Self::Output;
}

impl<I> Merged for I
where
    I: Iterator<Item: Merge + Default>,
{
    type Output = I::Item;

    fn merged(self) -> Self::Output {
        return self
            .reduce(|a, b| {
                return a.merge(b);
            })
            .unwrap_or_default();
    }
}

// -------------------------------------

pub(crate) trait MySpanned {
    fn my_span(&self) -> Span;
}

impl<S: Spanned> MySpanned for S {
    fn my_span(&self) -> Span {
        return self.span();
    }
}

pub(crate) trait MkErr {
    fn fail<T>(
        &self,
        msg: impl Display,
    ) -> Result<T, Error> {
        return Err(self.errorful(msg));
    }

    fn fail_combi<T>(
        &self,
        msg: impl Display,
        spanned: impl MySpanned,
        msg1: impl Display,
    ) -> Result<T, Error> {
        return Err(self.combine(msg, spanned, msg1));
    }

    fn errorful(
        &self,
        msg: impl Display,
    ) -> Error;

    fn combine(
        &self,
        msg0: impl Display,
        spanned: impl MySpanned,
        msg1: impl Display,
    ) -> Error {
        let mut lhs = self.errorful(msg0);
        let rhs = spanned.errorful(msg1);
        lhs.combine(rhs);
        return lhs;
    }
}

impl<U: MySpanned> MkErr for U {
    fn errorful(
        &self,
        msg: impl Display,
    ) -> Error {
        return Error::new(self.my_span(), msg);
    }
}

// -------------------------------------

pub(crate) trait ToCollection<T> {
    fn vec(self) -> Vec<T>;

    fn set(self) -> BTreeSet<T>
    where
        T: Ord;
}

impl<I, T> ToCollection<T> for I
where
    I: IntoIterator<Item = T>,
{
    fn vec(self) -> Vec<T> {
        return self.into_iter().collect();
    }

    fn set(self) -> BTreeSet<T>
    where
        T: Ord,
    {
        return self.into_iter().collect();
    }
}
