# rust_json

[![Latest Version](https://img.shields.io/crates/v/rust_json.svg)](https://crates.io/crates/rust_json)

JSON serializer and deserializer written for learning rust.

## Feature

### Parse json from string

```rust
use rust_json::json_parse;

fn example() {
    let j = json_parse(r#"
    {
        "S": [
            1, 
            2.3, 
            {
                "4": {
                    "5": {
                        "6": [
                            null,
                            true, 
                            false
                            ]
                        }
                    }
            }
        ]
    }"#);

    println!("{}", j["S"]);
}
```

### Construct JsonElem with json literal

```rust
use rust_json::json;

fn example() {
    let j = json!(
        {
            "S": [
                1.2, 
                "2.3", 
                {
                    "4": {
                        "5": {
                            "6": [
                                null,
                                true, 
                                false
                                ]
                            }
                        }
                }
            ]
        }
    );

    println!("{}", j["S"]);
}
```

### Construct JsonElem with js object literal style

```rust
use rust_json::js_object;

fn proc(n: i32) -> i32 {
    n * n + n / 2
}

fn main() {
    let a = true;
    let n = 32;
    let j = js_object!([
        {
            a // 属性的简洁表示 Property Shorthand
        },
        {
            // 使用表达式作为值 Using expression
            proc_n: if n % 2 == 0 { proc(n) + 1 } else { 0 }, 
            [n * 12]: n * 12 // 属性名表达式 Computed Property Names
        }
    ]);
    println!("{}", j);
}
```

### ToJson and FromJson traits

Impl the `ToJson` and `FromJson` to serialize and deserialize custom struct. Or you can use [rust_json_derive](https://crates.io/crates/rust_json_derive) to derive the traits.

### Stringify with space

```rust
use rust_json::json;

fn example() {
    let j = json!({"a": 12});
    println!("{:#}", j);
    // { 
    //     "a": 12
    // }
    println!("{:3}", j);
    // { 
    //    "a": 12
    // }
}
```