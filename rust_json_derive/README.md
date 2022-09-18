# rust_json_derive

[![Latest Version](https://img.shields.io/crates/v/rust_json_derive.svg)](https://crates.io/crates/rust_json_derive)

Derive the `ToJson` and `FromJson` for [rust_json](https://crates.io/crates/rust_json).

### `#[derive(ToJson)]`
```rust
use rust_json::ToJson;
use rust_json_derive::ToJson;

#[derive(ToJson)]
struct Simple {
    n: f64,
    b: bool,
}

#[derive(ToJson)]
struct Nest {
    a: Vec<f64>,
    s: Simple,
}

#[derive(ToJson)]
enum Enum {
    Unit,
    One(i32),
    Two(i32, i32),
    Cmpx { a: i32, b: i32, c: i32 },
}

fn main() {
    let s = Simple { n: 12.3, b: true };
    println!("{}", s.to_json());

    let n = Nest {
        a: vec![1.2, 2.3],
        s: s,
    };
    println!("{}", n.to_json());

    let u = E::Unit;
    let o = E::One(1);
    let t = E::Two(1, 2);
    let c = E::Cmpx { a: 1, b: 2, c: 3 };
    println!("{}", u.to_json());
    println!("{}", o.to_json());
    println!("{}", t.to_json());
    println!("{}", c.to_json());
}
```

### `#[derive(FromJson)]`
```rust
use rust_json::json_parse;
use rust_json_derive::FromJson;

#[derive(Debug, FromJson)]
struct Simple {
    n: f64,
    b: bool,
}

#[derive(Debug, FromJson)]
struct Nest {
    a: Vec<f64>,
    s: Simple,
}

#[derive(Debug, FromJson)]
enum Enum {
    Unit,
    One(i32),
    Two(i32, i32),
    Cmpx { a: i32, b: i32, c: i32 },
}

fn main() {
    println!("{:?}", json_parse(r#"{"n": 12.3, "b": true}"#).get::<Simple>());

    println!("{:?}", json_parse(r#"
    {
        "a": [1.2, 2.3],
        "s": {"n": 12.3, "b": true}
    }
    "#).get::<Nest>());

    println!("{:?}", json_parse(r#""Unit""#).get::<Enum>());
    
    println!("{:?}", json_parse(r#"{"One": 1}"#).get::<Enum>());

    println!("{:?}", json_parse(r#"
    {
        "Two": [1, 2]
    }
    "#).get::<Enum>());

    println!("{:?}", json_parse(r#"
    {
        "Cmpx": {"a": 1, "b": 2, "c": 3}
    }
    "#).get::<Enum>());
}
```
