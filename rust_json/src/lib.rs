mod macros;
mod parser;
mod serialize;
mod traits;

pub use parser::json_parse;
pub use traits::FromJson;
pub use traits::ToJson;

use std::ops;

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

    pub fn is_null(&self) -> bool {
        matches!(self, JsonElem::Null)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, JsonElem::Bool(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, JsonElem::Str(_))
    }
    pub fn is_array(&self) -> bool {
        matches!(self, JsonElem::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, JsonElem::Object(_))
    }
}

impl ops::Index<usize> for JsonElem {
    type Output = JsonElem;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            JsonElem::Array(a) if idx < a.len() => &a[idx],
            _ => &JsonElem::Null,
        }
    }
}

impl ops::Index<&str> for JsonElem {
    type Output = JsonElem;
    fn index(&self, idx: &str) -> &Self::Output {
        match self {
            JsonElem::Object(o) if o.contains_key(idx.into()) => &o[idx.into()],
            _ => &JsonElem::Null,
        }
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
