extern crate futures;
extern crate hyper;
extern crate chiisai;

use futures::future::{ ok, FutureResult };
use hyper::{Get, Post};
use hyper::header::ContentLength;
use hyper::server::{ Request, Response };
use chiisai::{ Chiisai, Route };

static INDEX: &'static [u8] = b"Try POST /echo\n";

fn main() {
    let server = Chiisai::new()
                         .route(Get, "/", Box::new(GetEcho))
                         .route(Get, "/echo", Box::new(GetEcho))
                         .route(Post, "/echo", Box::new(PostEcho));
    server.run().unwrap();
}

struct PostEcho;
struct GetEcho;

impl Route for PostEcho {
    fn handler(&self, req: Request) -> FutureResult<Response, hyper::Error> {
        let mut res = Response::new();
        if let Some(len) = req.headers().get::<ContentLength>() {
            res.headers_mut().set(len.clone());
        }
        ok(res.with_body(req.body()))
    }
}

impl Route for GetEcho {
    fn handler(&self, _req: Request) -> FutureResult<Response, hyper::Error> {
        ok(Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX))
    }
}
