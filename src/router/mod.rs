use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;


pub enum Method {
    GET,
    POST,
    DELETE,
}

pub trait Router {
    fn add<H>(&mut self, m: Method, path: String, h: H) where H: Handler;
}

pub struct IronMux {
    routes: HashMap<String, Box<Handler>>,
}

impl IronMux {
    pub fn new() -> Self {
        IronMux { routes: HashMap::new() }
    }
}

impl Handler for IronMux {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path.join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

impl Router for IronMux {
    fn add<H>(&mut self, _: Method, path: String, h: H)
        where H: Handler
    {
        self.routes.insert(path, Box::new(h));
    }
}
