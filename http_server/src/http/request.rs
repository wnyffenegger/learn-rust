use super::method::Method;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(byte_array: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(byte_array)?;

        match get_next_word(request) {
            Some((method, request)) => {}
            None => return Err(ParseError::InvalidRequest),
        }

        // request is actually a brand new variable
        // called variable shadowing
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let char_iterator = request.chars();

    for (i, c) in request.chars().enumerate() {
        // The i + 1 is okay because we know that we are skipping
        // the space character which is only one byte. Otherwise we would need
        // to get the number of bytes of c
        if c == ' ' {
            return Some((&request[..i], &request[i + 1..]));
        }

        if c == '\r' {
            return Some((&request[..i], &request[i + 2..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        return ParseError::InvalidEncoding;
    }
}
