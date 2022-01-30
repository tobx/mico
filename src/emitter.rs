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
    use crate::List;

    use super::*;

    fn encode(mappings: &[Mapping], indent_size: u8) -> io::Result<String> {
        let mut buffer = Vec::new();
        let mut emitter = Emitter::new(&mut buffer, indent_size);
        emitter.emit(mappings)?;
        Ok(std::str::from_utf8(buffer.as_slice()).unwrap().to_string())
    }

    fn list(values: &[&str]) -> List {
        values.iter().copied().map(Into::into).collect()
    }

    #[test]
    fn test_string() {
        let encoded = encode(&[Mapping::new("key", "value")], 0);
        assert_eq!(encoded.unwrap(), "key: value\n");
    }

    #[test]
    fn test_list() {
        let encoded = encode(&[Mapping::new("key", list(&["value"]))], 0);
        assert_eq!(encoded.unwrap(), "key\n- value\n");
    }

    #[test]
    fn test_list_indent() {
        let encoded = encode(&[Mapping::new("key", list(&["value1", "value2"]))], 2);
        assert_eq!(encoded.unwrap(), "key\n  - value1\n  - value2\n");
    }

    #[test]
    fn test_empty() {
        let encoded = encode(&[], 1);
        assert_eq!(encoded.unwrap(), "");
    }

    #[test]
    fn test_mixed() {
        let encoded = encode(
            &[
                Mapping::new("key1", "value1"),
                Mapping::new("key2", list(&["value2"])),
                Mapping::new("key3", "value3"),
                Mapping::new("key4", list(&["value4"])),
            ],
            2,
        );
        assert_eq!(
            encoded.unwrap(),
            concat!(
                "key1: value1\n",
                "key2\n  - value2\n",
                "key3: value3\n",
                "key4\n  - value4\n",
            )
        );
    }
}
