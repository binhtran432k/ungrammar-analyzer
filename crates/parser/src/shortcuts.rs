//! Shortcuts that span lexer/parser abstraction.
//!
//! The way Ungrammar works, parser doesn't necessary parse text, and you might
//! tokenize text without parsing it further. So, it makes sense to keep
//! abstract token parsing, and string tokenization as completely separate
//! layers.
//!
//! However, often you do parse text into syntax trees and the glue code for
//! that needs to live somewhere. Rather than putting it to lexer or parser, we
//! use a separate shortcuts module for that.

use std::mem;

use crate::{LexedStr, Step, SyntaxKind};

#[derive(Debug)]
pub enum StrStep<'a> {
    Token { kind: SyntaxKind, text: &'a str },
    Enter { kind: SyntaxKind },
    Exit,
    Error { msg: &'a str, pos: usize },
}

impl LexedStr<'_> {
    pub fn to_input(&self) -> crate::Input {
        let _p = tracing::span!(tracing::Level::INFO, "LexedStr::to_input").entered();
        let mut res = crate::Input::default();
        let mut was_joint = false;
        for i in 0..self.len() {
            let kind = self.kind(i);
            if kind.is_trivia() {
                was_joint = false
            } else {
                if was_joint {
                    res.was_joint();
                }
                res.push(kind);
                was_joint = true;
            }
        }
        res
    }

    /// NB: only valid to call with Output from Reparser/TopLevelEntry.
    pub fn intersperse_trivia(
        &self,
        output: &crate::Output,
        sink: &mut dyn FnMut(StrStep<'_>),
    ) -> bool {
        let mut builder = Builder { lexed: self, pos: 0, state: State::PendingEnter, sink };

        for event in output.iter() {
            match event {
                Step::Token { kind, n_input_tokens: n_raw_tokens } => {
                    builder.token(kind, n_raw_tokens)
                }
                Step::Enter { kind } => builder.enter(kind),
                Step::Exit => builder.exit(),
                Step::Error { msg } => {
                    let text_pos = builder.lexed.text_start(builder.pos);
                    (builder.sink)(StrStep::Error { msg, pos: text_pos });
                }
            }
        }

        match mem::replace(&mut builder.state, State::Normal) {
            State::PendingExit => {
                builder.eat_trivia();
                (builder.sink)(StrStep::Exit);
            }
            State::PendingEnter | State::Normal => unreachable!(),
        }

        // is_eof?
        builder.pos == builder.lexed.len()
    }
}

struct Builder<'a, 'b> {
    lexed: &'a LexedStr<'a>,
    pos: usize,
    state: State,
    sink: &'b mut dyn FnMut(StrStep<'_>),
}

enum State {
    PendingEnter,
    Normal,
    PendingExit,
}

impl Builder<'_, '_> {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
        self.eat_trivia();
        self.do_token(kind, n_tokens as usize);
    }

    fn enter(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => {
                (self.sink)(StrStep::Enter { kind });
                // No need to attach trivia to previous node: there is no
                // previous node.
                return;
            }
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }

        let n_trivia =
            (self.pos..self.lexed.len()).take_while(|&it| self.lexed.kind(it).is_trivia()).count();
        let leading_trivia = self.pos..self.pos + n_trivia;
        let n_attached_trivia = n_attached_trivia(
            kind,
            leading_trivia.rev().map(|it| (self.lexed.kind(it), self.lexed.text(it))),
        );
        self.eat_n_trivia(n_trivia - n_attached_trivia);
        (self.sink)(StrStep::Enter { kind });
        self.eat_n_trivia(n_attached_trivia);
    }

    fn exit(&mut self) {
        match mem::replace(&mut self.state, State::PendingExit) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
    }

    fn eat_trivia(&mut self) {
        while self.pos < self.lexed.len() {
            let kind = self.lexed.kind(self.pos);
            if !kind.is_trivia() {
                break;
            }
            self.do_token(kind, 1);
        }
    }

    fn eat_n_trivia(&mut self, n: usize) {
        for _ in 0..n {
            let kind = self.lexed.kind(self.pos);
            assert!(kind.is_trivia());
            self.do_token(kind, 1);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, n_tokens: usize) {
        let text = &self.lexed.range_text(self.pos..self.pos + n_tokens);
        self.pos += n_tokens;
        (self.sink)(StrStep::Token { kind, text });
    }
}

fn n_attached_trivia<'a>(
    _kind: SyntaxKind,
    _trivia: impl Iterator<Item = (SyntaxKind, &'a str)>,
) -> usize {
    0
}
