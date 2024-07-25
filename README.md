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

With lazy-regex macros, regular expressions

* are checked at compile time, with clear error messages
* are wrapped in `once_cell` lazy static initializers so that they're compiled only once
* can hold flags as suffix: `let case_insensitive_regex = regex!("ab*"i);`
* are defined in a less verbose way

The `regex!` macro returns references to normal instances of `regex::Regex` or `regex::bytes::Regex` so all the usual features are available.

Other macros are specialized for testing a match, replacing with concise closures, or capturing groups as substrings in some common situations:

* `regex_is_match!`
* `regex_find!`
* `regex_captures!`
* `regex_replace!`
* `regex_replace_all!`
* `regex_switch!`

They support the `B` flag for the `regex::bytes::Regex` variant.

All macros exist with a `bytes_` prefix for building `bytes::Regex`, so you also have `bytes_regex!`, `bytes_regex_is_match!`, `bytes_regex_find!`, `bytes_regex_captures!`, `bytes_regex_replace!`, `bytes_regex_replace_all!`, and `bytes_regex_switch!`.

Some structs of the regex crate are reexported to ease dependency managment.
The regex crate itself is also reexported, to avoid the need to synchronize the versions/flavor (see [Features](#features_and_reexport) below)

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

// build a regex that operates on &[u8]
let r = regex!("(byte)?string$"B);
assert_eq!(r.is_match(b"bytestring"), true);

// there's no problem using the multiline definition syntax
let r = regex!(r#"(?x)
    (?P<name>\w+)
    -
    (?P<version>[0-9.]+)
"#);
assert_eq!(r.find("This is lazy_regex-2.2!").unwrap().as_str(), "lazy_regex-2.2");
// (look at the regex_captures! macro to easily extract the groups)

```
```compile_fail
// this line doesn't compile because the regex is invalid:
let r = regex!("(unclosed");

```
Supported regex flags: `i`, `m`, `s`, `x`, `U`.

See [regex::RegexBuilder](https://docs.rs/regex/latest/regex/struct.RegexBuilder.html).

# Test a match

```rust
use lazy_regex::regex_is_match;

let b = regex_is_match!("[ab]+", "car");
assert_eq!(b, true);
```


# Extract a value

```rust
use lazy_regex::regex_find;

let f_word = regex_find!(r#"\bf\w+\b"#, "The fox jumps.");
assert_eq!(f_word, Some("fox"));
let f_word = regex_find!(r#"\bf\w+\b"#B, b"The forest is silent.");
assert_eq!(f_word, Some(b"forest" as &[u8]));
```

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

# Replace with captured groups

The [regex_replace!] and [regex_replace_all!] macros bring once compilation and compilation time checks to the `replace` and `replace_all` functions.

## Replace with a closure

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

If it doesn't match you get, at compilation time, a clear error message.

## Replace with another kind of Replacer

```rust
use lazy_regex::regex_replace_all;
let text = "UwU";
let output = regex_replace_all!("U", text, "O");
assert_eq!(&output, "OwO");
```

# Switch over regexes

Execute the expression bound to the first matching regex, with named captured groups declared as varibles:

```rust
use lazy_regex::regex_switch;
pub enum ScrollCommand {
    Top,
    Bottom,
    Lines(i32),
    Pages(i32),
}
impl std::str::FromStr for ScrollCommand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        regex_switch!(s,
            "^scroll-to-top$" => Self::Top,
            "^scroll-to-bottom$" => Self::Bottom,
            r#"^scroll-lines?\((?<n>[+-]?\d{1,4})\)$"# => Self::Lines(n.parse().unwrap()),
            r#"^scroll-pages?\((?<n>[+-]?\d{1,4})\)$"# => Self::Pages(n.parse().unwrap()),
        ).ok_or(())
    }
}
```

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

# Features and reexport

With default features, `lazy-regex` use the `regex` crate with its default features, tailored for performances and complete Unicode support.

You may enable a different set of regex features by directly enabling them when importing `lazy-regex`.

It's also possible to use the [regex-lite](https://docs.rs/regex-lite/) crate instead of the [regex](https://docs.rs/regex/) crate by declaring the ``lite`` feature:

```TOML
lazy-regex = { version = "3.0", default-features = false, features = ["lite"] }
```

The `lite` flavor comes with slightly lower performances and a reduced Unicode support (see crate documentation) but also a much smaller binary size.

If you need to refer to the regex crate in your code, prefer to use the reexport (i.e. `use lazy_regex::regex;`) so that you don't have a version or flavor conflict. When the `lite` feature is enabled, `lazy_regex::regex` refers to `regex_lite` so you don't have to change your code when switching regex engine.

## `#![no_std]` support

If you need to use this crate in a `no_std` environment, you must enable the "critical-section" feature.
