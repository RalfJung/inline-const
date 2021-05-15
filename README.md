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
```
