/*!

With lazy-regex macros, regular expressions

* are checked at compile time, with clear error messages
* are wrapped in `once_cell` lazy static initializers so that they're compiled only once
* can hold flags as suffix: `let case_insensitive_regex = regex!("ab*"i);`
* are defined in a less verbose way

The [regex!] macro returns references to normal instances of [regex::Regex] or [regex::bytes::Regex] so all the usual features are available.

But most often, you won't even use the `regex!` macro but the other macros which are specialized for testing a match, replacing, or capturing groups in some common situations:

* [regex_is_match!]
* [regex_find!]
* [regex_captures!]
* [regex_replace!]
* [regex_replace_all!]
* [regex_switch!]

They support the `B` flag for the `regex::bytes::Regex` variant.

All macros exist with a `bytes_` prefix for building `bytes::Regex`, so you also have [bytes_regex!], [bytes_regex_is_match!], [bytes_regex_find!], [bytes_regex_captures!], [bytes_regex_replace!], [bytes_regex_replace_all!], and [bytes_regex_switch!].

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
Supported regex flags: [`i`, `m`, `s`, `x`, `U`][regex::RegexBuilder], and you may also use `B` to build a bytes regex.

The following regexes are equivalent:
* `bytes_regex!("^ab+$"i)`
* `bytes_regex!("(?i)^ab+$")`
* `regex!("^ab+$"iB)`
* `regex!("(?i)^ab+$"B)`

They're all case insensitive instances of `regex::bytes::Regex`.


# Test a match

```rust
use lazy_regex::*;

let b = regex_is_match!("[ab]+", "car");
assert_eq!(b, true);
let b = bytes_regex_is_match!("[ab]+", b"car");
assert_eq!(b, true);
```

doc: [regex_is_match!]


# Extract a value

```rust
use lazy_regex::regex_find;

let f_word = regex_find!(r#"\bf\w+\b"#, "The fox jumps.");
assert_eq!(f_word, Some("fox"));
let f_word = regex_find!(r#"\bf\w+\b"#B, b"The forest is silent.");
assert_eq!(f_word, Some(b"forest" as &[u8]));
```

doc: [regex_find!]

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

doc: [regex_captures!]

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

doc: [regex_switch!]

# Shared lazy static

When a regular expression is used in several functions, you sometimes don't want
to repeat it but have a shared static instance.

The [regex!] macro, while being backed by a lazy static regex, returns a reference.

If you want to have a shared lazy static regex, use the [lazy_regex!] macro:

```rust
use lazy_regex::*;

pub static GLOBAL_REX: Lazy<Regex> = lazy_regex!("^ab+$"i);
```

Like for the other macros, the regex is static, checked at compile time, and lazily built at first use.

doc: [lazy_regex!]

*/

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(all(feature = "std", feature = "critical-section"))]
compile_error!("The `critical-section` flag should only be used in embedded envrionemnts without a standard library.");

pub use {
    lazy_regex_proc_macros::{
        lazy_regex,
        regex,
        regex_captures,
        regex_find,
        regex_if,
        regex_is_match,
        regex_replace,
        regex_replace_all,
        regex_switch,
        bytes_lazy_regex,
        bytes_regex,
        bytes_regex_captures,
        bytes_regex_find,
        bytes_regex_if,
        bytes_regex_is_match,
        bytes_regex_replace,
        bytes_regex_replace_all,
        bytes_regex_switch,
    },
    once_cell::sync::Lazy,
};

#[cfg(not(feature = "lite"))]
pub use {
    regex::{
        self,
        Captures, Regex, RegexBuilder,
        bytes::{
            Regex as BytesRegex,
            RegexBuilder as BytesRegexBuilder
        },
    },
};

#[cfg(feature = "lite")]
pub use {
    regex_lite::{
        self as regex,
        Captures, Regex, RegexBuilder,
    },
};

