//! Defines input for code generation process.

pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc<'_> = KindsSrc {
    punct: &[
        ("=", "EQ"),
        ("*", "STAR"),
        ("|", "PIPE"),
        ("?", "QUESTION"),
        (":", "COLON"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
    ],
    tokens: &["ERROR", "IDENT", "STRING", "WHITESPACE", "COMMENT"],
    #[rustfmt::skip]
    nodes: &[
        "GRAMMAR",
        "NODE",
        // atoms
        "LABEL",
        "PAREN_RULE",
        // unary
        "NAME",
        "NAME_REF",
        "TOKEN",
        "SEQ_RULE",
        "LABELED_RULE",
        "ALT_RULE",
        "OPT_RULE",
        "REP_RULE",
    ],
};
