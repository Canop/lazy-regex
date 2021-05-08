# lazy-regex
a macro for when you're tired of the regex creation boilerplate

## What it does

It's a shortcut to write static lazily compiled regular expressions as is usually done with lazy_static or once_cell.

It lets you replace


```
fn some_helper_function(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("...").unwrap();
    }
    RE.is_match(text)
}
```

with


```
fn some_helper_function(text: &str) -> bool {
    regex!("...").is_match(text)
}
```

The first code comes from the regex documentation.


## FAQ

### Is it really useful ?

Regarding the binary, it's as using lazy_static or once_cell.
It just makes some code a little easier to read. You're judge.

### Can I have several `regex!` in the same function ? On the same Line ?

Yes, no problem.

### It hides the `unwrap()`, isn't it concerning ?

Not so much in my opinion as the macro only accepts a litteral: you won't hide a failure occuring on a dynamic string.

### I'd like to have flags too

You mean something like `regex!("somestring", "i")` ? Cool. I was just waiting for somebody's else to ask for it. Create an issue and I'll see if I can easily wrap `RegexBuilder` to handle flags ala JavaScript.

### What's the licence ?

It's MIT. No attribution is needed.
