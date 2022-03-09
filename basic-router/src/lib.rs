use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    fn new() -> Self {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request),
        }
    }
}

fn not_found_response() -> Response {
    unimplemented!();
}

struct FnPointerRouter {
    routes: HashMap<String, fn(&Request) -> Response>,
}

impl FnPointerRouter {
    fn new() -> Self {
        FnPointerRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response) {
        self.routes.insert(url.to_string(), callback);
    }
}
