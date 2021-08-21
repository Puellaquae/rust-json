mod macros;
mod parser;
mod serialize;
mod traits;

pub use parser::json_parse;
pub use traits::ToJson;

#[derive(Debug, PartialEq)]
pub enum JsonElem {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonElem>),
    Object(std::collections::HashMap<String, JsonElem>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum JsonParseErr {
    ExpectValue,
    InvalidValue,
    RootNotSingular,
    InvalidStringEscape,
    MissQuotationMark,
    InvalidStringChar,
    InvalidUnicodeHex,
    InvalidUnicodeSurrogate,
    ArrayMissCommaOrSquareBacket,
    ObjectMissCommaOrCurlyBacket,
    ObjectMissKey,
    ObjectMissColon,
}
