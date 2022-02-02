use std::io;

use crate::{Mapping, Value};

pub struct Emitter<W> {
    writer: W,
    indentation: String,
}

impl<W: io::Write> Emitter<W> {
    pub fn new(writer: W, indent_size: u8) -> Self {
        Self {
            writer,
            indentation: " ".repeat(indent_size.into()),
        }
    }

    pub fn emit(&mut self, mappings: &[Mapping]) -> io::Result<()> {
        for Mapping { key, value } in mappings {
            match value {
                Value::List(value) => self.emit_list(key, value)?,
                Value::String(value) => self.emit_string(key, value)?,
            }
        }
        Ok(())
    }

    fn emit_list(&mut self, key: &str, list: &[String]) -> io::Result<()> {
        writeln!(&mut self.writer, "{}", key)?;
        for item in list {
            writeln!(&mut self.writer, "{}- {}", self.indentation, item)?;
        }
        Ok(())
    }

    fn emit_string(&mut self, key: &str, value: &str) -> io::Result<()> {
        writeln!(&mut self.writer, "{}: {}", key, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    impl Into<Value> for &[&str] {
        fn into(self) -> Value {
            self.iter()
                .copied()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into()
        }
    }

    #[test]
    fn test_string() {
        let encoded = to_string(&[Mapping::new("key", "value")], 0);
        assert_eq!(encoded, "key: value\n");
    }

    #[test]
    fn test_list() {
        let encoded = to_string(&[Mapping::new("key", ["value"].as_ref())], 0);
        assert_eq!(encoded, "key\n- value\n");
    }

    #[test]
    fn test_list_indent() {
        let encoded = to_string(&[Mapping::new("key", ["value1", "value2"].as_ref())], 2);
        assert_eq!(encoded, "key\n  - value1\n  - value2\n");
    }

    #[test]
    fn test_empty() {
        let encoded = to_string(&[], 1);
        assert_eq!(encoded, "");
    }

    #[test]
    fn test_mixed() {
        let encoded = to_string(
            &[
                Mapping::new("key1", "value1"),
                Mapping::new("key2", ["value2"].as_ref()),
                Mapping::new("key3", "value3"),
                Mapping::new("key4", ["value4"].as_ref()),
            ],
            2,
        );
        assert_eq!(
            encoded,
            concat!(
                "key1: value1\n",
                "key2\n  - value2\n",
                "key3: value3\n",
                "key4\n  - value4\n",
            )
        );
    }
}
