use super::{Request, Response};

pub type HandlerFunc = Box<dyn Fn(Request) -> Response>;