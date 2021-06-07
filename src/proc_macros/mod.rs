mod args;
mod regex_code;

use {
    crate::{
        args::*,
        regex_code::*,
    },
    proc_macro::TokenStream,
    quote::quote,
    syn::parse_macro_input,
};

/// Return a lazy static Regex checked at compilation time and
/// built at first use.
///
/// Flags can be specified as suffix:
/// ```
/// let case_insensitive_regex = regex!("^ab+$"i);
/// ```
///
/// The macro returns a reference to a [regex::Regex] instance:
/// ```
/// let verbose = regex!(r#"_([\d\.]+)"#)
///     .replace("This is lazy-regex_2.2", " (version $1)");
/// assert_eq!(verbose, "This is lazy-regex (version 2.2)");
/// ```
#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let lit_str = syn::parse::<syn::LitStr>(input).unwrap();
    let regex_build = RegexCode::from(lit_str).build;
    let q = quote! {{
        static RE: lazy_regex::Lazy<lazy_regex::Regex> = #regex_build;
        &RE
    }};
    q.into()
}

/// Return an instance of `once_cell::sync::Lazy<regex::Regex>` that
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
    let lit_str = syn::parse::<syn::LitStr>(input).unwrap();
    let regex_build = RegexCode::from(lit_str).build;
    regex_build.into()
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
    let regex_and_expr_args = parse_macro_input!(input as RexValArgs);
    let regex_build = RegexCode::from(regex_and_expr_args.regex_str).build;
    let value = regex_and_expr_args.value;
    let q = quote! {{
        static RE: lazy_regex::Lazy<lazy_regex::Regex> = #regex_build;
        RE.is_match(#value)
    }};
    q.into()
}

/// Extract the leftmost match of the regex in the
/// second argument, as a &str
///
/// Example:
/// ```
/// let f_word = regex_find!(r#"\bf\w+\b"#, "The fox jumps.");
/// assert_eq!(f_word, Some("fox"));
/// ```
#[proc_macro]
pub fn regex_find(input: TokenStream) -> TokenStream {
    let regex_and_expr_args = parse_macro_input!(input as RexValArgs);
    let regex_code = RegexCode::from(regex_and_expr_args.regex_str);
    let regex_build = regex_code.build;
    let value = regex_and_expr_args.value;
    let q = quote! {{
        static RE: lazy_regex::Lazy<lazy_regex::Regex> = #regex_build;
        RE.find(#value).map(|mat| mat.as_str())
    }};
    q.into()
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
    let regex_and_expr_args = parse_macro_input!(input as RexValArgs);
    let regex_code = RegexCode::from(regex_and_expr_args.regex_str);
    let regex_build = regex_code.build;
    let value = regex_and_expr_args.value;
    let n = regex_code.regex.captures_len();
    let groups = (0..n).map(|i| {
        quote! {
            caps.get(#i).map_or("", |c| c.as_str())
        }
    });
    let q = quote! {{
        static RE: lazy_regex::Lazy<lazy_regex::Regex> = #regex_build;
        RE.captures(#value)
            .map(|caps| (
                #(#groups),*
            ))
    }};
    q.into()
}

/// common implementation of regex_replace and regex_replace_all
fn replacen(input: TokenStream, limit: usize) -> TokenStream {
    let args = parse_macro_input!(input as RexValFunArgs);
    let regex_code = RegexCode::from(args.regex_str);
    let regex_build = regex_code.build;
    let value = args.value;
    let fun = args.fun;
    let n = regex_code.regex.captures_len();
    let groups = (0..n).map(|i| {
        quote! {
            caps.get(#i).map_or("", |c| c.as_str())
        }
    });
    let q = quote! {{
        static RE: lazy_regex::Lazy<lazy_regex::Regex> = #regex_build;
        RE.replacen(
            #value,
            #limit,
            |caps: &lazy_regex::Captures<'_>| {
                let fun = #fun;
                fun(
                    #(#groups),*
                )
            })
    }};
    q.into()
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
