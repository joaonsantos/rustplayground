use std::collections::HashMap;

use super::{HandlerFunc, Request, Response, StatusCode};


pub struct Router {
    routes: HashMap<String, HandlerFunc>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: HashMap::new() }
    }

    pub fn register<H>(&mut self, url: &str, func: H) 
    where H: Fn(super::Request) -> super::Response + 'static
    {
        self.routes.insert(url.to_string(), Box::new(func));
    }

    pub fn handle_request(&self, req: Request) -> Response {
        match self.routes.get(req.path) {
            Some(func) => func(req),
            None => Response::new(StatusCode::NotFound, None)
        }
    }
}