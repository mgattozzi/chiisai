# Chiisai

A micro framework for microservices!

## About
Chiisai is a small low level wrapper around Hyper. It aims to take in few if any
dependencies and acts as a basic handler for incoming requests. It isn't meant
to be used for your website, if you need that look into something like Rocket or
Gotham. Instead it's used for microservices, where routing would be useful, but
large web frameworks would take up to many resources.

This means you can still write code for your microservice to do what they need
and not have to worry about setting up all the nuts and bolts for handling
incoming requests or mucking around with low level HTTP internals.

## Build requirements

You only need a stable version of the Rust compiler. Due to the use of the `?`
operator only versions 1.15 and up of `rustc` are supported.

## How to use the library

Put the following in your `Cargo.toml`:

```toml
[dependencies]
chiisai = "0.1"
```

Then import the crate with:

```rust
#[macro_use] extern crate chiisai;
use chiisai::*;
```

in your crate root. From here you can set up your routes for dealing with
incoming requests.

### Example

The following bit of code creates a simple echo server:

```rust
#[macro_use] extern crate chiisai;

// Imports traits and the rexported hyper and futures crates
use chiisai::*;
use futures::future::ok;
use hyper::header::ContentLength;


static INDEX: &'static [u8] = b"Try POST /echo\n";

fn main() {
    // Set up the routes and run it.
    // You can set the port as well with a function call
    // before run() to port() by default it runs on 7878
    Chiisai::new().routes(router! {
        ("/", GetEcho)
        ("/echo", GetEcho)
        ("/echo", PostEcho)
    }).run().unwrap();
}


routes!(
    // Each route handler needs 3 things:
    // 1) Takes a request verb needed for routes that use this:
    //    Post, Put, Patch, Get, or Delete
    // 2) A name for the handler type, in this case PostEcho
    // 3) A closure. Closures take a hyper::server::Request type and returns a
    //    futures::future::FutureResult<hyper::server::Response, hyper::Error>;
    //    These types are automatically imported in the routes macro (except for
    //    hyper::Error) to reduce what things you need to import
    (Post, PostEcho, |req: Request| {
        let mut res = Response::new();
        if let Some(len) = req.headers().get::<ContentLength>() {
            res.headers_mut().set(len.clone());
        }
        ok(res.with_body(req.body()))
    })

    (Get, GetEcho, |_| {
        ok(Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX))

    })
);
```

After starting this server up if you run the following commands you can
see that all the routes were implemented!

```bash
% curl localhost:7878
Try POST /echo
% curl localhost:7878/echo
Try POST /echo
% curl -X POST -d 'Hello!' localhost:7878/echo
Hello!
```

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
