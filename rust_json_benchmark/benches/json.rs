use criterion::{criterion_group, criterion_main, Criterion};
use rust_json::json_parse;
use serde_json::from_str;

fn parse(c: &mut Criterion) {
    c.bench_function("parse-number", |b| {
        b.iter(|| json_parse("-4.9406564584124654e-324"))
    });
    c.bench_function("parse-number-serde-json", |b| {
        b.iter(|| {
            let _: serde_json::Value = from_str("-4.9406564584124654e-324").unwrap();
        })
    });
    c.bench_function("parse-array", |b| {
        b.iter(|| json_parse("[ null , false , true , 123 , \"abc\" ]"))
    });
    c.bench_function("parse-array-serde-json", |b| {
        b.iter(|| {
            let _: serde_json::Value = from_str("[ null , false , true , 123 , \"abc\" ]").unwrap();
        })
    });
}

criterion_group!(benches, parse);
criterion_main!(benches);
