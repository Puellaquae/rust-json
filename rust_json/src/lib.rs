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

    /// Represents a JSON null value.
    /// 
    /// ```
    /// # use rust_json::json;
    /// let j = json!(null);
    /// ```
    Null,

    /// Represents a JSON boolean.
    /// 
    /// ```
    /// # use rust_json::json;
    /// let j = json!(true);
    /// ```
    Bool(bool),

    /// Represents a JSON number, storing in `f64`.
    /// 
    /// ```
    /// # use rust_json::json;
    /// let j = json!(3.14);
    /// ```
    Number(f64),

    /// Represents a JSON string.
    /// 
    /// ```
    /// # use rust_json::json;
    /// let j = json!("abc");
    /// ```
    Str(String),

    /// Represents a JSON array.
    /// 
    /// ```
    /// # use rust_json::json;
    /// let j = json!(["an", "array"]);
    /// ```
    Array(Vec<JsonElem>),

    /// Represents a JSON object.
    /// 
    /// ```
    /// # use rust_json::json;
    /// let j = json!({"an": "object"});
    /// ```
    Object(std::collections::HashMap<String, JsonElem>),
}

impl JsonElem {

    /// Get the value stored in `JsonElem` and desrialize into `T`.
    /// 
    /// ```
    /// # use rust_json::json;
    /// assert_eq!(Some(12.3), json!(12.3).get());
    /// assert_eq!(None, json!("abc").get::<bool>());
    /// ```
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
