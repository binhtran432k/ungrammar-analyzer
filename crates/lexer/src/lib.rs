//! Low-level Ungrammar lexer.
//!
//! The purpose of this crate is to convert raw sources into a labeled sequence
//! of well-known token types, so building an actual Ungrammar token stream will
//! be easier.
//!
//! The main entity of this crate is the [`TokenKind`] enum which represents common
//! lexeme types.

mod cursor;
pub mod unescape;

#[cfg(test)]
mod tests;

pub use crate::cursor::Cursor;

/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Multi-char tokens:
    /// `// comment`
    LineComment,

    /// Any whitespace character sequence.
    Whitespace,

    /// Token `\r` is invalid to make parsing easier
    InvalidLineEnding,

    Ident,

    Str {
        terminated: bool,
    },

    // One-char tokens:
    /// `=`
    Eq,
    /// `*`
    Star,
    /// `|`
    Or,
    /// `?`
    Question,
    /// `:`
    Colon,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,

    /// Unknown token, not expected by the lexer, e.g. `№`
    Unknown,

    /// End of input.
    Eof,
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

fn is_escapable(c: char) -> bool {
    matches!(c, '\\' | '\'')
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n')
}

fn is_ident_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    pub fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, 0),
        };
        let token_kind = match first_char {
            // Slash, comment or block comment.
            '/' => match self.first() {
                '/' => self.line_comment(),
                _ => TokenKind::Unknown,
            },

            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // Identifier (this should be checked after other variant that can
            // start as identifier).
            c if is_ident_char(c) => self.ident(),

            // One-symbol tokens.
            '=' => TokenKind::Eq,
            '*' => TokenKind::Star,
            '|' => TokenKind::Or,
            '?' => TokenKind::Question,
            ':' => TokenKind::Colon,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,

            // String literal.
            '\'' => TokenKind::Str { terminated: self.quoted_string() },
            '\r' => TokenKind::InvalidLineEnding,
            _ => TokenKind::Unknown,
        };
        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
    }

    fn line_comment(&mut self) -> TokenKind {
        debug_assert!(self.prev() == '/' && self.first() == '/');
        self.bump();

        self.eat_while(|c| c != '\n');
        TokenKind::LineComment
    }

    fn whitespace(&mut self) -> TokenKind {
        debug_assert!(is_whitespace(self.prev()));
        self.eat_while(is_whitespace);
        TokenKind::Whitespace
    }

    fn ident(&mut self) -> TokenKind {
        debug_assert!(is_ident_char(self.prev()));
        // Start is already eaten, eat the rest of identifier.
        self.eat_while(is_ident_char);
        TokenKind::Ident
    }

    /// Eats quoted string and returns true
    /// if string is terminated.
    fn quoted_string(&mut self) -> bool {
        debug_assert!(self.prev() == '\'');
        while let Some(c) = self.bump() {
            match c {
                '\'' => {
                    return true;
                }
                '\\' if is_escapable(self.first()) => {
                    // Bump again to skip escaped character.
                    self.bump();
                }
                _ => (),
            }
        }
        // End of file reached.
        false
    }
}