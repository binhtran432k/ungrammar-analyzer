//! Defines input for code generation process.

pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) contextual_keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc<'_> = KindsSrc {
    punct: &[
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("**", "STAR2"),
        ("/", "SLASH"),
        ("=", "EQ"),
        ("==", "EQ2"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("<>", "LTGT"),
        ("*>", "STAR_GT"),
        (">>", "GTGT"),
        ("$", "DOLLAR"),
        (",", "COMMA"),
        (";", "SEMICOLON"),
        (".", "DOT"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        (":", "COLON"),
    ],
    keywords: &[
    ],
    contextual_keywords: &[
    ],
    literals: &["STRING", "INT_NUMBER", "FLOAT_NUMBER", "HEX_STRING"],
    tokens: &[
        "ERROR",
        "IDENT",
        "NUMBER_IDENT",
        "WHITESPACE",
        "SEQUENCE_NUMBER_AREA",
        "INDICATOR_AREA",
        "PROGRAM_IDENTIFICATION_AREA",
        "COMMENT_AREA",
        "DEBUG_AREA",
        "CONTINUE_LINE_AREA",
        "INVALID_AREA",
        "COMMENT",
        "DEBUG",
    ],
    nodes: &["SOURCE_FILE"],
};