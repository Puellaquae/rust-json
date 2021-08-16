# rust-json

学习 rust 所写的 JSON 解析器，参考了 miloyip 的 [json-tutorial](https://github.com/miloyip/json-tutorial)。

## 功能

### 解析 json

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