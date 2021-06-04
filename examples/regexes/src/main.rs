use {
    lazy_regex::*,
};

pub static SHARED: Lazy<Regex> = lazy_regex!("^test$");

fn example_builds() {

    // build a simple regex
    let r = regex!("sa+$");
    assert_eq!(r.is_match("Saa"), false);

    // build a regex with flag(s)
    let r = regex!("sa+b?$"i);
    assert_eq!(r.is_match("Saa"), true);

    // you can use a raw literal
    let r = regex!(r#"^"+$"#);
    assert_eq!(r.is_match("\"\""), true);

    // and a raw literal with flag(s)
    let r = regex!(r#"^\s*("[a-t]*"\s*)+$"#i);
    assert_eq!(r.is_match(r#" "Aristote" "Platon" "#), true);

    // this line wouldn't compile:
    // let r = regex!("(unclosed");

}

fn example_is_match() {
    let b = regex_is_match!("[ab]+", "car");
    assert_eq!(b, true);
}

fn example_using_shared_static() {
    let b = SHARED.is_match("not test");
    assert_eq!(b, false);
}

fn example_captures() {
    let (whole, name, version) = regex_captures!(
        r#"(\w+)-([0-9.]+)"#, // a literal regex
        "This is lazy_regex-2.0!", // any expression
    ).unwrap();
    assert_eq!(whole, "lazy_regex-2.0");
    assert_eq!(name, "lazy_regex");
    assert_eq!(version, "2.0");
}

fn examples_replace_all() {
    let text = "Foo fuu";
    let text = regex_replace_all!(
        r#"\bf(?P<suffix>\w+)"#i,
        text,
        |_, suffix| format!("F<{}>", suffix),
    );
    assert_eq!(text, "F<oo> F<uu>");

    let text = "A = 5 + 3 and B=27+4";
    let text = regex_replace_all!(
        r#"(?x)
            (\d+)
            \s*
            \+
            \s*
            (\d+)
        "#,
        text,
        |_, a: &str, b: &str| {
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();
            (a+b).to_string()
        },
    );
    assert_eq!(text, "A = 8 and B=31");
}

fn main() {

    // the regular expressions will be built only once
    for _ in 0..10 {
        example_builds();
    }

    example_is_match();

    for _ in 0..10 {
        example_captures();
        example_using_shared_static();
        examples_replace_all();
    }


}
