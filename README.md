[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/lazy-regex.svg
[l1]: https://crates.io/crates/lazy-regex

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/lazy-regex/badge.svg
[l3]: https://docs.rs/lazy-regex/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3


# lazy-regex

Use the  `regex!` macro to build regexes:

* they're checked at compile time
* they're wrapped in `once_cell` lazy static initializers so that they're compiled only once
* they can hold flags as suffix: `let case_insensitive_regex = regex!("ab*"i);`
* regex creation is less verbose

This macro builds normal instances of `regex::Regex` so all the usual features are available.

You may also use shortcut macros for testing a match or capturing groups as substrings:

* `regex_is_match!`
* `regex_find!`
* `regex_captures!`

# Build Regexes

```rust
use lazy_regex::regex;

// build a simple regex
let r = regex!("sa+$");
assert_eq!(r.is_match("Saa"), false);

// build a regex with flag(s)
let r = regex!("sa+$"i);
assert_eq!(r.is_match("Saa"), true);

// supported regex flags: 'i', 'm', 's', 'x', 'U'
// see https://docs.rs/regex/1.5.4/regex/struct.RegexBuilder.html

// you can use a raw literal
let r = regex!(r#"^"+$"#);
assert_eq!(r.is_match("\"\""), true);

// or a raw literal with flag(s)
let r = regex!(r#"^\s*("[a-t]*"\s*)+$"#i);
assert_eq!(r.is_match(r#" "Aristote" "Platon" "#), true);

// this line wouldn't compile:
// let r = regex!("(unclosed");

```

# Test

```rust
use lazy_regex::regex_is_match;

let b = regex_is_match!("[ab]+", "car");
assert_eq!(b, true);
```

# Extract

```rust
use lazy_regex::regex_find;

let f_word = regex_find!(r#"\bf\w+\b"#, "The fox jumps.").unwrap();
assert_eq!(f_word, "fox");
```

# Capture

```rust
use lazy_regex::regex_captures;

let (_, letter) = regex_captures!(r#"([a-z])\d+"#i, "form A42").unwrap();
assert_eq!(letter, "A");

let (whole, name, version) = regex_captures!(
    r#"(\w+)-([0-9.]+)"#, // a literal regex
    "This is lazy_regex-2.0!", // any expression
).unwrap();
assert_eq!(whole, "lazy_regex-2.0");
assert_eq!(name, "lazy_regex");
assert_eq!(version, "2.0");
```

The size of the tupple is checked at compile time and ensures you have the right number of capturing groups.
