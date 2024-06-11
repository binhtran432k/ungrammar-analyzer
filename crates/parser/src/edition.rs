//! The edition of the Rust language used in a crate.
// Ideally this would be defined in the span crate, but the dependency chain is all over the place
// wrt to span, parser and syntax.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Edition {
    V1,
}

impl Edition {
    pub const CURRENT: Edition = Edition::V1;
    pub const DEFAULT: Edition = Edition::V1;
}

#[derive(Debug)]
pub struct ParseEditionError {
    invalid_input: String,
}

impl std::error::Error for ParseEditionError {}
impl fmt::Display for ParseEditionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid edition: {:?}", self.invalid_input)
    }
}

impl std::str::FromStr for Edition {
    type Err = ParseEditionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "1" => Edition::V1,
            _ => return Err(ParseEditionError { invalid_input: s.to_owned() }),
        };
        Ok(res)
    }
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Edition::V1 => "1",
        })
    }
}
