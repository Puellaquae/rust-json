use crate::JsonElem;
use std::collections::HashMap;

pub trait ToJson {
    fn to_json(&self) -> JsonElem;
}

impl ToJson for JsonElem {
    fn to_json(&self) -> JsonElem {
        self.clone()
    }
}

impl ToJson for f64 {
    fn to_json(&self) -> JsonElem {
        JsonElem::Number(*self)
    }
}

impl ToJson for i32 {
    fn to_json(&self) -> JsonElem {
        JsonElem::Number(*self as f64)
    }
}

impl ToJson for i64 {
    fn to_json(&self) -> JsonElem {
        JsonElem::Number(*self as f64)
    }
}

impl ToJson for bool {
    fn to_json(&self) -> JsonElem {
        JsonElem::Bool(*self)
    }
}

impl ToJson for &str {
    fn to_json(&self) -> JsonElem {
        JsonElem::Str(String::from(*self))
    }
}

impl ToJson for String {
    fn to_json(&self) -> JsonElem {
        JsonElem::Str(self.clone())
    }
}

impl<T: ToJson> ToJson for &[T] {
    fn to_json(&self) -> JsonElem {
        JsonElem::Array(self.iter().map(|v| v.to_json()).collect())
    }
}

impl<T: ToJson> ToJson for [T] {
    fn to_json(&self) -> JsonElem {
        JsonElem::Array(self.iter().map(|v| v.to_json()).collect())
    }
}

impl<T: ToJson> ToJson for Vec<T> {
    fn to_json(&self) -> JsonElem {
        JsonElem::Array(self.iter().map(|v| v.to_json()).collect())
    }
}

impl<T: ToJson> ToJson for HashMap<String, T> {
    fn to_json(&self) -> JsonElem {
        let o = self
            .iter()
            .map(|(k, v)| (k.clone(), v.to_json()))
            .collect::<HashMap<String, JsonElem>>();
        JsonElem::Object(o)
    }
}

impl<T: ToJson> ToJson for Option<T> {
    fn to_json(&self) -> JsonElem {
        if let Some(v) = self {
            v.to_json()
        } else {
            JsonElem::Null
        }
    }
}
