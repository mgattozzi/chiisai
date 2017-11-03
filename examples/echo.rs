#[macro_use] extern crate chiisai;
extern crate hyper;
extern crate futures;

// Imports traits and the rexported hyper and futures crates
use chiisai::*;
use futures::future::ok;
use hyper::header::ContentLength;


static INDEX: &'static [u8] = b"Try POST /echo\n";

fn main() {
    let server = Chiisai::new()
                         .routes(router! {
                            ("/", GetEcho)
                            ("/echo", GetEcho)
                            ("/echo", PostEcho)
                         });
    server.run().unwrap();
}


routes!(
    // Each route handler needs 3 things:
    // 1) Takes a request verb needed for routes that use this:
    //    Post, Put, Patch, Get, or Delete
    // 2) A name for the handler type, in this case PostEcho
    // 3) A closures. Closures take a hyper::server::Request type and returns a
    //    futures::future::FutureResult<hyper::server::Response, hyper::Error>;
    //    These types are automatically imported in the routes macro (except for
    //    hyper::Error) to reduce what things you need to import
    (Post, PostEcho, |req: Request| {
        let mut res = Response::new();
        if let Some(len) = req.headers().get::<ContentLength>() {
            res.headers_mut().set(len.clone());
        }
        Box::new(ok(res.with_body(req.body())))
    })

    (Get, GetEcho, |_| {
        Box::new(ok(Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX)))

    })
);
