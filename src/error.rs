use core::fmt;


#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!{f, "failed to parse the protocol tag"}
    }
}

impl std::error::Error for ParseError{}