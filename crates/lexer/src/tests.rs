use super::*;

use expect_test::{expect, Expect};
use std::fmt::Write;

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = tokenize(src).fold(String::new(), |mut output, token| {
        let _ = writeln!(output, "{:?}", token);
        output
    });
    expect.assert_eq(&actual)
}

#[test]
fn smoke_test() {
    check_lexing(
        r#"/// ungrammar for HelloWorld
Model =
  (persons:Person | greetings:Greeting)*

Person =
  'person' name:'ident'

Greeting =
  'Hello' person:Person '!'"#,
        expect![[r#"
            Token { kind: LineComment, len: 28 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 3 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Ident, len: 7 }
            Token { kind: Colon, len: 1 }
            Token { kind: Ident, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Or, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 9 }
            Token { kind: Colon, len: 1 }
            Token { kind: Ident, len: 8 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Star, len: 1 }
            Token { kind: Whitespace, len: 2 }
            Token { kind: Ident, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 3 }
            Token { kind: Str { terminated: true }, len: 8 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Colon, len: 1 }
            Token { kind: Str { terminated: true }, len: 7 }
            Token { kind: Whitespace, len: 2 }
            Token { kind: Ident, len: 8 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 3 }
            Token { kind: Str { terminated: true }, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 6 }
            Token { kind: Colon, len: 1 }
            Token { kind: Ident, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Str { terminated: true }, len: 3 }
        "#]],
    )
}
