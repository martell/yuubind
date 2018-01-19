use std::{fmt, str};
use nom;
use nom::Needed;

#[derive(Fail, Debug, Clone)]
pub enum ParseError {
    DidNotConsumeEverything(usize),
    ParseError(#[cause] nom::Err),
    IncompleteString(Needed),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseError::*;
        match self {
            &DidNotConsumeEverything(rem) =>
                write!(f, "Input contains {} trailing characters", rem),
            &ParseError(ref err) =>
                write!(f, "Parse error: {}", err), // TODO: make error display nicer with nom
            &IncompleteString(Needed::Unknown) =>
                write!(f, "Input appears to be incomplete"),
            &IncompleteString(Needed::Size(sz)) =>
                write!(f, "Input appears to be missing {} characters", sz),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct DbgBytes<'a>(&'a [u8]);

impl<'a> fmt::Debug for DbgBytes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b\"{}\"",
               self.0.iter().flat_map(|x| char::from(*x).escape_default()).collect::<String>())
    }
}

pub fn bytes_to_dbg(b: &[u8]) -> DbgBytes {
    DbgBytes(b)
}
