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

pub use once_cell;

#[macro_export]
macro_rules! regex {
    ($s: literal) => {{
        use lazy_regex::once_cell::sync::OnceCell;
        static RE: OnceCell::<regex::Regex> = OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($s).unwrap())
    }};
}
