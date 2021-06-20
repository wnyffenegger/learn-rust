use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    OK = 200,
    BAD_REQUEST = 400,
    NOT_FOUND = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "Ok",
            Self::BAD_REQUEST => "Bad Request",
            Self::NOT_FOUND => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
