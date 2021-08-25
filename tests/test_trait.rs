use rust_json::{json, json_parse, JsonElem, JsonElem::*, ToJson};
use std::collections::HashMap;

macro_rules! map {
    ($($key: expr => $val: expr), *) => {{
        let mut hm = HashMap::new();
        $( hm.insert(String::from($key), $val); )*
        hm
    }};
}

struct StructA {
    field_a: i32,
    field_b: bool,
    field_c: String,
}

impl ToJson for StructA {
    fn to_json(&self) -> JsonElem {
        Object(map!(
            "field_a" => self.field_a.to_json(),
            "field_b" => self.field_b.to_json(),
            "field_c" => self.field_c.to_json()
        ))
    }
}

#[test]
fn test_to_json() {
    assert_eq!(Bool(true), true.to_json());
    assert_eq!(Bool(false), false.to_json());
    assert_eq!(Number(123.0), 123.to_json());
    assert_eq!(Number(123.4), 123.4.to_json());
    assert_eq!(Str(String::from("abcd")), "abcd".to_json());
    let arr = vec![Null, Bool(true), Number(12.3)];
    assert_eq!(json!([null, true, 12.3]), arr.to_json());
    assert_eq!(json!([null, true]), arr[..2].to_json());

    let num_arr = vec![1, 2, 3];

    assert_eq!(json!([1, 2, 3]), num_arr.to_json());

    assert_eq!(
        json_parse(r#"{"field_a":1,"field_b":true,"field_c":"123"}"#),
        Ok(StructA {
            field_a: 1,
            field_b: true,
            field_c: String::from("123")
        }
        .to_json())
    );

    let obj = map!(
        "a" => "a",
        "b" => "b",
        "c" => "c"
    );

    assert_eq!(json!({"a":"a","b":"b","c":"c"}), obj.to_json());

    let nest_obj = map!(
        "a" => vec![1,2],
        "b" => vec![2,3],
        "c" => vec![3,4]
    );

    assert_eq!(json!({"a":[1,2],"b":[2,3],"c":[3,4]}), nest_obj.to_json());
}

#[test]
fn test_serialize() {
    assert_eq!("null", Null.to_string().as_str());
    assert_eq!("true", Bool(true).to_string().as_str());
    assert_eq!("false", Bool(false).to_string().as_str());
    assert_eq!("123", Number(123.0).to_string().as_str());
    assert_eq!("123.4", Number(123.4).to_string().as_str());
    assert_eq!(
        "[null,true,false,3.14]",
        Array(vec![Null, Bool(true), Bool(false), Number(3.14)])
            .to_string()
            .as_str()
    );
    assert_eq!(
        "\"\\t\\r\\n\\u0001\"",
        Str(format!("\t\r\n\u{1}")).to_string().as_str()
    );

    let j = json!({"\t": true, "\u{c}": false});
    let serj = j.to_string();
    assert_eq!(Ok(j), json_parse(serj.as_str()))
}
