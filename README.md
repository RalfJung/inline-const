# inline-const

You don't want to wait until [inline consts](https://rust-lang.github.io/rfcs/2920-inline-const.html) are stable?
This crate offers much of the functionality of inline consts in a pure macro-based implementation, at a slight const of convenience:
you need to explicitly annotate the type of the constant.
```rust
use std::net::Ipv6Addr;

use inline_const::inline_const;

fn mock_ip(use_localhost: bool) -> &'static Ipv6Addr {
    if use_localhost {
        &Ipv6Addr::LOCALHOST
    } else {
        inline_const! { [&'static Ipv6Addr]
            &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)
        }
    }
}
```

Unlike the current unstable implementation of inline-const, this crate even supports consts that depend on generic parameters,
albeit at some further annotation cost: you need to repeat the generic parameters that the constant refers to, and their lifetime bounds.
```rust
use inline_const::inline_const;

fn make_static_vec<T: 'static>() -> &'static Vec<T>{
    inline_const! { <T: 'static> [&'static Vec<T>] &Vec::new() }
}
```

### Static assertions

This can be used to implement static assertions that depend on generic parameters:
```rust
use inline_const::inline_const;

#[allow(unconditional_panic)]
const fn assert(b: bool) {
    if !b {
        ["const assertion failed"][1];
    }
}

fn size_at_least_2<T>() {
    inline_const!{ <T> [()] assert(std::mem::size_of::<T>() >= 2)};
}
fn const_at_least_2<const N: usize>() {
    inline_const!{ <const N: usize> [()] assert(N >= 2)};
}
// When an inline const depends on both types and const generics, a double-comma `,,` must be used
// to separate the two.
fn size_at_least<T, const N : usize>() {
    inline_const!{ <T,, const N: usize> [()] assert(std::mem::size_of::<T>() >= N)};
}

size_at_least_2::<i32>();
//size_at_least_2::<i8>();
const_at_least_2::<4>();
//const_at_least_2::<1>();
size_at_least::<i8, 1>();
//size_at_least::<i8, 2>();
```

### Array of constants

Since recently, `[C; N]` works for any constant `C`, even for non-`Copy` types.
However, without inline consts, this is somewhat annoying to use.
This crate offers the `const_array!` macro to help with that situation:

```rust
use inline_const::const_array;

fn make_i32_vecs() -> [Vec<i32>; 5] {
    // [Vec::new(); 5]: rejected since `Vec::new` is not a constant.
    const_array![ [Vec<i32>] Vec::new(); 5]
}
fn make_vecs<T>() -> [Vec<T>; 5] {
    // Generic parameters used in the const expression must be explicitly specified:
    const_array![<T> [Vec<T>] Vec::new(); 5]
}
fn make_n_vecs<T, const N: usize>() -> [Vec<T>; N] {
    const_array![<T> [Vec<T>] Vec::new(); N]
}
```
