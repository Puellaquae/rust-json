use crate::JsonElem;
use std::collections::HashMap;

/// Get the data in `JsonElem` and box into `Self`
pub trait FromJson: Sized {
    fn from_json(json: JsonElem) -> Option<Self>;
}

impl FromJson for f64 {
    fn from_json(json: JsonElem) -> Option<f64> {
        if let JsonElem::Number(n) = json {
            Some(n)
        } else {
            None
        }
    }
}

impl FromJson for i32 {
    fn from_json(json: JsonElem) -> Option<i32> {
        if let JsonElem::Number(n) = json {
            Some(n as i32)
        } else {
            None
        }
    }
}

impl FromJson for bool {
    fn from_json(json: JsonElem) -> Option<bool> {
        if let JsonElem::Bool(b) = json {
            Some(b)
        } else {
            None
        }
    }
}

impl FromJson for String {
    fn from_json(json: JsonElem) -> Option<String> {
        if let JsonElem::Str(s) = json {
            Some(s)
        } else {
            None
        }
    }
}

impl FromJson for Vec<JsonElem> {
    fn from_json(json: JsonElem) -> Option<Vec<JsonElem>> {
        if let JsonElem::Array(a) = json {
            Some(a)
        } else {
            None
        }
    }
}

impl<T: FromJson> FromJson for Vec<T> {
    fn from_json(json: JsonElem) -> Option<Vec<T>> {
        if let JsonElem::Array(a) = json {
            Some(a.into_iter().map(|v| v.get().unwrap()).collect())
        } else {
            None
        }
    }
}

impl FromJson for HashMap<String, JsonElem> {
    fn from_json(json: JsonElem) -> Option<HashMap<String, JsonElem>> {
        if let JsonElem::Object(o) = json {
            Some(o)
        } else {
            None
        }
    }
}
