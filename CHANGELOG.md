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
