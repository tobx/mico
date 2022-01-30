use std::io::{self, BufRead, BufReader};

use crate::{List, Mapping};

#[derive(Default)]
pub struct Parser {
    mappings: Vec<Mapping>,
    list: Option<(String, List)>,
}

impl Parser {
    pub fn parse<R: io::Read>(mut self, reader: R) -> io::Result<Vec<Mapping>> {
        for line in BufReader::new(reader).lines() {
            let line = line?;
            let line = line.trim();
            if !line.is_empty() {
                self.parse_line(line);
            }
        }
        if let Some((key, list)) = self.list {
            self.mappings.push(Mapping::new(key, list));
        }
        Ok(self.mappings)
    }

    fn parse_line(&mut self, line: &str) {
        if let Some((key, mut list)) = self.list.take() {
            if let Some(value) = line.strip_prefix("- ") {
                list.push(value.trim_start().into());
                self.list = Some((key, list));
                return;
            }
            self.mappings.push(Mapping::new(key, list));
        }
        if let Some((key, value)) = line.split_once(": ") {
            let key = key.trim_end();
            let value = value.trim_start();
            self.mappings.push(Mapping::new(key, value));
        } else {
            self.list = Some((line.into(), List::new()));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    fn assert_list(mapping: Mapping, key: &str, value: &[&str]) {
        let list: List = value.iter().copied().map(Into::into).collect();
        assert_eq!(mapping.key, key);
        assert_eq!(mapping.value, list.into());
    }

    fn assert_string(mapping: Mapping, key: &str, value: &str) {
        assert_eq!(mapping.key, key);
        assert_eq!(mapping.value, value.into());
    }

    fn parse(config: &str) -> Vec<Mapping> {
        Parser::default().parse(Cursor::new(config)).unwrap()
    }

    #[test]
    fn test_string() {
        let mut mappings = parse("key: value");
        assert_eq!(mappings.len(), 1);
        assert_string(mappings.remove(0), "key", "value");
    }

    #[test]
    fn test_string_with_whitespace() {
        let mut mappings = parse(" key  with  whitespace :  value  with  whitespace ");
        assert_eq!(mappings.len(), 1);
        assert_string(
            mappings.remove(0),
            "key  with  whitespace",
            "value  with  whitespace",
        );
    }

    #[test]
    fn test_list() {
        let mut mappings = parse("key\n- value1\n- value2");
        assert_eq!(mappings.len(), 1);
        assert_list(mappings.remove(0), "key", &["value1", "value2"]);
    }

    #[test]
    fn test_list_with_whitespace() {
        let mut mappings = parse(" key  with  whitespace \n -  value 1 \n -  value 2 ");
        assert_eq!(mappings.len(), 1);
        assert_list(
            mappings.remove(0),
            "key  with  whitespace",
            &["value 1", "value 2"],
        );
    }

    #[test]
    fn test_empty() {
        let mappings = parse(" ");
        assert_eq!(mappings.len(), 0);
    }

    #[test]
    fn test_empty_lists() {
        let mut mappings = parse("key1\nkey2");
        assert_eq!(mappings.len(), 2);
        assert_list(mappings.remove(0), "key1", &[]);
        assert_list(mappings.remove(0), "key2", &[]);
    }

    #[test]
    fn test_mixed() {
        let mut mappings = parse(concat!(
            "key1: value1\n",
            " \n",
            "key2\n",
            "- value2\n",
            "key3: value3\n",
            "key4\n",
            "- value4\n",
        ));
        assert_eq!(mappings.len(), 4);
        assert_string(mappings.remove(0), "key1", "value1");
        assert_list(mappings.remove(0), "key2", &["value2"]);
        assert_string(mappings.remove(0), "key3", "value3");
        assert_list(mappings.remove(0), "key4", &["value4"]);
    }
}