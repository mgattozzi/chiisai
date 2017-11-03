#[macro_use] extern crate chiisai;
extern crate hyper;
extern crate futures;

// Imports traits and the rexported hyper and futures crates
use chiisai::*;
use futures::future::ok;


fn main() {
    let server = Chiisai::new()
                            // We define routes here with parameters here.
                            // If the section of the url you want to be a
                            // parameter starts with : code inside chiisai
                            // will handle it properly to match.
                         .routes(router! {
                            ("/test/:param/test2", Parameter)
                            ("/test/:param/test2", PostParameter)
                            ("/test/:user/test2", PostParameter)
                         });
    server.run().unwrap();
}


routes!(
    (Get, Parameter, |req: Request| {
        println!("{:?}", req);
        Box::new(ok(Response::new()))
    })
    (Post, PostParameter, |req: Request| {
        println!("{:?}", req);
        Box::new(ok(Response::new()))
    })
);
