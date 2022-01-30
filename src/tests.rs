use std::io::Cursor;

use crate::*;

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
    let mut mappings = parse("key\n- item1\n- item2");
    assert_eq!(mappings.len(), 1);
    assert_list(mappings.remove(0), "key", vec!["item1", "item2"]);
}

#[test]
fn test_list_with_whitespace() {
    let mut mappings = parse(" key  with  whitespace \n -  item 1 \n -  item 2 ");
    assert_eq!(mappings.len(), 1);
    assert_list(
        mappings.remove(0),
        "key  with  whitespace",
        vec!["item 1", "item 2"],
    );
}

#[test]
fn test_empty_lists() {
    let mut mappings = parse("key1\nkey2");
    assert_eq!(mappings.len(), 2);
    assert_list(mappings.remove(0), "key1", Vec::new());
    assert_list(mappings.remove(0), "key2", Vec::new());
}

#[test]
fn test_mixed() {
    let mut mappings = parse("key1: value1\n \nkey2\n- item1\n- item2\nkey3: value2");
    assert_eq!(mappings.len(), 3);
    assert_string(mappings.remove(0), "key1", "value1");
    assert_list(mappings.remove(0), "key2", vec!["item1", "item2"]);
    assert_string(mappings.remove(0), "key3", "value2");
}

fn assert_list(mapping: (String, Value), key: &str, value: Vec<&str>) {
    let (k, v) = mapping;
    assert_eq!(k, key);
    match v {
        Value::List(list) => assert_eq!(list, value),
        Value::String(_) => panic!("expected enum `Value::String`, found enum `Value::List`"),
    }
}

fn assert_string(mapping: (String, Value), key: &str, value: &str) {
    let (k, v) = mapping;
    assert_eq!(k, key);
    match v {
        Value::String(s) => assert_eq!(s, value),
        Value::List(_) => panic!("expected enum `Value::String`, found enum `Value::List`"),
    }
}

fn parse(config: &str) -> Mappings {
    Parser::default().parse(Cursor::new(config)).unwrap()
}
