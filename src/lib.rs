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
