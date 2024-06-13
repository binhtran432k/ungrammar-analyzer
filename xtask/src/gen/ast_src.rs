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
    tokens: &["ERROR", "IDENT", "STR", "WHITESPACE", "COMMENT"],
    nodes: &["SOURCE_FILE", "NODE", "TOKEN"],
};
