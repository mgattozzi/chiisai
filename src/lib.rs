pub extern crate hyper;
pub extern crate futures;

use std::net::SocketAddr;
use std::collections::HashMap;
use hyper::server::{ Http, Service };
use hyper::{ Request, Response, Method, StatusCode };
use futures::future::FutureResult;

pub struct Chiisai {
    routes: HashMap<(Method, String), Box<Route<Method = Method>>>,
    port: u64,
}

impl Chiisai {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            port: 7878 // Rust on T9!
        }
    }

    pub fn routes(mut self, routes: HashMap<(Method, String), Box<Route<Method = Method>>>) -> Self {
        self.routes = routes;
        self
    }

    pub fn port(mut self, port_num: u64) -> Self {
        self.port = port_num;
        self
    }

    pub fn run(self) -> Result<(), hyper::Error>
    {
        let address_str = "127.0.0.1:".to_string() + &self.port.to_string();
        let address: SocketAddr = address_str.parse().unwrap();
        println!("Running server on {}", address);
        Http::new()
             .bind(&address, move || Ok(&self))?
             .run()?;
        Ok(())
    }
}

impl<'c> Service for &'c Chiisai
{
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let method = req.method().to_owned();
        let path = req.path().to_owned();
        match self.routes.get(&(method, path)) {
            Some(route) => {
                route.handler(req)
            },
            None => {
                futures::future::ok(Response::new().with_status(StatusCode::NotFound))
            },
        }
    }

}

pub trait Route {
    type Method;
    fn handler(&self, Request) -> FutureResult<Response, hyper::Error>;
    fn method(&self) -> Self::Method;
}

#[macro_export]
macro_rules! router {
    ($( ($route: expr, $handler: expr))*) => {
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            $(
                let boxed: Box<Route<Method = hyper::Method>> = Box::new($handler);
                map.insert((boxed.method(), $route.to_string()), boxed);
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! routes {
    ($(($method: ident, $type: ident, $function: expr))*) => {
        use hyper::server::{ Request, Response };
        use futures::future::FutureResult;
        $(
        struct $type;
        impl Route for $type {
            type Method = hyper::Method;
            fn handler(&self, _req: Request) -> FutureResult<Response, hyper::Error> {
                #[inline(always)]
                $function(_req)
            }
            #[inline(always)]
            fn method(&self) -> Self::Method {
                hyper::Method::$method
            }
        }
        )*
    }
}

