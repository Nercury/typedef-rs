# TypeDef

Identify or compare types, get or print type names.

[![Build Status](https://travis-ci.org/Nercury/typedef-rs.svg?branch=master)](https://travis-ci.org/Nercury/typedef-rs)

## Quick example

```rust
use typedef::{ TypeDef };

let typedef = TypeDef::of::<int>();

assert!(typedef.is::<int>());
assert!(typedef.get_str() == "int");
assert!(typedef == TypeDef::of::<int>());

println!("type is {}", typedef);
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
