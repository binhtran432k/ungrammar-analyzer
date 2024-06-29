//! The Ungrammar parser.
//!
//! The parser doesn't know about concrete representation of tokens and syntax
//! trees. Abstract [`TokenSource`] and [`TreeSink`] traits are used instead. As
//! a consequence, this crate does not contain a lexer.
//!
//! The [`Parser`] struct from the [`parser`] module is a cursor into the
//! sequence of tokens.  Parsing routines use [`Parser`] to inspect current
//! state and advance the parsing.
//!
//! The actual parsing happens in the [`grammar`] module.
//!
//! Tests for this crate live in the `syntax` crate.
//!
//! [`Parser`]: crate::parser::Parser

extern crate lexer;

mod edition;
mod event;
mod grammar;
mod input;
mod lexed_str;
mod output;
mod parser;
mod shortcuts;
mod syntax_kind;
mod token_set;

#[cfg(test)]
mod tests;

pub(crate) use token_set::TokenSet;

pub use crate::{
    edition::Edition,
    input::Input,
    lexed_str::LexedStr,
    output::{Output, Step},
    shortcuts::StrStep,
    syntax_kind::SyntaxKind,
};

/// Parse the whole of the input as a given syntactic construct.
///
/// This covers two main use-cases:
///
///   * Parsing a Ungrammar file.
///
/// [`TopEntryPoint::parse`] makes a guarantee that
///   * all input is consumed
///   * the result is a valid tree (there's one root node)
#[derive(Debug)]
pub enum TopEntryPoint {
    Grammar,
}

impl TopEntryPoint {
    pub fn parse(&self, input: &Input, edition: Edition) -> Output {
        let _p = tracing::span!(tracing::Level::INFO, "TopEntryPoint::parse", ?self).entered();
        let entry_point: fn(&'_ mut parser::Parser<'_>) = match self {
            TopEntryPoint::Grammar => grammar::entry::top::grammar,
        };
        let mut p = parser::Parser::new(input, edition);
        entry_point(&mut p);
        let events = p.finish();
        let res = event::process(events);

        if cfg!(debug_assertions) {
            let mut depth = 0;
            let mut first = true;
            for step in res.iter() {
                assert!(depth > 0 || first);
                first = false;
                match step {
                    Step::Enter { .. } => depth += 1,
                    Step::Exit => depth -= 1,
                    Step::Token { .. } | Step::Error { .. } => (),
                }
            }
            assert!(!first, "no tree at all");
            assert_eq!(depth, 0, "unbalanced tree");
        }

        res
    }
}
