use crate::JsonElem;

pub trait ToJson {
    fn to_json(self) -> JsonElem;
}

impl ToJson for f64 {
    fn to_json(self) -> JsonElem {
        JsonElem::Number(self)
    }
}

impl ToJson for i32 {
    fn to_json(self) -> JsonElem {
        JsonElem::Number(self as f64)
    }
}

impl ToJson for i64 {
    fn to_json(self) -> JsonElem {
        JsonElem::Number(self as f64)
    }
}

impl ToJson for bool {
    fn to_json(self) -> JsonElem {
        JsonElem::Bool(self)
    }
}

impl ToJson for &str {
    fn to_json(self) -> JsonElem {
        JsonElem::Str(String::from(self))
    }
}