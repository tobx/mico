# mico

This library implements a parser and emitter for `mico` (minimalistic config file format).

Format example:

```plaintext
Name: mico
Description: minimalistic config file format

Benefits
 - easy to read and write for everyone
 - ludicrously simple parsing logic (roughly 30 lines of code)
```

## Parsing and emitting mico

There are two convenience functions to parse and emit:

```rust
use mico::Mapping;

// parse string
let mappings = mico::from_str("foo: bar");
assert_eq!(mappings[0].key, "foo");
assert_eq!(mappings[0].value, "bar".into());

// emit mappings
let mappings = [Mapping::new("foo", "bar")];
assert_eq!(mico::to_string(&mappings, 0), "foo: bar\n");
```

## Notes

`mico` is meant for people to write simple config files very fast. There is no
indentation, escaping or quoting to worry about, so any text line can simply be
pasted to a `mico` file without further editing.

There are only two types:

1. A mapping from key to string:

   ```plaintext
   key: value
   ```

2. A mapping from key to a list of strings

   ```plaintext
   key
    - value 1
    - value 2
   ```

Here is a mico example:

```plaintext
foo: bar
  indentation is possible: but does not matter
white space  :  will be trimmed
this is a key:this: is:a :value
empty string:

empty lines: will be ignored

list ...
 - keys must not include a colon
 - items start with '-'

this is an empty list, because this line does not include a colon

this is no list because of the colon at the end of this line:
 - this is an empty list
```

Here is the corresponding JSON example:

```json
[
  { "foo": "bar" },
  { "indentation is possible": "but does not matter" },
  { "white space": "will be trimmed" },
  { "this is a key": "this: is:a :value" },
  { "empty string": "" },
  { "empty lines": "will be ignored" },
  { "list ...": ["keys must not include a colon", "items start with '-'"] },
  { "This is an empty list, because this line does not include a colon": [] },
  { "This is no list because of the colon at the end of this line:": "" },
  { "- this is an empty list": [] }
]
```
