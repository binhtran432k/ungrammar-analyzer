use super::*;

mod atom;

const RULE_FIRST: TokenSet = atom::ATOM_RULE_FIRST;

// test bp
// SOURCE = 'a'? | 'b'* | 'c'
mod bp {
    pub const NONE: u8 = 0;
    pub const SMALLEST: u8 = 1;
    pub const ALT: u8 = 1;
    pub const SEQ: u8 = 2;
    pub const OPT: u8 = 3;
    pub const REP: u8 = 3;
}

pub(super) fn rule(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    rule_bp(p, None, bp::SMALLEST)
}

enum Associativity {
    Left,
    _Right,
}

/// Binding powers of operators for a Pratt parser.
///
/// See <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html>
///
/// Note that Rust doesn't define associativity for some infix operators (e.g. `==` and `..`) and
/// requires parentheses to disambiguate. We just treat them as left associative.
fn current_op(p: &Parser<'_>) -> (u8, SyntaxKind, Associativity) {
    use Associativity::*;
    const NOT_AN_OP: (u8, SyntaxKind, Associativity) = (bp::NONE, EOF, Left);
    match p.current() {
        // test alt_rule
        // Hello = 'Hello' | 'Hi'
        // HelloB = 'Hello' | 'Hi' | 'Hey'
        T![|] => (bp::ALT, T![|], Left),
        // test rep_rule
        // Hello = 'Hello'*
        // HelloB = 'Hello'**
        T![*] => (bp::REP, T![*], Left),
        // test opt_rule
        // Hello = 'Hello'?
        // HelloB = 'Hello'??
        T![?] => (bp::OPT, T![?], Left),
        _ => NOT_AN_OP,
    }
}

// Parses expression with binding power of at least bp.
fn rule_bp(p: &mut Parser<'_>, m: Option<Marker>, bp: u8) -> Option<CompletedMarker> {
    let m = m.unwrap_or_else(|| p.start());

    if !p.at_ts(RULE_FIRST) {
        p.err_recover("expected rule", atom::RULE_RECOVERY_SET);
        m.abandon(p);
        return None;
    }
    if is_end_of_node(p) {
        p.error("expected rule");
        m.abandon(p);
        return None;
    }
    let mut lhs = match lhs(p) {
        Some(lhs) => lhs.extend_to(p, m),
        None => {
            m.abandon(p);
            return None;
        }
    };

    loop {
        if p.at_ts(RULE_FIRST) && !is_end_of_node(p) {
            let mut count = 0;
            while p.at_ts(RULE_FIRST) && !is_end_of_node(p) {
                if rule_bp(p, None, bp::SEQ).is_none() {
                    break;
                }
                count += 1;
            }
            if count > 0 {
                let m = lhs.precede(p);
                // test seq_rule
                // SourceFile = 'Hello' 'World'
                lhs = m.complete(p, SEQ_RULE);
                continue;
            }
        }

        let (op_bp, op, associativity) = current_op(p);
        if op_bp < bp {
            break;
        }

        let op_kind = match op {
            T![|] => ALT_RULE,
            T![*] => REP_RULE,
            T![?] => OPT_RULE,
            _ => break,
        };

        if p.at(T![?]) || p.at(T![*]) {
            let m = lhs.precede(p);
            p.bump(op);
            lhs = m.complete(p, op_kind);
            continue;
        }

        let m = lhs.precede(p);
        p.bump(op);

        let op_bp = match associativity {
            Associativity::Left => op_bp + 1,
            Associativity::_Right => op_bp,
        };

        // test_err recovery_node_alt
        // SourceFile = 'Hello' |
        // Hello = 'Hello'
        rule_bp(p, None, op_bp);
        lhs = m.complete(p, op_kind)
    }
    Some(lhs)
}

fn is_end_of_node(p: &Parser<'_>) -> bool {
    let la = p.nth(1);
    matches!((p.current(), la), (IDENT, T![=]))
}

fn lhs(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    atom::atom_rule(p)
}
