use rust_json::{json_parse, JsonElem::*};
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
