use {
    proc_macro::TokenStream,
    proc_macro2,
    quote::quote,
    syn::{
        Expr,
        ExprClosure,
        LitStr,
        Token,
        parse::{
            Parse,
            ParseStream,
            Result,
        },
        parse_macro_input,
    },
};

struct RegexCode {
    regex: regex::Regex,
    build: proc_macro2::TokenStream,
}

impl From<LitStr> for RegexCode {
    fn from(lit_str: LitStr) -> Self {
        let regex_string = lit_str.value();
        let mut case_insensitive = false;
        let mut multi_line = false;
        let mut dot_matches_new_line = false;
        let mut ignore_whitespace = false;
        let mut swap_greed = false;
        for ch in lit_str.suffix().chars() {
            match ch {
                'i' => case_insensitive = true,
                'm' => multi_line = true,
                's' => dot_matches_new_line = true,
                'x' => ignore_whitespace = true,
                'U' => swap_greed = true,
                _ => {
                    panic!("unrecognized regex flag {:?}", ch);
                }
            };
        }

        // the next line prevents compilation if the
        // literal is invalid as a regular expression
        let regex = regex::Regex::new(&regex_string).unwrap();

        let build = quote! {{
            lazy_regex::Lazy::new(|| {
                //println!("compiling regex {:?}", #regex_string);
                let mut builder = lazy_regex::RegexBuilder::new(#regex_string);
                builder.case_insensitive(#case_insensitive);
                builder.multi_line(#multi_line);
                builder.dot_matches_new_line(#dot_matches_new_line);
                builder.ignore_whitespace(#ignore_whitespace);
                builder.swap_greed(#swap_greed);
                builder.build().unwrap()
            })
        }};

        Self { regex, build }
    }
}

/// Return a lazy static Regex checked at compilation time.
///
/// Flags can be specified as suffix:
/// ```
/// let case_insensitive_regex = regex!("^ab+$"i);
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
/// use lazy_regex::*;
///
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

/// Wrapping of the two arguments given to one of the
/// `regex_is_match`, `regex_find`, or `regex_captures`
/// macros
struct RegexAndExpr {
    regex_str: LitStr,
    value: Expr,
}
impl Parse for RegexAndExpr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let regex_str = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let value = input.parse::<Expr>()?;
        let _ = input.parse::<Token![,]>(); // allow a trailing comma
        Ok(RegexAndExpr {
            regex_str,
            value,
        })
    }
}

/// Wrapping of the three arguments given to the
/// `regex_replace_all` macro
struct RegexAnd2Exprs {
    regex_str: LitStr,
    value: Expr,
    fun: ExprClosure,
}
impl Parse for RegexAnd2Exprs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let regex_str = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let value = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let fun = input.parse::<ExprClosure>()?;
        let _ = input.parse::<Token![,]>(); // allow a trailing comma
        Ok(RegexAnd2Exprs {
            regex_str,
            value,
            fun,
        })
    }
}

/// Test whether an expression matches a lazy static
/// regular expression (the regex is checked at compile
/// time)
///
/// Example:
/// ```
/// use lazy_regex::regex_is_match;
///
/// let b = regex_is_match!("[ab]+", "car");
/// assert_eq!(b, true);
/// ```
#[proc_macro]
pub fn regex_is_match(input: TokenStream) -> TokenStream {
    let regex_and_expr_args = parse_macro_input!(input as RegexAndExpr);
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
    let regex_and_expr_args = parse_macro_input!(input as RegexAndExpr);
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
    let regex_and_expr_args = parse_macro_input!(input as RegexAndExpr);
    let regex_code = RegexCode::from(regex_and_expr_args.regex_str);
    let regex_build = regex_code.build;
    let value = regex_and_expr_args.value;
    let n = regex_code.regex.captures_len();
    let groups = (0..n).map(|i| quote! {
            caps.get(#i).map_or("", |c| c.as_str())
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

/// Replaces all non-overlapping matches in the second argument
/// with the value returned by the closure given as third argument.
///
/// The closure is given one or more `&str`, the first one for
/// the whole match and the following ones for the groups.
/// Any optional group with no value is replaced with `""`.
///
/// Example:
/// ```
/// use lazy_regex::*;
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
    let args = parse_macro_input!(input as RegexAnd2Exprs);
    let regex_code = RegexCode::from(args.regex_str);
    let regex_build = regex_code.build;
    let value = args.value;
    let fun = args.fun;
    let n = regex_code.regex.captures_len();
    let groups = (0..n).map(|i| quote! {
            caps.get(#i).map_or("", |c| c.as_str())
        });
    let q = quote! {{
        static RE: lazy_regex::Lazy<lazy_regex::Regex> = #regex_build;
        RE.replace_all(
            #value,
            |caps: &lazy_regex::Captures<'_>| {
                let fun = #fun;
                fun(
                    #(#groups),*
                )
            })
    }};
    q.into()
}
