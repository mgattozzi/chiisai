#[macro_use] extern crate chiisai;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

// Imports traits and the rexported hyper and futures crates
use chiisai::*;
use futures::future::ok;

fn main() {
    let server = Chiisai::new()
                         .routes(router! {
                            ("/", PostJson)
                         });
    server.run().unwrap();
}

#[derive(Debug, Serialize)]
struct ResponseBody {
    id: u8,
    name: String,
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
    (Post, PostJson, |_req: Request| {
        use hyper::header::{Headers, ContentType};

        let res = Response::new();

        let mut headers = Headers::new();
        headers.set(ContentType::json());

        let response_body = serde_json::to_string(&ResponseBody { id: 20, name: String::from("It works!\n") }).unwrap();
        ok(res.with_headers(headers).with_body(response_body))
    })
);
