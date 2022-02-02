//! # mico
//!
//! This library implements a parser and emitter for `mico` (minimalistic config file format).
//!
//! Format example:
//!
//! ```plaintext
//! Name: mico
//! Description: minimalistic config file format
//!
//! Benefits
//!  - easy to read and write for everyone
//!  - ludicrously simple parsing logic (roughly 30 lines of code)
//! ```
//!
//! ## Parsing and emitting mico
//!
//! There are two convenience functions to parse and emit:
//!
//! ```rust
//! use mico::Mapping;
//!
//! // parse string
//! let mappings = mico::from_str("foo: bar");
//! assert_eq!(mappings[0].key, "foo");
//! assert_eq!(mappings[0].value, "bar".into());
//!
//! // emit mappings
//! let mappings = [Mapping::new("foo", "bar")];
//! assert_eq!(mico::to_string(&mappings, 0), "foo: bar\n");
//! ```
//!
//! ## Notes
//!
//! `mico` is meant for people to write simple config files very fast. There is no
//! indentation, escaping or quoting to worry about, so any text line can simply be
//! pasted to a `mico` file without further editing.
//!
//! There are only two types:
//!
//! 1. A mapping from key to string:
//!
//!    ```plaintext
//!    key: value
//!    ```
//!
//! 2. A mapping from key to a list of strings
//!
//!    ```plaintext
//!    key
//!     - value 1
//!     - value 2
//!    ```
//!
//! Here is a mico example:
//!
//! ```plaintext
//! foo: bar
//!   indentation is possible: but does not matter
//! white space  :  will be trimmed
//! this is a key:this: is:a :value
//! empty string:
//!
//! empty lines: will be ignored
//!
//! list ...
//!  - keys must not include a colon
//!  - items start with '-'
//!
//! this is an empty list, because this line does not include a colon
//!
//! this is no list because of the colon at the end of this line:
//!  - this is an empty list
//! ```
//!
//! Here is the corresponding JSON example:
//!
//! ```json
//! [
//!   { "foo": "bar" },
//!   { "indentation is possible": "but does not matter" },
//!   { "white space": "will be trimmed" },
//!   { "this is a key": "this: is:a :value" },
//!   { "empty string": "" },
//!   { "empty lines": "will be ignored" },
//!   { "list ...": ["keys must not include a colon", "items start with '-'"] },
//!   { "This is an empty list, because this line does not include a colon": [] },
//!   { "This is no list because of the colon at the end of this line:": "" },
//!   { "- this is an empty list": [] }
//! ]
//! ```
mod emitter;
mod parser;

use std::io::Cursor;

pub use emitter::Emitter;
pub use parser::Parser;

pub type List = Vec<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    List(List),
    String(String),
}

impl From<List> for Value {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

macro_rules! impl_into_string_value {
    ( $( $T:ty ),* ) => {
        $(
            impl From<$T> for Value {
                fn from(value: $T) -> Value {
                    Value::String(value.to_string())
                }
            }
        )*
    };
}

impl_into_string_value![
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64, bool, char, &str
];

pub struct Mapping {
    pub key: String,
    pub value: Value,
}

impl Mapping {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

pub fn from_str(s: &str) -> Vec<Mapping> {
    Parser::default().parse(Cursor::new(s)).unwrap()
}

pub fn to_string(mappings: &[Mapping], indent_size: u8) -> String {
    let mut buffer = Vec::new();
    let mut emitter = Emitter::new(&mut buffer, indent_size);
    emitter.emit(mappings).unwrap();
    std::str::from_utf8(buffer.as_slice()).unwrap().to_string()
}
