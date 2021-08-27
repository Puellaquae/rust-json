use rust_json::{json, ToJson};
use rust_json_derive::ToJson;

#[derive(ToJson)]
struct S {
    n: f64,
    b: bool,
}

#[derive(ToJson)]
struct N {
    a: Vec<f64>,
    s: S,
}

#[derive(ToJson)]
struct Unit;

#[derive(ToJson)]
struct T(N, Unit);

#[test]
fn test_derive_to_json() {
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
