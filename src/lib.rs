//! You don't want to wait until [inline consts](https://rust-lang.github.io/rfcs/2920-inline-const.html) are stable?
//! This crate offers much of the functionality of inline consts in a pure macro-based implementation, at a slight const of convenience:
//! you need to explicitly annotate the type of the constant.
//! ```rust
//! use std::net::Ipv6Addr;
//! 
//! use inline_const::inline_const;
//! 
//! fn mock_ip(use_localhost: bool) -> &'static Ipv6Addr {
//!     if use_localhost {
//!         &Ipv6Addr::LOCALHOST
//!     } else {
//!         inline_const! { [&'static Ipv6Addr]
//!             &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)
//!         }
//!     }
//! }
//! ```
//! 
//! Unlike the current unstable implementation of inline-const, this crate even supports consts that depend on generic parameters,
//! albeit at some further annotation cost: you need to repeat the generic parameters that the constant refers to, and their lifetime bounds.
//! ```rust
//! use inline_const::inline_const;
//! 
//! fn make_static_vec<T: 'static>() -> &'static Vec<T>{
//!     inline_const! { <T: 'static> [&'static Vec<T>] &Vec::new() }
//! }
//! ```

#[cfg(all(const_generics, doctest))]
#[macro_use]
extern crate doc_comment;
#[cfg(all(const_generics, doctest))]
doctest!("../README.md");

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

/// Macro to generate a constant as an inline expression.
/// 
/// See [module-level docs](index.html) for examples.
#[macro_export(local_inner_macros)]
macro_rules! inline_const {
    ( $($params:tt)* ) => {
        __generate_assoc_const!(__inline_const_callback @parse $($params)* ; @emp)
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __inline_const_callback {
    ($const:path, @emp) => {
        $const
    }
}

/// Macro to generate an array by repeating a constant `N` times.
/// 
/// Since recently, `[C; N]` works for any constant `C`, even for non-`Copy` types.
/// However, without inline consts, this is somewhat annoying to use.
/// The `const_array!` macro helps with that situation:
/// 
/// ```rust
/// use inline_const::const_array;
/// 
/// fn make_i32_vecs() -> [Vec<i32>; 5] {
///     // [Vec::new(); 5]: rejected since `Vec::new` is not a constant.
///     const_array![ [Vec<i32>] Vec::new(); 5]
/// }
/// fn make_vecs<T>() -> [Vec<T>; 5] {
///     // Generic parameters used in the const expression must be explicitly specified:
///     const_array![<T> [Vec<T>] Vec::new(); 5]
/// }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! const_array {
    ( $($params:tt)* ) => {
        // Conveniently, the surface user syntax already contains a `;` and then the remaining
        // arguments, so we just re-use those for the `;` expected by __generate_assoc_const.
        __generate_assoc_const!(__const_array_callback @parse $($params)*)
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __const_array_callback {
    ($const:path, $N:expr) => {
        [$const; $N]
    }
}
