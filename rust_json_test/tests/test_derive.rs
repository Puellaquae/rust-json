use rust_json::{json, json_parse, FromJson, ToJson};
use rust_json_derive::{FromJson, ToJson};

#[derive(Debug, PartialEq, ToJson, FromJson)]
struct S {
    n: f64,
    b: bool,
}

#[derive(Debug, PartialEq, ToJson, FromJson)]
struct N {
    a: Vec<f64>,
    s: S,
}

#[derive(Debug, PartialEq, ToJson, FromJson)]
struct Unit;

#[derive(Debug, PartialEq, ToJson, FromJson)]
struct T(N, Unit);

#[derive(Debug, PartialEq, ToJson, FromJson)]
enum E {
    Unit,
    One(i32),
    Two(i32, i32),
    Cmpx { a: i32, b: i32, c: i32 },
}

#[test]
fn test_derive_to_json_struct() {
    let s = S { n: 12.3, b: true };
    assert_eq!(json!({"n": 12.3, "b": true}), s.to_json());

    let n = N {
        a: vec![1.2, 2.3],
        s: s,
    };
    assert_eq!(
        json!({"a":[1.2,2.3], "s": {"n": 12.3, "b": true}}),
        n.to_json()
    );

    let u = Unit;
    assert_eq!(json!("Unit"), u.to_json());

    let t = T(n, u);
    assert_eq!(
        json!([{"a":[1.2,2.3], "s": {"n": 12.3, "b": true}}, "Unit"]),
        t.to_json()
    );
}

#[test]
fn test_derive_to_json_enum() {
    let u = E::Unit;
    let o = E::One(1);
    let t = E::Two(1, 2);
    let c = E::Cmpx { a: 1, b: 2, c: 3 };
    assert_eq!(json!("Unit"), u.to_json());
    assert_eq!(json!({"One": 1}), o.to_json());
    assert_eq!(json!({"Two": [1, 2]}), t.to_json());
    assert_eq!(json!({"Cmpx": {"a": 1, "b": 2, "c": 3}}), c.to_json());
}

#[test]
fn test_derive_from_json_struct() {
    let s = S { n: 12.3, b: true };
    assert_eq!(Some(s), s.to_json().get());

    let n = N {
        a: vec![1.2, 2.3],
        s: s,
    };
    assert_eq!(
        Some(n),
        n.to_json().get()
    );

    let u = Unit;
    assert_eq!(Some(u), u.to_json().get());

    let t = T(n, u);
    assert_eq!(
        Some(t),
        t.to_json().get()
    );
}
