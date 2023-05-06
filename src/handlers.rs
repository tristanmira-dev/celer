use std::collections::HashMap;
use super::methods::*;



pub struct RouteHandler {
    routes: HashMap<String, Box<dyn Route>>
}


impl RouteHandler {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new()
        }
    }
    pub fn test_route(self: &Self, key: String)  {
        match self.routes.get(&key) {
            Some(value) => {
                value.return_method()();
            },
            None => {
                println!("FAILED");
            }
        }
    }

    pub fn register_route(self: &mut Self, route_name: &str, route_details: Box<dyn Route>) {
        self.routes.insert(route_name.to_owned(), route_details);
    }
    pub fn get<F: Fn() + 'static>(self: &mut Self, route_name: &str, F: F) {
        self.register_route(route_name, Box::new(Get::new(route_name.to_owned(), Box::new(F))));
    }
    pub fn post<F: Fn() + 'static>(self: &mut Self,  route_name: &str, F: F) {
        self.register_route(route_name, Box::new(Post::new(route_name.to_owned(), Box::new(F))));
    }
    pub fn put<F: Fn() + 'static>(self: &mut Self,  route_name: &str, F: F) {
        self.register_route(route_name, Box::new(Put::new(route_name.to_owned(), Box::new(F))));
    }
} 