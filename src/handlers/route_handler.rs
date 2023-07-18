use std::collections::HashMap;
use crate::resource_sys::system::{self, ArgsCollection};
use crate::http::http_methods;
use crate::http::transaction::Response;
//Delete, Post, Put


// pub trait Route {
//     fn return_method<'a> (self: &'a Self) -> &'a Box<dyn Fn()-> Response>;
//     fn ret_http_method(self: &Self) -> &str;
// //...TO BE DECIDED
// }

pub struct RouteDetails {
    pub method: String,
    pub route: String
}


pub struct RouteHandler {
    pub routes: HashMap<String, Box<dyn system::System>>
}

impl RouteHandler {
    pub fn execute_route(&mut self, route: String, mut args: ArgsCollection) {
        println!("{}", route);
        self.routes.get_mut(&route).unwrap().call_system(&mut args.args);
    }


    pub fn insert_route(route: RouteDetails, f: Box<dyn system::System>) -> HashMap<String, Box<dyn system::System>> {
        let mut hash_map = HashMap::new();

        hash_map.insert(route.method + " " + &route.route, f);

        return hash_map;
    }

}


// impl RouteHandler {
//     pub fn new() -> Self {
//         Self {
//             routes: HashMap::new()
//         }
//     }
//     pub fn test_route(self: &Self, key: String)  {
//         match self.routes.get(&key) {
//             Some(value) => {
//                 // value.return_method()();
//             },
//             None => {
//                 println!("FAILED");
//             }
//         }
//     }

//     pub fn register_route(self: Self, route_name: &str, route_details: Box<dyn Route>) -> Self {
        
//         let mut route_map = HashMap::new();

//         route_map.insert(route_name.to_owned(), route_details);
        

//         Self {
//             routes: route_map
//         }
//     }
//     pub fn get<F: Fn() -> String + 'static>(self: Self, route_name: &str, F: F) -> Self {
//         self.register_route(route_name, Box::new(Get::new(route_name.to_owned(), Box::new(F))))
//     }

//     pub fn return_routes(self: &Self) -> &HashMap<String, Box<dyn Route>> {
//         return &self.routes;
//     }
//     // pub fn post<F: Fn() + 'static>(self: &mut Self,  route_name: &str, F: F) -> Self {
//     //     self.register_route(route_name, Box::new(Post::new(route_name.to_owned(), Box::new(F))))
//     // }
//     // pub fn put<F: Fn() + 'static>(self: &mut Self,  route_name: &str, F: F) -> Self {
//     //     self.register_route(route_name, Box::new(Put::new(route_name.to_owned(), Box::new(F))))
//     // }
// } 