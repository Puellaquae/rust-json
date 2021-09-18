use rust_json::{json_parse, JsonParseErr::*};

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
