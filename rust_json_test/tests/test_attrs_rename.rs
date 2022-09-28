use rust_json::{JsonElem, ToJson};
use rust_json_derive::{FromJson, ToJson};

#[derive(Debug, PartialEq, ToJson, FromJson)]
struct S {
    #[rename = "s^n"]
    n: f64,
}

#[derive(Debug, PartialEq, ToJson, FromJson)]
enum E {
    #[rename = "e$u"]
    Unit,
    #[rename = "e.o"]
    One(i32),
    #[rename = "t-e"]
    Two(i32, i32),
    #[rename = "复合"]
    Cmpx {
        #[rename = "z"]
        a: i32,
    },
}

#[test]
fn test_rename_struct() {
    let s = S { n: 12.3 };
    assert_eq!(r#"{"s^n":12.3}"#, s.to_json().to_string());
    assert_eq!(s, s.to_json().to_string().parse::<JsonElem>().unwrap().get().unwrap());
}

#[test]
fn test_rename_enum() {
    let u = E::Unit;
    let o = E::One(1);
    let t = E::Two(1, 2);
    let c = E::Cmpx { a: 1 };
    assert_eq!(r#""e$u""#, u.to_json().to_string());
    assert_eq!(r#"{"e.o":1}"#, o.to_json().to_string());
    assert_eq!(r#"{"t-e":[1,2]}"#, t.to_json().to_string());
    assert_eq!(r#"{"复合":{"z":1}}"#, c.to_json().to_string());
    assert_eq!(
        u,
        u.to_json()
            .to_string()
            .parse::<JsonElem>()
            .unwrap()
            .get()
            .unwrap()
    );
    assert_eq!(
        o,
        o.to_json()
            .to_string()
            .parse::<JsonElem>()
            .unwrap()
            .get()
            .unwrap()
    );
    assert_eq!(
        t,
        t.to_json()
            .to_string()
            .parse::<JsonElem>()
            .unwrap()
            .get()
            .unwrap()
    );
    assert_eq!(
        c,
        c.to_json()
            .to_string()
            .parse::<JsonElem>()
            .unwrap()
            .get()
            .unwrap()
    );
}
