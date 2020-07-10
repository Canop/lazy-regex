/*!

This crate introduces the `regex!` macro which is a shortcut to write static lazily compiled regular expressions as is usually done with lazy_static or once_cell.

It lets you replace


```not-executable
fn some_helper_function(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("...").unwrap();
    }
    RE.is_match(text)
}

```

with


```not-executable
fn some_helper_function(text: &str) -> bool {
    regex!("...").is_match(text)
}
```

*/

#[macro_export]
macro_rules! regex {
    ($s: literal) => {{
        use regex::Regex;
        lazy_static! {
            static ref RE: Regex = Regex::new($s).unwrap();
        }
        &*RE
    }};
}
