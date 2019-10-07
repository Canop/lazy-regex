#[macro_export]
macro_rules! regex {
    ($s: literal) => {{
        lazy_static! {
            static ref RE: Regex = Regex::new($s).unwrap();
        }
        &*RE
    }};
}
