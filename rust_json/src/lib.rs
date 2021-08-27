mod macros;
mod parser;
mod serialize;
mod traits;

pub use parser::json_parse;
pub use traits::FromJson;
pub use traits::ToJson;

use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonElem {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonElem>),
    Object(std::collections::HashMap<String, JsonElem>),
}

impl JsonElem {
    pub fn get<T: FromJson>(self) -> Option<T> {
        T::from_json(self)
    }
}

impl FromStr for JsonElem {
    type Err = JsonParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        json_parse(s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
