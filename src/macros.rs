#[macro_export]
macro_rules! json {
    (for_array [ $($val:expr)* ]) => {
        $crate::JsonElem::Array(vec!($($val),*))
    };

    (for_array [ $($val:expr)* ] $elem:tt $($rest:tt)*) => {
        json!(for_array [$($val)* json!($elem)] $($rest)*)
    };

    (for_object $obj:ident) => {
    };

    (for_object $obj:ident $key:tt : $val:tt) => {
        $obj.insert(String::from($key), json!($val));
    };

    (for_object $obj:ident $key:tt : $val:tt, $($rest:tt)*) => {
        $obj.insert(String::from($key), json!($val));
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
        $crate::ToJson::to_json(&$val)
    };
}

#[macro_export]
macro_rules! js_object {
    (for_array [$($elem:expr)*]) => {
        vec![$($elem),*]
    };

    (for_array [$($elem:expr)*] $val:tt , $($rest:tt)*) => {
        js_object!(for_array [$($elem)* js_object!($val)] $($rest)*)
    };

    (for_array [$($elem:expr)*] $val:tt) => {
        js_object!(for_array [$($elem)* js_object!($val)])
    };

    (for_object $obj:ident $key:tt : $val:tt , $($rest:tt)*) => {
        js_object!(obj_insert $obj $key, js_object!($val));
        js_object!(for_object $obj $($rest)*)
    };

    (for_object $obj:ident $key:tt : $val:tt) => {
        js_object!(obj_insert $obj $key, js_object!($val))
    };

    (for_object $obj:ident $key:tt : $val:expr , $($rest:tt)*) => {
        js_object!(obj_insert $obj $key, js_object!($val));
        js_object!(for_object $obj $($rest)*)
    };

    (for_object $obj:ident $key:tt : $val:expr) => {
        js_object!(obj_insert $obj $key, js_object!($val))
    };

    (for_object $obj:ident $key:ident , $($rest:tt)*) => {
        js_object!(obj_insert $obj $key, js_object!($key));
        js_object!(for_object $obj $($rest)*)
    };

    (for_object $obj:ident $key:ident) => {
        js_object!(obj_insert $obj $key, js_object!($key))
    };

    (for_object $obj:ident) => {
    };

    (obj_insert $obj:ident [ $key:expr ], $val: expr) => {
        {
            let key = ToString::to_string(&$key);
            $obj.insert(String::from(key), $val);
        }
    };

    (obj_insert $obj:ident $key:tt, $val: expr) => {
        {
            let key = stringify!($key);
            let key = if key.starts_with("\"") && key.ends_with("\"") {
                key.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap()
            } else {
                key
            };
            $obj.insert(String::from(key), $val);
        }
    };

    (null) => {
        $crate::JsonElem::Null
    };

    ([ $($val:tt)* ]) => {
        $crate::JsonElem::Array(js_object!(for_array [] $($val)*))
    };

    ({ $($val:tt)* }) => {
        {
            let mut hm = std::collections::HashMap::<String, $crate::JsonElem>::new();
            js_object!(for_object hm $($val)*);
            $crate::JsonElem::Object(hm)
        }
    };

    ($val: expr) => {
        $crate::ToJson::to_json(&$val)
    };
}
