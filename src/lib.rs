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

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __generate_assoc_const {
    (
        $callback:ident @parsed
        < $($generic_ty:ident $( : $lft_bound:lifetime )? ),* > < $(const $generic_const:ident : $generic_const_ty:ty),* >
        [ $t:ty ] $e:expr ;
        $($cont:tt)*
    ) => {{
        // We must actually use all the const parameters. To avoid adding `Sized` constraints, we
        // use them behind a pointer indirection.
        struct Const<$($generic_ty $(: $lft_bound)?,)* $(const $generic_const: $generic_const_ty,)*>($(*mut $generic_ty),*);
        impl<$($generic_ty $(: $lft_bound)?,)* $(const $generic_const: $generic_const_ty,)*> Const<$($generic_ty,)* $($generic_const,)*> {
            const C: $t = $e;
        }
        $callback!(Const::<$($generic_ty,)* $($generic_const,)*>::C, $($cont)*)
    }};
    (
        $callback:ident @parse
        < $($generic_ty:ident $( : $lft_bound:lifetime )? ),+ ; $(const $generic_const:ident : $generic_const_ty:ty),+ $(,)? >
        $($cont:tt)*
    ) => {
        __generate_assoc_const!($callback @parsed <$($generic_ty $(: $lft_bound)? ),*> <$(const $generic_const: $generic_const_ty),*> $($cont)*)
    };
    (
        $callback:ident @parse < $(const $generic_const:ident : $generic_const_ty:ty),+ $(,)? >
        $($cont:tt)*
    ) => {
        __generate_assoc_const!($callback @parsed <> <$(const $generic_const: $generic_const_ty),*> $($cont)*)
    };
    (
        $callback:ident @parse < $($generic_ty:ident $( : $lft_bound:lifetime )? ),* $(,)? > // covers the empty case
        $($cont:tt)*
    ) => {
        __generate_assoc_const!($callback @parsed <$($generic_ty $(: $lft_bound)? ),*> <> $($cont)*)
    };
    (
        $callback:ident @parse [ $t:ty ] $($cont:tt)*
    ) => {
        __generate_assoc_const!($callback @parsed <> <> [$t] $($cont)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inline_const_callback {
    ($const:path, @emp) => {
        $const
    }
}
#[macro_export(local_inner_macros)]
macro_rules! inline_const {
    ( $($params:tt)* ) => {
        __generate_assoc_const!(__inline_const_callback @parse $($params)* ; @emp)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __const_array_callback {
    ($const:path, $N:expr) => {
        [$const; $N]
    }
}
#[macro_export(local_inner_macros)]
macro_rules! const_array {
    ( $($params:tt)* ) => {
        // Conveniently, the surface user syntax already contains a `;` and then the remaining
        // arguments, so we just re-use those for the `;` expected by __generate_assoc_const.
        __generate_assoc_const!(__const_array_callback @parse $($params)*)
    };
}
