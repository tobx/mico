use std::io;

pub type Mappings = Vec<(String, Value)>;

pub type List = Vec<String>;

#[derive(Debug)]
pub enum Value {
    List(List),
    String(String),
}

#[derive(Default)]
pub struct Parser {
    mappings: Mappings,
    list: Option<(String, List)>,
}

impl Parser {
    pub fn parse<R: io::BufRead>(mut self, reader: R) -> io::Result<Mappings> {
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if !line.is_empty() {
                self.parse_line(line);
            }
        }
        if let Some((key, list)) = self.list {
            self.mappings.push((key, Value::List(list)));
        }
        Ok(self.mappings)
    }

    fn parse_line(&mut self, line: &str) {
        if let Some((key, mut list)) = self.list.take() {
            if let Some(item) = line.strip_prefix("- ") {
                list.push(item.trim_start().into());
                self.list = Some((key, list));
                return;
            }
            self.mappings.push((key, Value::List(list)));
        }
        if let Some((key, value)) = line.split_once(": ") {
            let key = key.trim_end().into();
            let value = value.trim_start().into();
            self.mappings.push((key, Value::String(value)));
        } else {
            self.list = Some((line.into(), List::new()));
        }
    }
}

#[cfg(test)]
mod tests;
