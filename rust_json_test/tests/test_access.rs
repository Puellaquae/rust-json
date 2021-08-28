use rust_json::{json, JsonElem};

#[test]
fn test_get() {
    assert_eq!(Some(true), json!(true).get());
    assert_eq!(Some(12.3), json!(12.3).get());
    assert_eq!(Some("abc".to_string()), json!("abc").get());
    assert_eq!(Some(vec![json!(1), json!(null), json!(true)]), json!([1, null, true]).get());
    assert_eq!(None, json!("abc").get::<f64>());
}

#[test]
fn test_index() {
    let j = json!({
        "a": [1, null, true],
        "b": {
            "c": false
        }
    });
    assert_eq!(JsonElem::Null, j[1]);
    assert_eq!(JsonElem::Number(1.0), j["a"][0]);
    assert_eq!(JsonElem::Bool(false), j["b"]["c"]);
}
