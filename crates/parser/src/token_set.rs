//! A bit-set of `SyntaxKind`s.

use crate::SyntaxKind;

const MAX_TOKEN_SET: usize = 1;

/// A bit-set of `SyntaxKind`s
#[derive(Clone, Copy)]
pub(crate) struct TokenSet([u64; MAX_TOKEN_SET]);

/// `TokenSet`s should only include token `SyntaxKind`s, so the discriminant of any passed/included
/// `SyntaxKind` must *not* be greater than that of the last token `SyntaxKind`.
/// See #17037.
const LAST_TOKEN_KIND_DISCRIMINANT: usize = SyntaxKind::__FIRST as usize - 1;

impl TokenSet {
    pub(crate) const EMPTY: TokenSet = TokenSet([0; MAX_TOKEN_SET]);

    pub(crate) const fn new(kinds: &[SyntaxKind]) -> TokenSet {
        let mut res = [0; MAX_TOKEN_SET];
        let mut i = 0;
        while i < kinds.len() {
            let discriminant = kinds[i] as usize;
            debug_assert!(
                discriminant <= LAST_TOKEN_KIND_DISCRIMINANT,
                "Expected a token `SyntaxKind`"
            );
            let idx = discriminant / 64;
            res[idx] |= 1 << (discriminant % 64);
            i += 1;
        }
        TokenSet(res)
    }

    pub(crate) const fn union(self, other: TokenSet) -> TokenSet {
        TokenSet([self.0[0] | other.0[0]])
    }

    pub(crate) const fn contains(&self, kind: SyntaxKind) -> bool {
        let discriminant = kind as usize;
        debug_assert!(
            discriminant <= LAST_TOKEN_KIND_DISCRIMINANT,
            "Expected a token `SyntaxKind`"
        );
        let idx = discriminant / 64;
        let mask = 1 << (discriminant % 64);
        self.0[idx] & mask != 0
    }
}

#[test]
fn token_set_works_for_tokens() {
    use crate::SyntaxKind::*;
    let ts = TokenSet::new(&[EOF, COMMENT]);
    assert!(ts.contains(EOF));
    assert!(ts.contains(COMMENT));
    assert!(!ts.contains(STAR));
}
