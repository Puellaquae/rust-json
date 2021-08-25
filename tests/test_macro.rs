use rust_json::{json, json_parse};

#[test]
fn test_macro_json() {
    assert_eq!(Ok(json!([])), json_parse("[]"));
    assert_eq!(Ok(json!({})), json_parse("{}"));
    assert_eq!(Ok(json!(null)), json_parse("null"));
    assert_eq!(Ok(json!(true)), json_parse("true"));
    assert_eq!(Ok(json!(false)), json_parse("false"));
    assert_eq!(Ok(json!([1, [2, [3]]])), json_parse("[1,[2,[3]]]"));
    assert_eq!(
        Ok(json!([{"a": 1, "b": 2}, {"a": 3, "b": 4}])),
        json_parse(r#"[{"a": 1, "b": 2}, {"a": 3, "b": 4}]"#)
    );
    assert_eq!(
        Ok(json!([{"a": 1, "b": 2}])),
        json_parse(r#"[{"a": 1, "b": 2}]"#)
    );
    assert_eq!(
        Ok(json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        })),
        json_parse(r#"{"name": "John Doe","age": 43,"phones": ["+44 1234567","+44 2345678"]}"#)
    );
}
