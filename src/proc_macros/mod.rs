use std::convert::TryFrom;

mod args;
mod regex_code;

use {
    crate::{args::*, regex_code::*},
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Expr, ExprClosure},
};

//  The following `process*` functions are convenience funcs
//  to reduce boilerplate in macro implementations below.
fn process<T, F>(input: TokenStream, f: F) -> TokenStream
where
    T: Into<TokenStream>,
    F: Fn(RegexCode) -> T,
{
    match RegexCode::try_from(input) {
        Ok(r) => f(r).into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn process_with_value<T, F>(input: TokenStream, f: F) -> TokenStream
where
    T: Into<TokenStream>,
    F: Fn(RegexCode, Expr) -> T,
{
    let parsed = parse_macro_input!(input as RexValArgs);
    match RegexCode::try_from(parsed.regex_str) {
        Ok(r) => f(r, parsed.value).into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn process_with_value_fun<T, F>(input: TokenStream, f: F) -> TokenStream
where
    T: Into<TokenStream>,
    F: Fn(RegexCode, Expr, ExprClosure) -> T,
{
    let parsed = parse_macro_input!(input as RexValFunArgs);
    match RegexCode::try_from(parsed.regex_str) {
        Ok(r) => f(r, parsed.value, parsed.fun).into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Return a lazy static Regex checked at compilation time and
/// built at first use.
///
/// Flags can be specified as suffix:
/// ```
/// let case_insensitive_regex = regex!("^ab+$"i);
/// ```
///
/// The macro returns a reference to a [regex::Regex]
/// or a [regex::bytes::Regex] instance,
/// differentiated by the `B` flag:
/// ```
/// let verbose = regex!(r#"_([\d\.]+)"#)
///     .replace("This is lazy-regex_2.2", " (version $1)");
/// assert_eq!(verbose, "This is lazy-regex (version 2.2)");
/// ```
#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    process(input, |regex_code| regex_code.lazy_static())
}

/// Return an instance of `once_cell::sync::Lazy<regex::Regex>` or
/// `once_cell::sync::Lazy<regex::bytes::Regex>` that
/// you can use in a public static declaration.
///
/// Example:
///
/// ```
/// pub static GLOBAL_REX: Lazy<Regex> = lazy_regex!("^ab+$"i);
/// ```
///
/// As for other macros, the regex is checked at compilation time.
#[proc_macro]
pub fn lazy_regex(input: TokenStream) -> TokenStream {
    process(input, |regex_code| regex_code.build)
}

/// Test whether an expression matches a lazy static
/// regular expression (the regex is checked at compile
/// time)
///
/// Example:
/// ```
/// let b = regex_is_match!("[ab]+", "car");
/// assert_eq!(b, true);
/// ```
#[proc_macro]
pub fn regex_is_match(input: TokenStream) -> TokenStream {
    process_with_value(input, |regex_code, value| {
        let statick = regex_code.statick();
        quote! {{
            #statick;
            RE.is_match(#value)
        }}
    })
}

/// Extract the leftmost match of the regex in the
/// second argument, as a `&str`, or a `&[u8]` if the `B` flag is set.
///
/// Example:
/// ```
/// let f_word = regex_find!(r#"\bf\w+\b"#, "The fox jumps.");
/// assert_eq!(f_word, Some("fox"));
/// let f_word = regex_find!(r#"\bf\w+\b"#B, "The forest is silent.");
/// assert_eq!(f_word, Some(b"forest" as &[u8]));
/// ```
#[proc_macro]
pub fn regex_find(input: TokenStream) -> TokenStream {
    process_with_value(input, |regex_code, value| {
        let statick = regex_code.statick();
        let as_method = match regex_code.regex {
            RegexInstance::Regex(..) => quote!(as_str),
            RegexInstance::Bytes(..) => quote!(as_bytes),
        };
        quote! {{
            #statick;
            RE.find(#value).map(|mat| mat. #as_method ())
        }}
    })
}

/// Extract captured groups as a tuple of &str.
///
/// If there's no match, the macro returns `None`.
///
/// If an optional group has no value, the tuple
/// will contain `""` instead.
///
/// Example:
/// ```
/// let (whole, name, version) = regex_captures!(
///     r#"(\w+)-([0-9.]+)"#, // a literal regex
///     "This is lazy_regex-2.0!", // any expression
/// ).unwrap();
/// assert_eq!(whole, "lazy_regex-2.0");
/// assert_eq!(name, "lazy_regex");
/// assert_eq!(version, "2.0");
/// ```
#[proc_macro]
pub fn regex_captures(input: TokenStream) -> TokenStream {
    process_with_value(input, |regex_code, value| {
        let statick = regex_code.statick();
        let n = regex_code.captures_len();
        let groups = (0..n).map(|i| {
            quote! {
                caps.get(#i).map_or("", |c| c.as_str())
            }
        });
        quote! {{
            #statick;
            RE.captures(#value)
                .map(|caps| (
                    #(#groups),*
                ))
        }}
    })
}

/// common implementation of regex_replace and regex_replace_all
fn replacen(input: TokenStream, limit: usize) -> TokenStream {
    process_with_value_fun(input, |regex_code, value, fun| {
        let statick = regex_code.statick();
        let n = regex_code.captures_len();
        let groups = (0..n).map(|i| {
            quote! {
                caps.get(#i).map_or("", |c| c.as_str())
            }
        });
        quote! {{
            #statick;
            RE.replacen(
                #value,
                #limit,
                |caps: &lazy_regex::Captures<'_>| {
                    let fun = #fun;
                    fun(
                        #(#groups),*
                    )
                })
        }}
    })
}

/// Replaces the leftmost match in the second argument
/// with the value returned by the closure given as third argument.
///
/// The closure is given one or more `&str`, the first one for
/// the whole match and the following ones for the groups.
/// Any optional group with no value is replaced with `""`.
///
/// Example:
/// ```
/// let text = "Fuu fuuu";
/// let text = regex_replace!(
///     "f(u*)"i,
///     text,
///     |_, suffix: &str| format!("F{}", suffix.len()),
/// );
/// assert_eq!(text, "F2 fuuu");
/// ```
#[proc_macro]
pub fn regex_replace(input: TokenStream) -> TokenStream {
    replacen(input, 1)
}

/// Replaces all non-overlapping matches in the second argument
/// with the value returned by the closure given as third argument.
///
/// The closure is given one or more `&str`, the first one for
/// the whole match and the following ones for the groups.
/// Any optional group with no value is replaced with `""`.
///
/// Example:
/// ```
/// let text = "Foo fuu";
/// let text = regex_replace_all!(
///     r#"\bf(?P<suffix>\w+)"#i,
///     text,
///     |_, suffix| format!("F<{}>", suffix),
/// );
/// assert_eq!(text, "F<oo> F<uu>");
/// ```
#[proc_macro]
pub fn regex_replace_all(input: TokenStream) -> TokenStream {
    replacen(input, 0)
}
