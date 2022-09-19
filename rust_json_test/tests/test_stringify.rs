use rust_json::json;

#[test]
fn test_stringify_simple() {
    assert_eq!(
        r#"{
    "a": 12
}"#,
        format!("{:#}", json!({"a": 12}))
    );

    assert_eq!(
        r#"[
    1,
    2,
    3
]"#,
        format!("{:#}", json!([1, 2, 3]))
    );
}

#[test]
fn test_stringify_nest() {
    assert_eq!(
        r#"{
    "a": [
        1,
        2,
        null
    ]
}"#,
        format!("{:#}", json!({"a": [1, 2, null]}))
    );

    assert_eq!(
        r#"[
    1,
    {
        "c": [
            false,
            true,
            "abc"
        ]
    },
    3
]"#,
        format!("{:#}", json!([1, {"c": [false, true, "abc"]}, 3]))
    );
}

#[test]
fn test_stringify_custom_width() {
    assert_eq!(
        r#"{
  "a": [
    1,
    2,
    null
  ]
}"#,
        format!("{:2}", json!({"a": [1, 2, null]}))
    );

    assert_eq!(
        r#"[
        1,
        {
                "c": [
                        false,
                        true,
                        "abc"
                ]
        },
        3
]"#,
        format!("{:8}", json!([1, {"c": [false, true, "abc"]}, 3]))
    );
}
