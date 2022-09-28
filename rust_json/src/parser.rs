use crate::{JsonElem, JsonParseErr};
use std::collections::HashMap;
use std::str::Chars;
use std::str::FromStr;

impl FromStr for JsonElem {
    type Err = JsonParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        json_parse(s)
    }
}

pub fn json_parse(json: &str) -> Result<JsonElem, JsonParseErr> {
    let mut json_trim = json.trim();
    let res = json_parse_value(&mut json_trim);
    if res.is_ok() && !json_trim.is_empty() {
        Err(JsonParseErr::RootNotSingular)
    } else {
        res
    }
}

fn json_parse_value(json: &mut &str) -> Result<JsonElem, JsonParseErr> {
    match json.chars().next() {
        Some('n') => json_parse_literal(json, "null", JsonElem::Null),
        Some('t') => json_parse_literal(json, "true", JsonElem::Bool(true)),
        Some('f') => json_parse_literal(json, "false", JsonElem::Bool(false)),
        Some('"') => json_parse_string(json),
        Some('[') => json_parse_array(json),
        Some('{') => json_parse_object(json),
        None => Err(JsonParseErr::ExpectValue),
        _ => json_parse_number(json),
    }
}

fn json_parse_literal(
    json: &mut &str,
    literal: &str,
    res: JsonElem,
) -> Result<JsonElem, JsonParseErr> {
    if let Some(new_json) = json.strip_prefix(literal) {
        *json = new_json;
        Ok(res)
    } else {
        Err(JsonParseErr::InvalidValue)
    }
}

fn json_parse_number(json: &mut &str) -> Result<JsonElem, JsonParseErr> {
    let mut iter = json.chars().peekable();
    if iter.peek() == Some(&'-') {
        iter.next();
    }

    if iter.peek() == Some(&'0') {
        iter.next();
    } else if is_digit_1_to_9(iter.peek()) {
        iter.next();
        while is_digit(iter.peek()) {
            iter.next();
        }
    } else {
        return Err(JsonParseErr::InvalidValue);
    }

    if iter.peek() == Some(&'.') {
        iter.next();
        if !is_digit(iter.peek()) {
            return Err(JsonParseErr::InvalidValue);
        }
        iter.next();
        while is_digit(iter.peek()) {
            iter.next();
        }
    }

    if iter.peek() == Some(&'e') || iter.peek() == Some(&'E') {
        iter.next();
        if iter.peek() == Some(&'+') || iter.peek() == Some(&'-') {
            iter.next();
        }
        if !is_digit(iter.peek()) {
            return Err(JsonParseErr::InvalidValue);
        }
        iter.next();
        while is_digit(iter.peek()) {
            iter.next();
        }
    }
    let len = json.chars().count() - iter.count();
    let end = json.char_indices().map(|(i, _)| i).nth(len - 1).unwrap();
    let number = &json[0..=end];
    *json = &json[(end + 1)..];
    Ok(JsonElem::Number(String::from(number).parse().unwrap()))
}

fn is_digit_1_to_9(ch: Option<&char>) -> bool {
    if let Some(ch) = ch {
        '1' <= *ch && *ch <= '9'
    } else {
        false
    }
}

fn is_digit(ch: Option<&char>) -> bool {
    if let Some(ch) = ch {
        '0' <= *ch && *ch <= '9'
    } else {
        false
    }
}

fn json_parse_string(json: &mut &str) -> Result<JsonElem, JsonParseErr> {
    let mut str_buf: Vec<char> = Vec::new();
    let mut chars = json.chars();
    chars.next();
    loop {
        match chars.next() {
            Some('"') => break,
            Some('\\') => match chars.next() {
                Some('"') => str_buf.push('"'),
                Some('\\') => str_buf.push('\\'),
                Some('/') => str_buf.push('/'),
                Some('b') => str_buf.push('\x08'),
                Some('f') => str_buf.push('\x0c'),
                Some('n') => str_buf.push('\n'),
                Some('r') => str_buf.push('\r'),
                Some('t') => str_buf.push('\t'),
                Some('u') => {
                    if let Some(res) = try_get_hex4(&mut chars) {
                        if (0xd800..=0xdbff).contains(&res) {
                            if chars.next() != Some('\\') {
                                return Err(JsonParseErr::InvalidUnicodeSurrogate);
                            }
                            if chars.next() != Some('u') {
                                return Err(JsonParseErr::InvalidUnicodeSurrogate);
                            }
                            if let Some(res2) = try_get_hex4(&mut chars) {
                                let u = (((res - 0xD800) << 10) | (res2 - 0xDC00)) + 0x10000;
                                if let Some(ch) = char::from_u32(u) {
                                    str_buf.push(ch)
                                } else {
                                    return Err(JsonParseErr::InvalidUnicodeSurrogate);
                                }
                            } else {
                                return Err(JsonParseErr::InvalidUnicodeSurrogate);
                            }
                        } else if let Some(ch) = char::from_u32(res) {
                            str_buf.push(ch)
                        } else {
                            return Err(JsonParseErr::InvalidUnicodeSurrogate);
                        }
                    } else {
                        return Err(JsonParseErr::InvalidUnicodeHex);
                    }
                }
                _ => return Err(JsonParseErr::InvalidStringEscape),
            },
            None => return Err(JsonParseErr::MissQuotationMark),
            Some(c) => {
                if (c as u32) < 0x20 {
                    return Err(JsonParseErr::InvalidStringChar);
                } else {
                    str_buf.push(c)
                }
            }
        }
    }
    *json = chars.as_str();
    Ok(JsonElem::Str(str_buf.into_iter().collect()))
}

fn hex_to_u32(h: char) -> u32 {
    match h {
        '0'..='9' => h as u32 - '0' as u32,
        'a'..='f' => h as u32 - 'a' as u32 + 10,
        'A'..='F' => h as u32 - 'A' as u32 + 10,
        _ => panic!("input is not a hex digit"),
    }
}

fn try_get_hex4(chars: &mut Chars) -> Option<u32> {
    let c1 = chars.next()?;
    let c2 = chars.next()?;
    let c3 = chars.next()?;
    let c4 = chars.next()?;
    if c1.is_ascii_hexdigit()
        && c2.is_ascii_hexdigit()
        && c3.is_ascii_hexdigit()
        && c4.is_ascii_hexdigit()
    {
        Some(
            (hex_to_u32(c1) << 12)
                + (hex_to_u32(c2) << 8)
                + (hex_to_u32(c3) << 4)
                + (hex_to_u32(c4)),
        )
    } else {
        None
    }
}

fn json_parse_array(json: &mut &str) -> Result<JsonElem, JsonParseErr> {
    let mut chars = json.chars();
    assert_eq!(Some('['), chars.next());
    let mut arr: Vec<JsonElem> = Vec::new();
    *json = chars.as_str().trim();
    if let Some(new_json) = json.strip_prefix(']') {
        *json = new_json;
        return Ok(JsonElem::Array(arr));
    }
    loop {
        let res = json_parse_value(json);
        if let Ok(elem) = res {
            arr.push(elem);
        } else {
            return res;
        }
        *json = json.trim();
        if let Some(new_json) = json.strip_prefix(',') {
            *json = new_json.trim();
        } else if let Some(new_json) = json.strip_prefix(']') {
            *json = new_json;
            return Ok(JsonElem::Array(arr));
        } else {
            return Err(JsonParseErr::ArrayMissCommaOrSquareBacket);
        }
    }
}

fn json_parse_object(json: &mut &str) -> Result<JsonElem, JsonParseErr> {
    let mut chars = json.chars();
    assert_eq!(Some('{'), chars.next());
    let mut obj = HashMap::new();
    *json = chars.as_str().trim();
    if let Some(new_json) = json.strip_prefix('}') {
        *json = new_json;
        return Ok(JsonElem::Object(obj));
    }
    loop {
        let res = json_parse_member(json);
        match res {
            Ok((key, elem)) => obj.insert(key, elem),
            Err(e) => return Err(e),
        };
        *json = json.trim();
        if let Some(new_json) = json.strip_prefix(',') {
            *json = new_json.trim();
        } else if let Some(new_json) = json.strip_prefix('}') {
            *json = new_json;
            return Ok(JsonElem::Object(obj));
        } else {
            return Err(JsonParseErr::ObjectMissCommaOrCurlyBacket);
        }
    }
}

fn json_parse_member(json: &mut &str) -> Result<(String, JsonElem), JsonParseErr> {
    if !json.starts_with('"') {
        return Err(JsonParseErr::ObjectMissKey);
    }
    let key;
    match json_parse_string(json) {
        Ok(JsonElem::Str(k)) => key = k,
        Err(e) => return Err(e),
        _ => panic!(
            "json_parse_string shouldn't return unexpected values other than JsonElem::Str and Err"
        ),
    };
    *json = json.trim();
    match json.strip_prefix(':') {
        Some(new_json) => *json = new_json.trim(),
        None => return Err(JsonParseErr::ObjectMissColon),
    };
    match json_parse_value(json) {
        Ok(elem) => Ok((key, elem)),
        Err(e) => Err(e),
    }
}
