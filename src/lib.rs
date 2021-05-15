#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;
#[cfg(doctest)]
doctest!("../README.md");

/// Hiden module for things the macros need to access.
#[doc(hidden)]
pub mod __priv {
    pub use core;
}

#[macro_export]
macro_rules! inline_const {
    ( < $($generic_ty:ident $( : $lft_bound:lifetime )? ),* $(,)? > [ $t:ty ] $e:expr) => {{
        // We must actually use all the const parameters. To avoid adding `Sized` constraints, we
        // use them behind a pointer indirection.
        struct Const<$($generic_ty $(: $lft_bound)? ),*>($(*mut $generic_ty),*);
        impl<$($generic_ty $(: $lft_bound)?),*> Const<$($generic_ty),*> {
            const C: $t = $e;
        }
        Const::<$($generic_ty),*>::C
    }};
    ( [ $t:ty ] $e:expr) => {{
        const C: $t = $e;
        C
    }};
}
