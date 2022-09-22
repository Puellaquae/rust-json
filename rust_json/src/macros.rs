/// Construct `JsonElem` with JSON syntax
///
/// ```
/// # use rust_json::json;
/// let value = json!({
///     "code": 404,
///     "success": false,
///     "errors": [
///         {"code": 1234, "description": null}
///     ]
/// });
/// ```
#[macro_export]
macro_rules! json {
    (@for_array [ $($val:expr)* ]) => {
        $crate::JsonElem::Array(vec!($($val),*))
    };

    (@for_array [ $($val:expr)* ] $elem:tt $($rest:tt)*) => {
        $crate::json!(@for_array [$($val)* $crate::json!($elem)] $($rest)*)
    };

    (@for_object $obj:ident) => {
    };

    (@for_object $obj:ident $key:tt : $val:tt) => {
        $obj.insert(String::from($key), $crate::json!($val));
    };

    (@for_object $obj:ident $key:tt : $val:tt, $($rest:tt)*) => {
        $obj.insert(String::from($key), $crate::json!($val));
        $crate::json!(@for_object $obj $($rest)*)
    };

    (null) => {
        $crate::JsonElem::Null
    };

    ([ $($val:tt),* ]) => {
        $crate::json!(@for_array [] $($val)*)
    };

    ({ $($val:tt)* }) => {
        {
            let mut hm = std::collections::HashMap::<String, $crate::JsonElem>::new();
            $crate::json!(@for_object hm $($val)*);
            $crate::JsonElem::Object(hm)
        }
    };

    ($val: expr) => {
        $crate::ToJson::to_json(&$val)
    };
}

/// Construct `JsonElem` with JS syntax
///
/// ```
/// # use rust_json::js_object;
/// let code = 404;
/// let error_code = 1234;
/// let error_description: Option<String> = None;
///
/// let value = js_object!({
///     code, // Property Shorthand
///     success: code == 200, // Expressions
///     errors: [
///         {
///             [error_code]: error_description // Computed Property Names
///         }
///     ]
/// });
/// ```
///
/// Any type interpolated into an array element or a object value must implement `ToJson` trait, while any type interpolated into a object key must implement `ToString` trait.
#[macro_export]
macro_rules! js_object {
    (@for_array [$($elem:expr)*]) => {
        vec![$($elem),*]
    };

    (@for_array [$($elem:expr)*] $val:tt , $($rest:tt)*) => {
        $crate::js_object!(@for_array [$($elem)* $crate::js_object!($val)] $($rest)*)
    };

    (@for_array [$($elem:expr)*] $val:tt) => {
        $crate::js_object!(@for_array [$($elem)* $crate::js_object!($val)])
    };

    (@for_object $obj:ident $key:tt : $val:tt , $($rest:tt)*) => {
        $crate::js_object!(@obj_insert $obj $key, $crate::js_object!($val));
        $crate::js_object!(@for_object $obj $($rest)*)
    };

    (@for_object $obj:ident $key:tt : $val:tt) => {
        $crate::js_object!(@obj_insert $obj $key, $crate::js_object!($val))
    };

    (@for_object $obj:ident $key:tt : $val:expr , $($rest:tt)*) => {
        $crate::js_object!(@obj_insert $obj $key, $crate::js_object!($val));
        $crate::js_object!(@for_object $obj $($rest)*)
    };

    (@for_object $obj:ident $key:tt : $val:expr) => {
        $crate::js_object!(@obj_insert $obj $key, $crate::js_object!($val))
    };

    (@for_object $obj:ident $key:ident , $($rest:tt)*) => {
        $crate::js_object!(@obj_insert $obj $key, $crate::js_object!($key));
        $crate::js_object!(@for_object $obj $($rest)*)
    };

    (@for_object $obj:ident $key:ident) => {
        $crate::js_object!(@obj_insert $obj $key, $crate::js_object!($key))
    };

    (@for_object $obj:ident) => {
    };

    (@obj_insert $obj:ident [ $key:expr ], $val: expr) => {
        {
            let key = ToString::to_string(&$key);
            $obj.insert(String::from(key), $val);
        }
    };

    (@obj_insert $obj:ident $key:tt, $val: expr) => {
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
        $crate::JsonElem::Array($crate::js_object!(@for_array [] $($val)*))
    };

    ({ $($val:tt)* }) => {
        {
            let mut hm = std::collections::HashMap::<String, $crate::JsonElem>::new();
            $crate::js_object!(@for_object hm $($val)*);
            $crate::JsonElem::Object(hm)
        }
    };

    ($val: expr) => {
        $crate::ToJson::to_json(&$val)
    };
}
