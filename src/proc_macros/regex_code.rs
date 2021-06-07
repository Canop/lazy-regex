use {
    proc_macro2,
    quote::quote,
    syn::LitStr,
};

/// The lazy static regex building code, which is produced and
/// inserted by all lazy-regex macros
pub(crate) struct RegexCode {
    pub regex: regex::Regex,
    pub build: proc_macro2::TokenStream,
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


