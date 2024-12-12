[![Crates.io](https://img.shields.io/crates/v/macro-into.svg)](https://crates.io/crates/macro-into)
[![Docs](https://docs.rs/macro-into/badge.svg)](https://docs.rs/crate/macro-into/)
[![Download](https://img.shields.io/crates/d/macro-into.svg?style=flat-square)](https://crates.io/crates/macro-into)

# Macro Into

Rust macro for auto impl Into\<T> or From\<T> for Struct

## Usage

impl Into\<Bar> for Foo
```rust
struct Foo {
    field1: i32,
    field3: String,
}

#[into(Foo)]
struct Bar {
    field1: i32,
    #[into_skip]
    field2: String,
    #[into(self.field3.to_string())]
    field3: i32,
}

```
自动生成以下代码:
```rust
impl Into<Foo> for Bar {
    fn into(self) -> Foo {
        Foo {
            field1: self.field1,
            field3: self.field3.to_string(),
        }
    }
}
```

impl From\<Bar> for Foo
```rust
struct Foo {
    field1: i32,
    field2: String,
}

#[from(Foo)]
struct Bar {
    field1: i32,
    #[from(source.field2.parse::<i32>().unwrap())]
    field3: i32,
}

```
自动生成以下代码:
```rust
impl From<Foo> for Bar {
    fn from(source: Foo) -> Self {
        Bar {
            field1: source.field1,
            field3: source.field2.parse::<i32>().unwrap(),
        }
    }
}
```