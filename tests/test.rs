use rust_json::{js_object, json, json_parse, JsonElem, JsonElem::*, JsonParseErr::*, ToJson};
use std::collections::HashMap;

#[test]
fn test_null() {
    assert_eq!(Ok(Null), json_parse("null"));
    assert_eq!(Ok(Null), json_parse("   null   "));
    assert_eq!(Ok(Null), json_parse("\r\n\t\r\nnull\r\t\n"));
}

#[test]
fn test_true() {
    assert_eq!(Ok(Bool(true)), json_parse("true"));
    assert_eq!(Ok(Bool(true)), json_parse("   true    "));
    assert_eq!(Ok(Bool(true)), json_parse("\t\t\ttrue\r\n"));
}

#[test]
fn test_false() {
    assert_eq!(Ok(Bool(false)), json_parse("false"));
    assert_eq!(Ok(Bool(false)), json_parse("   false    "));
    assert_eq!(Ok(Bool(false)), json_parse("\t\t\tfalse\r\n"));
}

#[test]
fn test_number() {
    assert_eq!(Ok(Number(0.0)), json_parse("0"));
    assert_eq!(Ok(Number(0.0)), json_parse("-0"));
    assert_eq!(Ok(Number(0.0)), json_parse("-0.0"));
    assert_eq!(Ok(Number(1.0)), json_parse("1"));
    assert_eq!(Ok(Number(-1.0)), json_parse("-1"));
    assert_eq!(Ok(Number(1.5)), json_parse("1.5"));
    assert_eq!(Ok(Number(-1.5)), json_parse("-1.5"));
    assert_eq!(Ok(Number(3.1415)), json_parse("3.1415"));
    assert_eq!(Ok(Number(1e10)), json_parse("1E10"));
    assert_eq!(Ok(Number(1e10)), json_parse("1e10"));
    assert_eq!(Ok(Number(1e+10)), json_parse("1E+10"));
    assert_eq!(Ok(Number(1e-10)), json_parse("1e-10"));
    assert_eq!(Ok(Number(-1e10)), json_parse("-1e10"));
    assert_eq!(Ok(Number(-1.1234e10)), json_parse("-1.1234e10"));
    assert_eq!(Ok(Number(1.1234e-10)), json_parse("1.1234E-10"));

    assert_eq!(
        Ok(Number(1.0000000000000002)),
        json_parse("1.0000000000000002")
    );
    assert_eq!(
        Ok(Number(4.9406564584124654e-324)),
        json_parse("4.9406564584124654e-324")
    );
    assert_eq!(
        Ok(Number(-4.9406564584124654e-324)),
        json_parse("-4.9406564584124654e-324")
    );
    assert_eq!(
        Ok(Number(2.2250738585072009e-308)),
        json_parse("2.2250738585072009e-308")
    );
    assert_eq!(
        Ok(Number(-2.2250738585072009e-308)),
        json_parse("-2.2250738585072009e-308")
    );
    assert_eq!(
        Ok(Number(2.2250738585072014e-308)),
        json_parse("2.2250738585072014e-308")
    );
    assert_eq!(
        Ok(Number(-2.2250738585072014e-308)),
        json_parse("-2.2250738585072014e-308")
    );
    assert_eq!(
        Ok(Number(1.7976931348623157e+308)),
        json_parse("1.7976931348623157e+308")
    );
    assert_eq!(
        Ok(Number(-1.7976931348623157e+308)),
        json_parse("-1.7976931348623157e+308")
    );
}

fn test_string_help(s1: &str, s2: &str) {
    assert_eq!(Ok(Str(String::from(s1))), json_parse(s2));
}

#[test]
fn test_string() {
    test_string_help("", "\"\"");
    test_string_help("Hello", "\"Hello\"");
    test_string_help("Hello\nWorld", "\"Hello\\nWorld\"");
    test_string_help("\" \\ / \n \r \t", "\"\\\" \\\\ \\/ \\n \\r \\t\"");
    test_string_help("Hello\0World", "\"Hello\\u0000World\"");
    test_string_help("$", "\"\\u0024\""); /* Dollar sign U+0024 */
    test_string_help("¢", "\"\\u00A2\""); /* Cents sign U+00A2 */
    test_string_help("€", "\"\\u20AC\""); /* Euro sign U+20AC */
    test_string_help("𝄞", "\"\\uD834\\uDD1E\""); /* G clef sign U+1D11E */
    test_string_help("𝄞", "\"\\ud834\\udd1e\""); /* G clef sign U+1D11E */
}

#[test]
fn test_array() {
    assert_eq!(
        Ok(Array(vec![
            Null,
            Bool(false),
            Bool(true),
            Number(123f64),
            Str(String::from("abc"))
        ])),
        json_parse("[ null , false , true , 123 , \"abc\" ]")
    );

    assert_eq!(
        Ok(Array(vec![
            Array(vec![]),
            Array(vec![Number(0.0)]),
            Array(vec![Number(0.0), Number(1.0)]),
            Array(vec![Number(0.0), Number(1.0), Number(2.0)]),
        ])),
        json_parse("[ [ ] , [ 0 ] , [ 0 , 1 ] , [ 0 , 1 , 2 ] ]")
    );
}

macro_rules! map {
    ($($key: expr => $val: expr), *) => {{
        let mut hm = HashMap::new();
        $( hm.insert(String::from($key), $val); )*
        hm
    }};
}

#[test]
fn test_object() {
    let obj = map!(
        "n" => Null,
        "f" => Bool(false),
        "t" => Bool(true),
        "i" => Number(123.0),
        "s" => Str(String::from("abc")),
        "a" => Array(vec![Number(1.0), Number(2.0), Number(3.0)]),
        "o" => Object(map!("1" => Number(1.0), "2" => Number(2.0), "3" => Number(3.0)))
    );

    assert_eq!(
            Ok(Object(obj)),
            json_parse(" { \"n\" : null , \"f\" : false , \"t\" : true , \"i\" : 123 , \"s\" : \"abc\", \"a\" : [ 1, 2, 3 ],\"o\" : { \"1\" : 1, \"2\" : 2, \"3\" : 3 } } ")
        );
}

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
    assert_eq!(
        json_parse(r#"{"field_a":1,"field_b":true,"field_c":"123"}"#),
        Ok(StructA {
            field_a: 1,
            field_b: true,
            field_c: String::from("123")
        }
        .to_json())
    )
}

#[test]
fn test_macro_js_object() {
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
fn test_expect_value() {
    assert_eq!(Err(ExpectValue), json_parse(""));
    assert_eq!(Err(ExpectValue), json_parse("  \r\n  "));
}

#[test]
fn test_invalid_value() {
    assert_eq!(Err(InvalidValue), json_parse("nul"));
    assert_eq!(Err(InvalidValue), json_parse("tr ue"));
}

#[test]
fn test_root_not_singular() {
    assert_eq!(Err(RootNotSingular), json_parse("nulll"));
    assert_eq!(Err(RootNotSingular), json_parse("{\"true\":[]}{}"));
    assert_eq!(Err(RootNotSingular), json_parse("[false],[]"));
}

#[test]
fn test_invalid_unicode_surrogate() {
    assert_eq!(Err(InvalidUnicodeSurrogate), json_parse("\"\\ud900\""));
    assert_eq!(Err(InvalidUnicodeSurrogate), json_parse("\"\\ud900\\t\""));
    assert_eq!(
        Err(InvalidUnicodeSurrogate),
        json_parse("\"\\ud900\\u123\"")
    );
}

#[test]
fn test_invalid_unicode_hex() {
    assert_eq!(Err(InvalidUnicodeHex), json_parse("\"\\u12\""));
}

#[test]
fn test_invalid_string_escape() {
    assert_eq!(Err(InvalidStringEscape), json_parse("\"\\c\""));
}

#[test]
fn test_miss_quote() {
    assert_eq!(Err(MissQuotationMark), json_parse("\"abc"));
}

#[test]
fn test_invlid_string_char() {
    assert_eq!(
        Err(InvalidStringChar),
        json_parse(format!("\"\u{12}\"").as_str())
    );
}

#[test]
fn test_miss_comma_or_square_backet() {
    assert_eq!(Err(ArrayMissCommaOrSquareBacket), json_parse("[\"12\":12]"));
    assert_eq!(Err(ArrayMissCommaOrSquareBacket), json_parse("[\"12\", 12"));
}

#[test]
fn test_miss_comma_or_curly_bracket() {
    assert_eq!(
        Err(ObjectMissCommaOrCurlyBacket),
        json_parse("{\"12\":12:13}")
    );
    assert_eq!(
        Err(ObjectMissCommaOrCurlyBacket),
        json_parse("{\"abc\": 13")
    );
}

#[test]
fn test_miss_key() {
    assert_eq!(Err(ObjectMissKey), json_parse("{12:!2}"));
}

#[test]
fn test_miss_colon() {
    assert_eq!(Err(ObjectMissColon), json_parse("{\"12\", 12}"));
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
