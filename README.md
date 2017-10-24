# TypeDef

Identify or compare types, get or print type names.

Since Rust 1.0, this library can only work on nightly Rust. To activate the nice names instead
of gobbledygook, include this library with `features = ["nightly"]` configuration parameter.
On stable rust, it falls back to gobbledygook (type identifier) instead of a nice name.

[![Build Status](https://travis-ci.org/Nercury/typedef-rs.svg?branch=master)](https://travis-ci.org/Nercury/typedef-rs)

## Quick example

```rust
use typedef::{ TypeDef };

// type name querying:

assert!(TypeDef::name_of::<i64>() == "i64");

// and also type value:

let typedef = TypeDef::of::<i64>();

assert!(typedef.is::<i64>());
assert!(typedef.get_str() == "i64");
assert!(typedef == TypeDef::of::<i64>());

println!("type is {:?}", typedef);
```

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
typedef = "*"
```

And this in your crate root:

```rust
extern crate typedef;
```

## Resources

- [Use `TypeId` if you do not need type names](http://doc.rust-lang.org/std/intrinsics/struct.TypeId.html)
- [Full `TypeDef` documentation](http://nercury.github.io/typedef-rs)
- [`TypeDef` on crates.io](https://crates.io/crates/typedef)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Contains work Copyright 2015 Sean Kerr, Apache License, Version 2.0. Files
under this license can be identified by their headers.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.