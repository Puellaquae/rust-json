use rust_json::{js_object, JsonElem, ToJson};

fn proc(n: i32) -> i32 {
    n * n + n / 2
}

#[derive(ToJson)]
struct S {
    n: f64,
}

fn main() {
    let a = true;
    let n = 32;
    let s = S { n: 12.3 };
    println!("{}", s.to_json());
    let b: bool = JsonElem::Bool(false).get().unwrap();
    println!("{}", b);
    let j = js_object!([
        {
            a
        },
        {
            proc_n: if n % 2 == 0 { proc(n) + 1 } else { 0 },
            [n * 12]: n * 12
        }
    ]);
    println!("{}", j);
}
