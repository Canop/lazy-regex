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
