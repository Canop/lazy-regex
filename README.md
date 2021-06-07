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

This macro returns references to normal instances of `regex::Regex` so all the usual features are available.

You may also use shortcut macros for testing a match, replacing with concise closures, or capturing groups as substrings in some common situations:

* `regex_is_match!`
* `regex_find!`
* `regex_captures!`
* `regex_replace!`
* `regex_replace_all!`

Some structs of the regex crate are reexported to ease dependency managment.

# Build Regexes

```rust
use lazy_regex::regex;

// build a simple regex
let r = regex!("sa+$");
assert_eq!(r.is_match("Saa"), false);

// build a regex with flag(s)
let r = regex!("sa+$"i);
assert_eq!(r.is_match("Saa"), true);

// you can use a raw literal
let r = regex!(r#"^"+$"#);
assert_eq!(r.is_match("\"\""), true);

// or a raw literal with flag(s)
let r = regex!(r#"^\s*("[a-t]*"\s*)+$"#i);
assert_eq!(r.is_match(r#" "Aristote" "Platon" "#), true);

// there's no problem using the multiline definition syntax
let r = regex!(r#"(?x)
    (?P<name>\w+)
    -
    (?P<version>[0-9.]+)
"#);
assert_eq!(r.find("This is lazy_regex-2.2!").unwrap().as_str(), "lazy_regex-2.2");
// (look at the regex_captures! macro to easily extract the groups)

// this line wouldn't compile because the regex is invalid:
// let r = regex!("(unclosed");

```
Supported regex flags: 'i', 'm', 's', 'x', 'U'.

See `regex::RegexBuilder`.

# Test a match

```rust
use lazy_regex::regex_is_match;

let b = regex_is_match!("[ab]+", "car");
assert_eq!(b, true);
```

doc: `regex_is_match!`


# Extract a value

```rust
use lazy_regex::regex_find;

let f_word = regex_find!(r#"\bf\w+\b"#, "The fox jumps.");
assert_eq!(f_word, Some("fox"));
```

doc: `regex_find!`

# Capture

```rust
use lazy_regex::regex_captures;

let (_, letter) = regex_captures!("([a-z])[0-9]+"i, "form A42").unwrap();
assert_eq!(letter, "A");

let (whole, name, version) = regex_captures!(
    r#"(\w+)-([0-9.]+)"#, // a literal regex
    "This is lazy_regex-2.0!", // any expression
).unwrap();
assert_eq!(whole, "lazy_regex-2.0");
assert_eq!(name, "lazy_regex");
assert_eq!(version, "2.0");
```

There's no limit to the size of the tuple.
It's checked at compile time to ensure you have the right number of capturing groups.

You receive `""` for optional groups with no value.

doc: `regex_captures!`

# Replace with captured groups

```rust
use lazy_regex::regex_replace_all;

let text = "Foo8 fuu3";
let text = regex_replace_all!(
    r#"\bf(\w+)(\d)"#i,
    text,
    |_, name, digit| format!("F<{}>{}", name, digit),
);
assert_eq!(text, "F<oo>8 F<uu>3");
```
The number of arguments given to the closure is checked at compilation time to match the number of groups in the regular expression.

doc: `regex_replace!` and `regex_replace_all!`

# Shared lazy static

When a regular expression is used in several functions, you sometimes don't want
to repeat it but have a shared static instance.

The `regex!` macro, while being backed by a lazy static regex, returns a reference.

If you want to have a shared lazy static regex, use the `lazy_regex!` macro:

```rust
use lazy_regex::*;

pub static GLOBAL_REX: Lazy<Regex> = lazy_regex!("^ab+$"i);
```

Like for the other macros, the regex is static, checked at compile time, and lazily built at first use.

doc: `lazy_regex!`

*/

pub use {
    lazy_regex_proc_macros::{
        lazy_regex, regex,
        regex_captures,
        regex_find,
        regex_is_match,
        regex_replace,
        regex_replace_all,
    },
    once_cell::sync::Lazy,
    regex::{Captures, Regex, RegexBuilder},
};
