use {
    lazy_regex::{
        regex_remove,
    },
};

#[test]
fn test_regex_remove() {
    let input = "154681string63731";

    // no match: borrowed and unchanged
    let output = regex_remove!("[A-Z]+", input);
    assert!(matches!(output, std::borrow::Cow::Borrowed("154681string63731")));

    // removing in the middle (a new string is created)
    let output = regex_remove!("[a-z]+", input);
    assert_eq!(output, "15468163731");

    // removing at ends, no new string is created
    let output = regex_remove!(r"^\d+", input);
    let output = regex_remove!(r"\d+$", &output);
    assert_eq!(output, "string");
    assert!(matches!(output, std::borrow::Cow::Borrowed("string")));
}

