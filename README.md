# rust-json

学习 rust 所写的 JSON 解析器。

解析的编写参考了 miloyip 的 [json-tutorial](https://github.com/miloyip/json-tutorial)。

Rust 宏的编写参考了 [Serde JSON](https://github.com/serde-rs/json)。

## 功能

### 从字符串解析 json

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

    println!("{:?}", j);
}
```

### 以 json 的风格构造 JsonElem

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

    println!("{:?}", j);
}
```

### 以 js 的风格构造 JsonElem

```rust
use rust_json::js_object;

struct A(i32, bool);

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = A(12, true);
    println!(
        "{:#?}",
        js_object!({
            a: {
                b: [
                    if a.1 {
                        let n = 123;
                        add(n, a.0) 
                    } else {
                        321 
                    }, // 可以接受一个语句作为值
                    {
                        c: {
                            "a.0": a.0,
                            "a.1": a.1 // 但都不接受末尾的逗号
                        }
                    },
                    [
                        null,
                        [
                            true, 
                            [
                                null
                            ]
                        ]
                    ]
                ]
            }
        })
    );
}
```