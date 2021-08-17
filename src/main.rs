use rust_json::{js_object};

fn proc(n: i32) -> i32 {
    n * n + n / 2
}

fn main() {
    let a = true;
    let n = 32;
    let j = js_object!([
        {
            a 
        },
        {
            proc_n: if n % 2 == 0 { proc(n) + 1 } else { 0 },
            [n * 12]: n * 12
        }
    ]);
    println!("{:#?}", j);
}
