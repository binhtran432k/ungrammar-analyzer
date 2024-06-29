//! Lexing `&str` into a sequence of Ungrammar tokens.
//!
//! Note that strictly speaking the parser in this crate is not required to work
//! on tokens which originated from text. Macros, eg, can synthesize tokens out
//! of thin air. So, ideally, lexer should be an orthogonal crate. It is however
//! convenient to include a text-based lexer here!
//!
//! Note that these tokens, unlike the tokens we feed into the parser, do
//! include info about comments and whitespace.

use std::ops;

use lexer::unescape::EscapeError;

use crate::{
    SyntaxKind::{self, *},
    T,
};

pub struct LexedStr<'a> {
    text: &'a str,
    kind: Vec<SyntaxKind>,
    start: Vec<u32>,
    error: Vec<LexError>,
}

struct LexError {
    msg: String,
    token: u32,
}

impl<'a> LexedStr<'a> {
    pub fn new(text: &'a str) -> LexedStr<'a> {
        let _p = tracing::span!(tracing::Level::INFO, "LexedStr::new").entered();
        let mut conv = Converter::new(text);
        for token in lexer::tokenize(&text[conv.offset..]) {
            let token_text = &text[conv.offset..][..token.len as usize];

            conv.extend_token(&token.kind, token_text);
        }

        conv.finalize_with_eof()
    }

    pub fn as_str(&self) -> &str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.kind.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.kind[i]
    }

    pub fn text(&self, i: usize) -> &str {
        self.range_text(i..i + 1)
    }

    pub fn range_text(&self, r: ops::Range<usize>) -> &str {
        assert!(r.start < r.end && r.end <= self.len());
        let lo = self.start[r.start] as usize;
        let hi = self.start[r.end] as usize;
        &self.text[lo..hi]
    }

    // Naming is hard.
    pub fn text_range(&self, i: usize) -> ops::Range<usize> {
        assert!(i < self.len());
        let lo = self.start[i] as usize;
        let hi = self.start[i + 1] as usize;
        lo..hi
    }
    pub fn text_start(&self, i: usize) -> usize {
        assert!(i <= self.len());
        self.start[i] as usize
    }
    pub fn text_len(&self, i: usize) -> usize {
        assert!(i < self.len());
        let r = self.text_range(i);
        r.end - r.start
    }

    pub fn error(&self, i: usize) -> Option<&str> {
        assert!(i < self.len());
        let err = self.error.binary_search_by_key(&(i as u32), |i| i.token).ok()?;
        Some(self.error[err].msg.as_str())
    }

    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> + '_ {
        self.error.iter().map(|it| (it.token as usize, it.msg.as_str()))
    }

    fn push(&mut self, kind: SyntaxKind, offset: usize) {
        self.kind.push(kind);
        self.start.push(offset as u32);
    }
}

struct Converter<'a> {
    res: LexedStr<'a>,
    offset: usize,
}

impl<'a> Converter<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            res: LexedStr { text, kind: Vec::new(), start: Vec::new(), error: Vec::new() },
            offset: 0,
        }
    }

    fn finalize_with_eof(mut self) -> LexedStr<'a> {
        self.res.push(EOF, self.offset);
        self.res
    }

    fn push(&mut self, kind: SyntaxKind, len: usize, err: Option<&str>) {
        self.res.push(kind, self.offset);
        self.offset += len;

        if let Some(err) = err {
            let token = self.res.len() as u32;
            let msg = err.to_owned();
            self.res.error.push(LexError { msg, token });
        }
    }

    fn extend_token(&mut self, kind: &lexer::TokenKind, token_text: &str) {
        // A note on an intended tradeoff:
        // We drop some useful information here (see patterns with double dots `..`)
        // Storing that info in `SyntaxKind` is not possible due to its layout requirements of
        // being `u16` that come from `rowan::SyntaxKind`.
        let mut err = "";

        let syntax_kind = match kind {
            lexer::TokenKind::LineComment => COMMENT,
            lexer::TokenKind::Whitespace => WHITESPACE,
            lexer::TokenKind::InvalidLineEnding => {
                err = "unexpected `\\r`, only Unix-style line endings allowed";
                WHITESPACE
            }
            lexer::TokenKind::Ident => IDENT,
            lexer::TokenKind::Str { terminated } => {
                let len = token_text.len();
                if !terminated {
                    err = "missing trailing `\'` symbol to terminate the token literal";
                } else {
                    let text = &self.res.text[self.offset + 1..][..len - 1];
                    let i = text.rfind('\'').unwrap();
                    let text = &text[..i];
                    err = unescape_string_error_message(text);
                }
                STRING
            }
            lexer::TokenKind::Eq => T![=],
            lexer::TokenKind::Star => T![*],
            lexer::TokenKind::Or => T![|],
            lexer::TokenKind::Question => T![?],
            lexer::TokenKind::Colon => T![:],
            lexer::TokenKind::OpenParen => T!['('],
            lexer::TokenKind::CloseParen => T![')'],
            lexer::TokenKind::Unknown => ERROR,
            lexer::TokenKind::Eof => EOF,
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, token_text.len(), err);
    }
}

fn error_to_diagnostic_message(error: EscapeError) -> &'static str {
    match error {
        EscapeError::ZeroChars => "empty token literal",
        EscapeError::LoneSlash => "",
        EscapeError::InvalidEscape => "unknown character escape",
        EscapeError::BareCarriageReturn => "",
        EscapeError::EscapeOnlyChar => "character constant must be escaped",
    }
}

fn unescape_string_error_message(text: &str) -> &'static str {
    let mut error_message = "";
    lexer::unescape::unescape::<_, char>(text, &mut |_, res| {
        if let Err(e) = res {
            error_message = error_to_diagnostic_message(e);
        }
    });
    error_message
}
