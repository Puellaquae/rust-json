#[macro_export]
macro_rules! json {
    (for_array [ $($val: expr)* ]) => {
        $crate::JsonElem::Array(vec!($($val),*))
    };

    (for_array [ $($val: expr )* ] $elem:tt $($rest:tt)*) => {
        json!(for_array [$($val)* json!($elem)] $($rest)*)
    };

    (for_object $obj:ident) => {
    };

    (for_object $obj:ident $key:tt : $val:tt) => {
        json!(for_object $obj $key, json!($val), )
    };

    (for_object $obj:ident $key:tt : $val:tt, $($rest: tt)*) => {
        json!(for_object $obj $key, json!($val), $($rest)*)
    };

    (for_object $obj:ident $key:expr, $val:expr, $($rest: tt)*) => {
        $obj.insert(String::from($key), $val);
        json!(for_object $obj $($rest)*)
    };

    (null) => {
        $crate::JsonElem::Null
    };

    ([ $($val:tt),* ]) => {
        json!(for_array [] $($val)*)
    };

    ({ $($val:tt)* }) => {
        {
            let mut hm = std::collections::HashMap::<String, $crate::JsonElem>::new();
            json!(for_object hm $($val)*);
            $crate::JsonElem::Object(hm)
        }
    };

    ($val: expr) => {
        $crate::ToJson::to_json($val)
    };
}