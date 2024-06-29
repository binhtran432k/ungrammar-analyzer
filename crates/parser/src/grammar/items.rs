use super::*;

// test grammar_contents
// SourceFile =
//   (Hello name:'World')* '!'?
// Hello =
//   'Hello'
// | 'Hi'
pub(super) fn grammar_contents(p: &mut Parser<'_>) {
    while !p.at(EOF) {
        item(p);
    }
}

pub(super) fn item(p: &mut Parser<'_>) {
    let m = p.start();

    let m = match opt_item(p, m) {
        Ok(()) => return,
        Err(m) => m,
    };

    m.abandon(p);
    match p.current() {
        EOF => p.error("expected an item"),
        _ => p.err_and_bump("expected an item"),
    }
}

/// Try to parse an item, completing `m` in case of success.
fn opt_item(p: &mut Parser<'_>, m: Marker) -> Result<(), Marker> {
    match p.current() {
        IDENT | T![=] => node(p, m),
        _ => return Err(m),
    };
    Ok(())
}

fn node(p: &mut Parser<'_>, m: Marker) {
    // test_err recovery_node_name
    // Hello = 'Hello'
    // = 'World'
    name(p);
    // test_err recovery_node_eq
    // SourceFile
    // SourceFileB = 'Hello'
    p.expect(T![=]);
    // test_err recovery_node_rule
    // SourceFile =
    // Hello = 'Hello'
    rules::rule(p);
    m.complete(p, NODE);
}
