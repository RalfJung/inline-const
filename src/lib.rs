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
    ( @parsed < $($generic_ty:ident $( : $lft_bound:lifetime )? ),* > < $(const $generic_const:ident : $generic_const_ty:ty),* > [ $t:ty ] $e:expr) => {{
        // We must actually use all the const parameters. To avoid adding `Sized` constraints, we
        // use them behind a pointer indirection.
        struct Const<$($generic_ty $(: $lft_bound)?,)* $(const $generic_const: $generic_const_ty,)*>($(*mut $generic_ty),*);
        impl<$($generic_ty $(: $lft_bound)?,)* $(const $generic_const: $generic_const_ty,)*> Const<$($generic_ty,)* $($generic_const,)*> {
            const C: $t = $e;
        }
        Const::<$($generic_ty,)* $($generic_const,)*>::C
    }};
    ( < $($generic_ty:ident $( : $lft_bound:lifetime )? ),+ ,, $(const $generic_const:ident : $generic_const_ty:ty),+ $(,)? > [ $t:ty ] $e:expr) => {
        inline_const!(@parsed <$($generic_ty $(: $lft_bound)? ),*> <$(const $generic_const: $generic_const_ty),*> [$t] $e)
    };
    ( < $(const $generic_const:ident : $generic_const_ty:ty),+ $(,)? > [ $t:ty ] $e:expr) => {
        inline_const!(@parsed <> <$(const $generic_const: $generic_const_ty),*> [$t] $e)
    };
    ( < $($generic_ty:ident $( : $lft_bound:lifetime )? ),+ $(,)? > [ $t:ty ] $e:expr) => {
        inline_const!(@parsed <$($generic_ty $(: $lft_bound)? ),*> <> [$t] $e)
    };
    ( [ $t:ty ] $e:expr) => {
        inline_const!(@parsed <> <> [$t] $e)
    };
}

#[macro_export]
macro_rules! const_array {
    ( @parsed < $($generic_ty:ident $( : $lft_bound:lifetime )? ),* > < $(const $generic_const:ident : $generic_const_ty:ty),* > [ $t:ty ] $e:expr ; $N:expr) => {{
        // We must actually use all the const parameters. To avoid adding `Sized` constraints, we
        // use them behind a pointer indirection.
        struct Const<$($generic_ty $(: $lft_bound)?,)* $(const $generic_const: $generic_const_ty,)*>($(*mut $generic_ty),*);
        impl<$($generic_ty $(: $lft_bound)?,)* $(const $generic_const: $generic_const_ty,)*> Const<$($generic_ty,)* $($generic_const,)*> {
            const C: $t = $e;
        }
        [Const::<$($generic_ty,)* $($generic_const,)*>::C; $N]
    }};
    ( < $($generic_ty:ident $( : $lft_bound:lifetime )? ),+ ,, $(const $generic_const:ident : $generic_const_ty:ty),+ $(,)? > [ $t:ty ] $e:expr ; $N:expr) => {
        const_array!(@parsed <$($generic_ty $(: $lft_bound)? ),*> <$(const $generic_const: $generic_const_ty),*> [$t] $e; $N)
    };
    ( < $(const $generic_const:ident : $generic_const_ty:ty),+ $(,)? > [ $t:ty ] $e:expr ; $N:expr) => {
        const_array!(@parsed <> <$(const $generic_const: $generic_const_ty),*> [$t] $e; $N)
    };
    ( < $($generic_ty:ident $( : $lft_bound:lifetime )? ),+ $(,)? > [ $t:ty ] $e:expr ; $N:expr) => {
        const_array!(@parsed <$($generic_ty $(: $lft_bound)? ),*> <> [$t] $e; $N)
    };
    ( [ $t:ty ] $e:expr ; $N:expr) => {
        const_array!(@parsed <> <> [$t] $e; $N)
    };
}
