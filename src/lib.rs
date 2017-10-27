extern crate hyper;
extern crate futures;

use std::net::SocketAddr;
use std::collections::HashMap;
use hyper::server::{ Http, Service };
use hyper::{ Request, Response, Method, StatusCode };
use futures::future::FutureResult;

pub struct Chiisai {
    routes: HashMap<(Method, String), Box<Route>>,
    port: u64,
}

impl Chiisai {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            port: 7878 // Rust on T9!
        }
    }

    pub fn route(mut self, method: Method, route: &str, handler: Box<Route>) -> Self {
        self.routes.insert( (method, route.into()), handler);
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
    fn handler(&self, Request) -> FutureResult<Response, hyper::Error>;
}
