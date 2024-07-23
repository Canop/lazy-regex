use {
    lazy_regex::regex_switch,
    std::num::ParseIntError,
};

#[test]
fn test_regex_switch() {
    #[derive(Debug, PartialEq, Eq)]
    enum Color {
        Grey(u8),
        Pink,
        Rgb(u8, u8, u8),
    }
    fn read(s: &str) -> Option<Color> {
        regex_switch!(s,
            r#"^gr(a|e)y\((?<level>\d{1,2})\)$"#i => {
                Color::Grey(level.parse().unwrap())
            }
            "^pink"i => Color::Pink,
            r#"^rgb\((?<r>\d+),\s*(?<g>\d+),\s*(?<b>\d+),?\)$"#i => Color::Rgb (
                r.parse().unwrap(),
                g.parse().unwrap(),
                b.parse().unwrap(),
            ),
        )
    }
    assert_eq!(read("gray(15)"), Some(Color::Grey(15)));
    assert_eq!(read("pInk"), Some(Color::Pink));
    assert_eq!(read("pinkie"), Some(Color::Pink));
    assert_eq!(read("red"), None);
    assert_eq!(read("rgb(1,2,3)"), Some(Color::Rgb(1, 2, 3)));
}

#[test]
fn test_regex_switch_with_error_handling() -> Result<(), ParseIntError> {
    #[derive(Debug, PartialEq)]
    enum Color {
        Grey(u8),
        Pink,
        Rgb(u8, u8, u8),
    }
    let input = "RGB(1, 2, 3)";
    let color = regex_switch!(input,
        r#"^gr(a|e)y\((?<level>\d{1,2})\)$"#i => {
            Color::Grey(level.parse()?)
        }
        "^pink"i => Color::Pink,
        r#"^rgb\((?<r>\d+),\s*(?<g>\d+),\s*(?<b>\d+),?\)$"#i => Color::Rgb (
            r.parse()?,
            g.parse()?,
            b.parse()?,
        ),
    );
    assert_eq!(color, Some(Color::Rgb(1, 2, 3)));
    Ok(())
}
