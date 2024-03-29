use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use crate::resource_sys::system::{self, ArgsCollection, System, Global};
use crate::http::http_methods;
// use crate::http::transaction::Response;
//Delete, Post, Put


// pub trait Route {
//     fn return_method<'a> (self: &'a Self) -> &'a Box<dyn Fn()-> Response>;
//     fn ret_http_method(self: &Self) -> &str;
// //...TO BE DECIDED
// }




pub struct Route {
    pub method: String,
    pub system: Box<dyn System>,
    pub route: String,
    pub global: Option<Rc<dyn Any>>
}


// impl Route {
//     fn clone_box(&self) -> Box<dyn Any> {
//         self.globals
//     }
// }


pub struct RouteBuilder {
    staging_routes: Vec<Route>,
}

impl RouteBuilder {
    pub fn new() -> Self {
        Self {
            staging_routes: Vec::new()
        }
    }


    pub fn get<T: System + 'static>(mut self, route: String, f: T) -> Self {

        let route = Route { method: "GET".to_string(), route, system: Box::new(f), global: None  };

        self.staging_routes.push(route);

        return self;
        
    }

    pub fn post<T: System + 'static>(mut self, route: String, f: T) -> Self  {

        let route = Route { method: "POST".to_string(), route, system: Box::new(f), global: None  };

        self.staging_routes.push(route);

        return self;
    }

    pub fn put<T: System + 'static>(mut self, route: String, f: T) -> Self  {

        let route = Route { method: "PUT".to_string(), route, system: Box::new(f), global: None  };

        self.staging_routes.push(route);

        return self;
        
    }
    

    pub fn delete<T: System + 'static>(mut self, route: String, f: T) -> Self  {
        
        let route = Route { method: "DELETE".to_string(), route, system: Box::new(f), global: None  };

        self.staging_routes.push(route);

        return self;
     
    }

    pub fn with_global<T: 'static> (mut self, global: T) -> Self {
        let size: usize = self.staging_routes.len();


        let route = self.staging_routes.get_mut(size-1);

        match route {
            None => {

            },

            Some(route) => {
                
                println!("{}", route.route);
                route.global = Option::Some(Rc::new(global)); 
            }
        }

        return self;
    }

    pub fn to_handler(self) -> RouteHandler {

        let mut route_map = HashMap::new();

        for i in self.staging_routes {

            
            route_map.insert(i.method.clone() + " " + &i.route, RouteDetails { details: i });
            

        }


        RouteHandler { routes: route_map }
    }


    // pub fn init(self) -> RouteHandler<i32> {

    //     let iter = self.staging_routes.into_iter();

    //     let mut route_handler = RouteHandler::new();


    //     for i in iter {

    //         route_handler.routes.insert(
    //             i.method.clone() + " " + &i.route.clone(), 
    //             RouteDetails { 
    //                 details: Route { 
    //                     method: i.method.clone(), route: i.route.clone(), system: i.system, 
    //                 }, 
    //                 global: Global { inner: 0 } 
    //             });
    //     }

        
    //     return route_handler
    // }
}


pub struct RouteDetails {
    pub details: Route,
}


pub struct RouteHandler {
    pub routes: HashMap<String, RouteDetails>
}

impl RouteHandler {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new()
        }
    }

    pub fn to_route(req_collection: &Vec<String>) -> String {
        return req_collection[0].to_string() + " " + &req_collection[1];
          
    }

    pub fn execute_route(&mut self, route: String, mut args: ArgsCollection) {

        dbg!("{}", self.routes.keys());

        self.routes.get_mut(&route).unwrap().details.system.call_system(&mut args.args);
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