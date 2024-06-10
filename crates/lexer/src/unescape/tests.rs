use super::*;

fn check(literal_text: &str, expected: &str) {
    let mut buf = Ok(String::with_capacity(literal_text.len()));
    unescape(literal_text, &mut |range, c| {
        if let Ok(b) = &mut buf {
            match c {
                Ok(c) => b.push(c),
                Err(e) => buf = Err((range, e)),
            }
        }
    });
    assert_eq!(buf.as_deref(), Ok(expected))
}

fn check_error(literal_text: &str, expected: EscapeError) {
    let mut buf = Ok(String::with_capacity(literal_text.len()));
    unescape(literal_text, &mut |_range, c| {
        if let Ok(b) = &mut buf {
            match c {
                Ok(c) => b.push(c),
                Err(e) => buf = Err(e),
            }
        }
    });
    assert_eq!(buf.as_deref(), Err(&expected))
}

#[test]
fn test_unescape_char_bad() {
    check_error("", EscapeError::ZeroChars);
    check_error(r"\", EscapeError::LoneSlash);

    check_error("'", EscapeError::EscapeOnlyChar);
    check_error("\r", EscapeError::BareCarriageReturn);

    check_error(r"\v", EscapeError::InvalidEscape);
    check_error(r"\n", EscapeError::InvalidEscape);
    check_error(r"\💩", EscapeError::InvalidEscape);
    check_error(r"\●", EscapeError::InvalidEscape);
    check_error("\\\r", EscapeError::InvalidEscape);
}

#[test]
fn test_unescape_char_good() {
    check("a", "a");
    check("ы", "ы");
    check("🦀", "🦀");

    check(r"\\", "\\");
    check(r"\'", "\'");
}

#[test]
fn test_unescape_str_good() {
    check("foo", "foo");
    check(" \t\n", " \t\n");

    check("thread\"s", "thread\"s")
}
