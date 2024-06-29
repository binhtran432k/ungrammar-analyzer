use super::*;

// test rule_tokens
// Hello = 'Hello'
pub(crate) const TOKEN_FIRST: TokenSet = TokenSet::new(&[STRING]);

pub(crate) fn token(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at_ts(TOKEN_FIRST) {
        return None;
    }
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, TOKEN))
}

pub(super) const ATOM_RULE_FIRST: TokenSet = TOKEN_FIRST.union(TokenSet::new(&[T!['('], IDENT]));

pub(super) const RULE_RECOVERY_SET: TokenSet = TokenSet::new(&[T![')'], T![=]]);

pub(super) fn atom_rule(p: &mut Parser) -> Option<CompletedMarker> {
    if let Some(m) = token(p) {
        return Some(m);
    }
    let la = p.nth(1);
    let done = match p.current() {
        IDENT if la == T![:] => labeled_rule(p),
        IDENT => name_ref(p)?,
        T!['('] => paren_rule(p),
        _ => {
            p.err_and_bump("expected rule");
            return None;
        }
    };
    Some(done)
}

// test paren_rule
// SourceFile = ('Hello')
fn paren_rule(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(T!['(']));
    let m = p.start();
    p.expect(T!['(']);
    rule(p);
    p.expect(T![')']);
    m.complete(p, PAREN_RULE)
}

// test labeled_rule
// SourceFile = name:'World'
fn labeled_rule(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(IDENT) && p.nth_at(1, T![:]));
    let m = p.start();
    label(p);
    p.bump(T![:]);
    rule(p);
    m.complete(p, LABELED_RULE)
}

fn label(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(IDENT));
    let m = p.start();
    p.bump(IDENT);
    m.complete(p, LABEL)
}
