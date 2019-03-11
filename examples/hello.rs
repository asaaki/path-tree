extern crate futures;
extern crate hyper;
extern crate path_tree;

use futures::Future;
use hyper::server::Server;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, StatusCode};
use path_tree::PathTree;

type Params<'a> = Vec<(&'a str, &'a str)>;

type Handler = fn(Request<Body>, Params) -> Response<Body>;

fn index(_: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("Hello, Web!"))
}

fn hello_world(_: Request<Body>, params: Params) -> Response<Body> {
    let mut s = String::new();
    s.push_str("Hello, World!\n");
    for (_, v) in params {
        s.push_str(&format!("param = {}", v));
    }
    Response::new(Body::from(s))
}

fn hello_user(_: Request<Body>, params: Params) -> Response<Body> {
    let mut s = String::new();
    s.push_str("Hello, ");
    for (k, v) in params {
        s.push_str(&format!("{} = {}", k, v));
    }
    s.push_str("!");
    Response::new(Body::from(s))
}

fn hello_rust(_: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("Hello, Rust!"))
}

fn login(_req: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("I'm logined!"))
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let mut tree: PathTree<Handler> = PathTree::new();
    tree.insert("/GET/", index);
    tree.insert("/GET/*", hello_world);
    tree.insert("/GET/hello/:name", hello_user);
    tree.insert("/GET/rust", hello_rust);
    tree.insert("/POST/login", login);

    let routing = move || {
        let router = tree.clone();

        service_fn_ok(move |req| {
            let path = "/".to_owned() + req.method().as_str() + req.uri().path();

            dbg!(&path);

            match router.find(&path) {
                Some((handler, params)) => handler(req, params),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not Found"))
                    .unwrap(),
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(routing)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}