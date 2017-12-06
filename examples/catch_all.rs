#[macro_use] extern crate chiisai;
extern crate hyper;
extern crate futures;

// Imports traits and the rexported hyper and futures crates
use chiisai::*;
use futures::future::ok;


fn main() {
    let server = Chiisai::new()
                         .routes(router! {
                            ("/*", CatchAll)
                            ("/", GetRoot)
                            ("/test", GetTest)
                         });
    server.run().unwrap();
}


routes!(
    (Get, GetRoot, |req: Request| {
        println!("GetRoot: {:?}", req);
        Box::new(ok(Response::new()))
    })

    (Get, GetTest, |req: Request| {
        println!("GetTest: {:?}", req);
        Box::new(ok(Response::new()))
    })

    // It doesn't matter what you put for the method here or name the
    // route it'll work on all types of requests The magic that sets
    // this route as a catch all is putting "/*" in the router!()
    // macro above
    (Get, CatchAll, |req: Request| {
        println!("CatchAll: {:?}", req);
        Box::new(ok(Response::new()))
    })

);
