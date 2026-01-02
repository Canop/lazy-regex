use {
    super::regex::*,
    std::borrow::Cow,
};

/// Remove the first match of the regex from the text.
///
/// If the removed match is at the start or end of the input,
/// a borrowed slice is returned.
#[must_use]
pub fn remove_match<'s>(
    rex: &Regex,
    text: &'s str,
) -> Cow<'s, str> {
    let Some(m) = rex.find(text) else {
        return Cow::Borrowed(text);
    };
    if m.start() == 0 {
        return Cow::Borrowed(&text[m.end()..]);
    }
    if m.end() == text.len() {
        return Cow::Borrowed(&text[..m.start()]);
    }
    let mut s = String::with_capacity(text.len() - m.len());
    s.push_str(&text[..m.start()]);
    s.push_str(&text[m.end()..]);
    Cow::Owned(s)
}

