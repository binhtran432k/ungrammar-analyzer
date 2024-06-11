use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};
use sourcegen::{add_preamble, ensure_file_contents, project_root, reformat};

use super::ast_src::{KindsSrc, KINDS_SRC};

pub fn generate() {
    let syntax_kinds = generate_syntax_kinds(KINDS_SRC);
    let syntax_kinds_file = project_root().join("crates/parser/src/syntax_kind/generated.rs");
    ensure_file_contents(syntax_kinds_file.as_path(), &syntax_kinds);
}

fn generate_syntax_kinds(grammar: KindsSrc<'_>) -> String {
    let (single_byte_tokens_values, single_byte_tokens): (Vec<_>, Vec<_>) = grammar
        .punct
        .iter()
        .filter(|(token, _name)| token.len() == 1)
        .map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
        .unzip();

    let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
        if "{}[]()".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation =
        grammar.punct.iter().map(|(_token, name)| format_ident!("{}", name)).collect::<Vec<_>>();

    let tokens = grammar.tokens.iter().map(|name| format_ident!("{}", name)).collect::<Vec<_>>();

    let nodes = grammar.nodes.iter().map(|name| format_ident!("{}", name)).collect::<Vec<_>>();

    let ast = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        /// The kind of syntax node, e.g. `NODE`, `TOKEN`, or `STAR`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum SyntaxKind {
            // Technical SyntaxKinds: they appear temporally during parsing,
            // but never end up in the final tree
            #[doc(hidden)]
            TOMBSTONE,
            #[doc(hidden)]
            EOF,
            #(#punctuation,)*
            #(#tokens,)*
            // Technical kind so that we can mark the end of tokens
            #[doc(hidden)]
            __FIRST,
            #(#nodes,)*

            // Technical kind so that we can cast from u16 safely
            #[doc(hidden)]
            __LAST,
        }
        use self::SyntaxKind::*;

        impl SyntaxKind {
            pub fn is_punct(self) -> bool {
                matches!(self, #(#punctuation)|*)
            }

            pub fn from_char(c: char) -> Option<SyntaxKind> {
                let tok = match c {
                    #(#single_byte_tokens_values => #single_byte_tokens,)*
                    _ => return None,
                };
                Some(tok)
            }
        }

        #[macro_export]
        macro_rules! T {
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
        }
    };

    add_preamble("sourcegen_ast", reformat(ast.to_string()))
}
