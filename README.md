# TypeDef

Identify or compare types, get or print type names.

Since Rust 1.0, this library can only work on nightly Rust.

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

MIT
