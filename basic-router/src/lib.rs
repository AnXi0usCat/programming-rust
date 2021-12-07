use std::collections::HashMap;

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct Response {
    code: i32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct SimpleRouter {
    routes: HashMap<String, BoxedCallback>
}

impl SimpleRouter {
    // Create new router
    fn new() -> Self {
        SimpleRouter {routes: HashMap::new()}
    }
    // Add a new route
    fn add_route<C>(&mut self, url: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request)
        }
    }
}

fn not_found_response() -> Response {
    Response {
        code:  200,
        headers: HashMap::new(),
        body: Vec::new()
    }
}
