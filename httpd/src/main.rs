use httpd::{server::Server, http::{Router, Response, StatusCode, Method}};



fn main() {
    // server address
    let addr = String::from("127.0.0.1:8080");


    // create a new router
    let mut router = Router::new();
    router.register("/", |req| {
        match req.method {
            Method::GET => Response::new(StatusCode::Ok, Some(String::from("<h1>Hello world!</h1>"))),
            _ => Response::new(StatusCode::MethodNotAllowed, None)
        }
    });

    // server gets the router
    let server = Server::new(addr, router);
    server.run()
}