<a name="v2.3.1"></a>
### v2.3.1 - 2022-11-03
- better error messages on bad regexes - thanks @necauqua

<a name="v2.3.0"></a>
### v2.3.0 - 2022-03-05
- support for [bytes](https://docs.rs/regex/latest/regex/bytes/index.html) regexes with the `B` suffix notation - thanks @bnoctis - Fix #11

<a name="v2.2.2"></a>
### v2.2.2 - 2021-10-20
Reexpose features of the regex crate

<a name="v2.2.1"></a>
### v2.2.1 - 2021-06-07
Add the `regex_replace!` macro for when you only want to replace one match
Reexports more types of the regex crates

<a name="v2.2.0"></a>
### v2.2.0 - 2021-06-04
Add the `regex_replace_all!` macro to do replacements with a closure taking the right number of `&str` arguments according to the number of groups in the regular expression

<a name="v2.1.0"></a>
### v2.1.0 - 2021-06-02
Add the `lazy_regex!` macro returning a `Lazy<Regex>` for easy use in a `pub static` shared declaration.

<a name="v2.0.2"></a>
### v2.0.2 - 2021-05-31
Fix a cross compilation problem, thanks @AlephAlpha - Fix #5

<a name="v2.0.1"></a>
### v2.0.1 - 2021-05-20
Improved documentation

<a name="v2.0.0"></a>
### v2.0.0 - 2021-05-17
- regular expressions are now checked at compile time
- regex_is_match!
- regex_find!
- regex_captures!

<a name="v1.1.0"></a>
### v1.1.0 - 2021-05-08
- no more complementary import needed
- now based on once_cell instead of lazy_static

<a name="v1.0.0"></a>
### v1.0.0 - 2021-05-04
- first public release
