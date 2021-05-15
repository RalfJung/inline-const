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
