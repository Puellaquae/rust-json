use crate::JsonElem;
use std::fmt::{Display, Formatter, Result};

enum CharEscape {
    Quote,
    ReverseSolidus,
    Backspace,
    FormFeed,
    LineFeed,
    CarriageReturn,
    Tab,
    AsciiControl(u8),
    None(u8),
}

fn string_escape(str: &String) -> String {
    use CharEscape::*;
    let mut buf = Vec::with_capacity(str.capacity());
    for byte in str.bytes() {
        let escape = match byte {
            0x08 => Backspace,
            0x09 => Tab,
            0x0a => LineFeed,
            0x0c => FormFeed,
            0x0d => CarriageReturn,
            0x22 => Quote,
            0x5c => ReverseSolidus,
            b @ 0x00..=0x1f => AsciiControl(b),
            b => None(b),
        };

        match escape {
            Quote => buf.extend_from_slice(b"\\\""),
            ReverseSolidus => buf.extend_from_slice(b"\\\\"),
            Backspace => buf.extend_from_slice(b"\\b"),
            FormFeed => buf.extend_from_slice(b"\\f"),
            LineFeed => buf.extend_from_slice(b"\\n"),
            CarriageReturn => buf.extend_from_slice(b"\\r"),
            Tab => buf.extend_from_slice(b"\\t"),
            AsciiControl(b) => {
                static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
                buf.extend_from_slice(&[
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    HEX_DIGITS[(b >> 4) as usize],
                    HEX_DIGITS[(b & 0xF) as usize],
                ]);
            },
            None(b) => buf.push(b)
        };
    }
    String::from_utf8(buf).unwrap()
}

impl Display for JsonElem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::Str(s) => write!(f, "\"{}\"", string_escape(s)),
            Self::Array(a) => write!(
                f,
                "[{}]",
                a.into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Object(o) => write!(
                f,
                "{{{}}}",
                o.into_iter()
                    .map(|(k, v)| format!("\"{}\":{}", string_escape(k), v))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}
