[![Crates.io](https://img.shields.io/crates/v/into-from.svg)](https://crates.io/crates/into-from)
[![Docs](https://docs.rs/into-from/badge.svg)](https://docs.rs/crate/into-from/)
[![Download](https://img.shields.io/crates/d/into-from.svg?style=flat-square)](https://crates.io/crates/into-from)

# Macro Into

Rust macro for auto impl Into\<T> or From\<T> for Struct

## Usage

impl Into\<Bar> for Foo
```rust
#[derive(Debug, Default)]
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

Auto generated code:
```rust
impl Into<Foo> for Bar {
    fn into(self) -> Foo {
        Foo {
            field1: self.field1,
            field3: self.field3.to_string(),
            ...Default::default()
        }
    }
}
```

```rust
#[into(Foo, default)]
struct BarDefault {
    #[into_skip]
    field2: String,
    #[into(self.field3.to_string())]
    field3: i32,
}
```

Auto generated code:
```rust
impl Into<Foo> for BarDefault {
    fn into(self) -> Foo {
        Foo {
            field3: self.field3.to_string(),
            ...Default::default()
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
Auto generated code:
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