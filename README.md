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
use rust_json::{js_object};

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
            // 使用表达式作为值
            proc_n: if n % 2 == 0 { proc(n) + 1 } else { 0 }, 
            [n * 12]: n * 12 // 属性名表达式 Computed Property Names
        }
    ]);
    println!("{:#?}", j);
}
```