#[test]
fn test_macro_json() {
    use rust_json::{json, json_parse};
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

#[test]
fn test_macro_js_object() {
    use rust_json::{js_object, json};
    assert_eq!(js_object!([{"nest": [{}]}]), json!([{"nest": [{}]}]));
    let n = 1;
    assert_eq!(
        js_object!([{ n: n }, { n: n }]),
        json!([{"n" : 1}, {"n": 1}])
    );
    assert_eq!(
        js_object!([{n : n * 2}, {n : n.to_string()}]),
        json!([{"n" : 2}, {"n": "1"}])
    );

    assert_eq!(js_object!([1, [2, [3]]]), json!([1, [2, [3]]]));
    assert_eq!(
        js_object!({a: {b: {c: {}}}}),
        json!({"a": {"b": {"c": {}}}})
    );
    assert_eq!(
        js_object!({a: [{b: [{c: [{}]}]}]}),
        json!({"a": [{"b": [{"c": [{}]}]}]})
    );

    assert_eq!(js_object!({ n }), json!({"n": 1}));

    let b = true;

    assert_eq!(js_object!([{n, b}]), json!([{"n": 1, "b": true}]));
    assert_eq!(
        js_object!([{ [n + 21]: n }, { [n + 12]: b }]),
        json!([{"22": 1}, {"13": true}])
    );
}

#[test]
fn test_macro_json_full_qualified() {
    assert_eq!(Ok(rust_json::json!([])), rust_json::json_parse("[]"));
    assert_eq!(Ok(rust_json::json!({})), rust_json::json_parse("{}"));
    assert_eq!(Ok(rust_json::json!(null)), rust_json::json_parse("null"));
    assert_eq!(Ok(rust_json::json!(true)), rust_json::json_parse("true"));
    assert_eq!(Ok(rust_json::json!(false)), rust_json::json_parse("false"));
    assert_eq!(
        Ok(rust_json::json!([1, [2, [3]]])),
        rust_json::json_parse("[1,[2,[3]]]")
    );
    assert_eq!(
        Ok(rust_json::json!([{"a": 1, "b": 2}, {"a": 3, "b": 4}])),
        rust_json::json_parse(r#"[{"a": 1, "b": 2}, {"a": 3, "b": 4}]"#)
    );
    assert_eq!(
        Ok(rust_json::json!([{"a": 1, "b": 2}])),
        rust_json::json_parse(r#"[{"a": 1, "b": 2}]"#)
    );
    assert_eq!(
        Ok(rust_json::json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        })),
        rust_json::json_parse(
            r#"{"name": "John Doe","age": 43,"phones": ["+44 1234567","+44 2345678"]}"#
        )
    );
}

#[test]
fn test_macro_js_object_full_qualified() {
    assert_eq!(
        rust_json::js_object!([{"nest": [{}]}]),
        rust_json::json!([{"nest": [{}]}])
    );
    let n = 1;
    assert_eq!(
        rust_json::js_object!([{ n: n }, { n: n }]),
        rust_json::json!([{"n" : 1}, {"n": 1}])
    );
    assert_eq!(
        rust_json::js_object!([{n : n * 2}, {n : n.to_string()}]),
        rust_json::json!([{"n" : 2}, {"n": "1"}])
    );

    assert_eq!(
        rust_json::js_object!([1, [2, [3]]]),
        rust_json::json!([1, [2, [3]]])
    );
    assert_eq!(
        rust_json::js_object!({a: {b: {c: {}}}}),
        rust_json::json!({"a": {"b": {"c": {}}}})
    );
    assert_eq!(
        rust_json::js_object!({a: [{b: [{c: [{}]}]}]}),
        rust_json::json!({"a": [{"b": [{"c": [{}]}]}]})
    );

    assert_eq!(rust_json::js_object!({ n }), rust_json::json!({"n": 1}));

    let b = true;

    assert_eq!(
        rust_json::js_object!([{n, b}]),
        rust_json::json!([{"n": 1, "b": true}])
    );
    assert_eq!(
        rust_json::js_object!([{ [n + 21]: n }, { [n + 12]: b }]),
        rust_json::json!([{"22": 1}, {"13": true}])
    );
}
