use super::method::{Method, MethodError};
use std::{str, convert::TryFrom, error::Error, fmt::{Display, Debug, Formatter, Result as FmtResult}, str::{ParseBoolError, Utf8Error}};

pub struct Request<'buf> {
    pub path: &'buf str,
    pub query_str: Option<&'buf str>,
    pub method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET / HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let req = str::from_utf8(&buf)?;
        let (method, req) = get_next_token(req).ok_or(ParseError::InvalidRequest)?;
        let (mut path, req) = get_next_token(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_token(req).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        let mut query_str = None;
        if let Some(i) = path.find('?') {
            query_str = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_str,
            method,
        })
    }
}

fn get_next_token(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEnconding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn error(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEnconding => "Invalid Enconding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEnconding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Error for ParseError {}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.error())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::InvalidRequest => write!(f, "Invalid Request"),
            Self::InvalidEnconding => write!(f, "Invalid Enconding"),
            Self::InvalidProtocol => write!(f, "Invalid Protocol"),
            Self::InvalidMethod => write!(f, "Invalid Method"),
        }
    }
}