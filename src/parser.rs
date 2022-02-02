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
    use crate::*;

    fn assert_list(mapping: Option<&Mapping>, key: &str, value: &[&str]) {
        let mapping = mapping.unwrap();
        let list: List = value.iter().copied().map(Into::into).collect();
        assert_eq!(mapping.key, key);
        assert_eq!(mapping.value, list.into());
    }

    fn assert_string(mapping: Option<&Mapping>, key: &str, value: &str) {
        let mapping = mapping.unwrap();
        assert_eq!(mapping.key, key);
        assert_eq!(mapping.value, value.into());
    }

    #[test]
    fn test_string() {
        let mappings = from_str("key: value");
        assert_eq!(mappings.len(), 1);
        assert_string(mappings.get(0), "key", "value");
    }

    #[test]
    fn test_string_with_whitespace() {
        let mappings = from_str(" key  with  whitespace :  value  with  whitespace ");
        assert_eq!(mappings.len(), 1);
        assert_string(
            mappings.get(0),
            "key  with  whitespace",
            "value  with  whitespace",
        );
    }

    #[test]
    fn test_list() {
        let mappings = from_str("key\n- value1\n- value2");
        assert_eq!(mappings.len(), 1);
        assert_list(mappings.get(0), "key", &["value1", "value2"]);
    }

    #[test]
    fn test_list_with_whitespace() {
        let mappings = from_str(" key  with  whitespace \n -  value 1 \n -  value 2 ");
        assert_eq!(mappings.len(), 1);
        assert_list(
            mappings.get(0),
            "key  with  whitespace",
            &["value 1", "value 2"],
        );
    }

    #[test]
    fn test_empty() {
        let mappings = from_str(" ");
        assert_eq!(mappings.len(), 0);
    }

    #[test]
    fn test_empty_lists() {
        let mappings = from_str("key1\nkey2");
        assert_eq!(mappings.len(), 2);
        assert_list(mappings.get(0), "key1", &[]);
        assert_list(mappings.get(1), "key2", &[]);
    }

    #[test]
    fn test_mixed() {
        let mappings = from_str(concat!(
            "key1: value1\n",
            " \n",
            "key2\n",
            "- value2\n",
            "key3: value3\n",
            "key4\n",
            "- value4\n",
        ));
        assert_eq!(mappings.len(), 4);
        assert_string(mappings.get(0), "key1", "value1");
        assert_list(mappings.get(1), "key2", &["value2"]);
        assert_string(mappings.get(2), "key3", "value3");
        assert_list(mappings.get(3), "key4", &["value4"]);
    }
}
