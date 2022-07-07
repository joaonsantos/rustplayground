use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    MethodNotAllowed = 405,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       write!(f, "{} {}", *self as u16, self.reason_phrase())
    }
}