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
                    },
                    {
                        c: {
                            "a.0": a.0, 
                            "a.1": a.1
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
