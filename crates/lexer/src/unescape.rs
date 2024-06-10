//! Utilities for validating string and char literals and turning them into
//! values they represent.

use std::ops::Range;
use std::str::Chars;

#[cfg(test)]
mod tests;

/// Errors and warnings that can occur during string unescaping. They mostly
/// relate to malformed escape sequences, but there are a few that are about
/// other problems.
#[derive(Debug, PartialEq, Eq)]
pub enum EscapeError {
    /// Expected 1 or more chars, but 0 were found.
    ZeroChars,
    /// Escaped '\' character without continuation.
    LoneSlash,
    /// Invalid escape character (e.g. '\z').
    InvalidEscape,
    /// Raw '\r' encountered.
    BareCarriageReturn,
    /// Unescaped character that was expected to be escaped (e.g. raw '\t').
    EscapeOnlyChar,
}

/// Takes a contents of a string literal (without quotes) and produces a
/// sequence of escaped characters or errors.
pub fn unescape<F, T: From<char> + From<u8>>(src: &str, callback: &mut F)
where
    F: FnMut(Range<usize>, Result<T, EscapeError>),
{
    if src.is_empty() {
        callback(0..0, Err(EscapeError::ZeroChars));
    }
    let mut chars = src.chars();

    // The `start` and `end` computation here is complicated because
    // `skip_ascii_whitespace` makes us to skip over chars without counting
    // them in the range computation.
    while let Some(c) = chars.next() {
        let start = src.len() - chars.as_str().len() - c.len_utf8();
        let res = match c {
            '\\' => scan_escape::<T>(&mut chars),
            '\'' => Err(EscapeError::EscapeOnlyChar),
            '\r' => Err(EscapeError::BareCarriageReturn),
            _ => Ok(T::from(c)),
        };
        let end = src.len() - chars.as_str().len();
        callback(start..end, res);
    }
}

fn scan_escape<T: From<char> + From<u8>>(chars: &mut Chars<'_>) -> Result<T, EscapeError> {
    // Previous character was '\\', unescape what follows.
    let res: char = match chars.next().ok_or(EscapeError::LoneSlash)? {
        '\\' => '\\',
        '\'' => '\'',
        _ => return Err(EscapeError::InvalidEscape),
    };
    Ok(T::from(res))
}
