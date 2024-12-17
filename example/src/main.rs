#![allow(unused)]
use into_from::{from, into};

#[derive(Debug, Default)]
struct Foo {
    a: i32,
    b: String,
    c: i32,
}

#[derive(Debug)]
#[into(Foo, default)]
struct Bar {
    a: i32,
    b: String,
}

#[derive(Debug)]
#[from(Foo)]
struct Baz {
    a: i32,
    b: String,
}

fn main() {
    let bar = Bar {
        a: 1,
        b: "2".to_string(),
    };
    let f: Foo = bar.into();
    println!("{:?}", f);

    let b = Baz::from(f);
    println!("{:?}", b);
}
