# cutlass

> experimental auto-currying for rust functions

### example

this currently works only on nightly with the
`type_alias_impl_trait` feature enabled.

```rust
#![feature(type_alias_impl_trait)]

#[cutlass::curry]
fn add(x: u32, y: u32, z: u32) -> u32 {
    return x + y + z;
}

fn main() {
    let plus_3 = add(1)(2);
    let v: Vec<u32> = (1..=3).map(plus_3).collect();
    assert_eq!(v, vec![4, 5, 6]);
}
```

### how it works

the `#[curry]` proc-macro expands the above `add` function to
something like this (roughly):

```rust
type T0 = u32;
type T1 = impl Fn(u32) -> T0;
type T2 = impl Fn(u32) -> T1;
fn add(x: u32) -> T2 {
    return (move |y| move |z| x + y + z);
}
```

### gotchas

 - doesn't yet work for method functions (signatures with
   `self`)
 - the function has to have a return value
 - works only on nightly with `type_alias_impl_trait`
   enabled

### testing

test this crate with `cargo +nightly test`. 

to view macro expansions, install `cargo-expand` and run `cargo expand
--test <test name>`. expand `tests/smoke.rs` for example:

```
cargo expand --test smoke
```
