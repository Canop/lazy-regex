use {
    quote::quote,
    syn::LitStr,
    proc_macro::TokenStream,
    proc_macro2::TokenStream as TokenStream2,
};

fn quote_build(pattern: &str, is_bytes: bool,
        case_insensitive: bool, multi_line: bool,
        dot_matches_new_line: bool, ignore_whitespace: bool,
        swap_greed: bool) -> TokenStream2 {
    let builder_token = if is_bytes {
        quote! { BytesRegexBuilder }
    } else {
        quote! { RegexBuilder }
    };
    quote! {
        lazy_regex::Lazy::new(|| {
            //println!("compiling regex {:?}", #pattern);
            lazy_regex:: #builder_token ::new(#pattern)
                .case_insensitive(#case_insensitive)
                .multi_line(#multi_line)
                .dot_matches_new_line(#dot_matches_new_line)
                .ignore_whitespace(#ignore_whitespace)
                .swap_greed(#swap_greed)
                .build().unwrap()
        })
    }
}

/// The lazy static regex building code, which is produced and
/// inserted by all lazy-regex macros
pub(crate) struct RegexCode {
    pub build: TokenStream2,
    pub is_bytes: bool,
    regex: Option<regex::Regex>,
    regex_bytes: Option<regex::bytes::Regex>,
}

impl From<LitStr> for RegexCode {
    fn from(lit_str: LitStr) -> Self {
        let pattern = lit_str.value();
        let mut case_insensitive = false;
        let mut multi_line = false;
        let mut dot_matches_new_line = false;
        let mut ignore_whitespace = false;
        let mut swap_greed = false;
        let mut is_bytes = false;
        for ch in lit_str.suffix().chars() {
            match ch {
                'i' => case_insensitive = true,
                'm' => multi_line = true,
                's' => dot_matches_new_line = true,
                'x' => ignore_whitespace = true,
                'U' => swap_greed = true,
                'B' => is_bytes = true, // non-standard!
                _ => {
                    panic!("unrecognized regex flag {:?}", ch);
                }
            };
        }

        // also prevents compilation if the literal is invalid as
        // a regular expression
        let regex = if is_bytes {
            None
        } else {
            Some(regex::Regex::new(&pattern).unwrap())
        };
        let regex_bytes = if is_bytes {
            Some(regex::bytes::Regex::new(&pattern).unwrap())
        } else {
            None
        };

        let build = quote_build(&pattern, is_bytes,
            case_insensitive, multi_line, dot_matches_new_line,
            ignore_whitespace, swap_greed);
        Self {
            build,
            is_bytes,
            regex,
            regex_bytes,
        }
    }
}

impl From<TokenStream> for RegexCode {
    fn from(token_stream: TokenStream) -> Self {
        Self::from(syn::parse::<syn::LitStr>(token_stream).unwrap())
    }
}

impl RegexCode {
    pub fn regex(&self) -> &regex::Regex {
        self.regex.as_ref().unwrap()
    }

    pub fn regex_bytes(&self) -> &regex::bytes::Regex {
        self.regex_bytes.as_ref().unwrap()
    }

    pub fn statick(&self) -> TokenStream2 {
        let build = &self.build;
        let regex_token = if self.is_bytes {
            quote! { BytesRegex }
        } else {
            quote! { Regex }
        };
        quote! {
            static RE: lazy_regex::Lazy<lazy_regex:: #regex_token > = #build;
        }
    }

    pub fn lazy_static(&self) -> TokenStream2 {
        let statick = self.statick();
        quote! {{
            #statick;
            &RE
        }}
    }

    pub fn captures_len(&self) -> usize {
        if self.is_bytes {
            self.regex_bytes().captures_len()
        } else {
            self.regex().captures_len()
        }
    }
}

