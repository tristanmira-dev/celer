use std::{net::TcpListener, io::{Error}, collections::HashMap};

use handlers::route_handler::{RouteDetails, RouteHandler};
use resource_sys::system::{Req, Res, IntoSystem, System};

pub mod handlers {
    pub mod route_handler;
    pub mod connection_handler;
}
pub mod http {
    pub mod http_methods;
    pub mod transaction;
}
pub mod resource_sys {
    pub mod system;
}

pub mod thread_pool;


pub struct Application {
    listener: Result<TcpListener, Error>,
    route_collection: RouteHandler
}

//CLEANUP
fn hey(res: Res<String>, req: Req<String>) {
    println!("EUREKA req: {}, res: {}", req.inner, res.inner);
}

impl Application {
    pub fn get<T: resource_sys::system::System + 'static>(self, route: String, f: T) -> Application {
        let route = RouteDetails { method: "GET".to_string(), route };

        return Application {
            listener: self.listener,
            route_collection: RouteHandler { 
                routes: RouteHandler::insert_route(route, Box::new(f))
            }
        }
    }

    pub fn post<T: resource_sys::system::System + 'static>(self, route: String, f: T) -> Application {
        let route = RouteDetails { method: "POST".to_string(), route };

        return Application {
            listener: self.listener,
            route_collection: RouteHandler { 
                routes: RouteHandler::insert_route(route, Box::new(f))
            }
        }
    }

    pub fn put<T: resource_sys::system::System + 'static>(self, route: String, f: T) -> Application {
        let route = RouteDetails { method: "PUT".to_string(), route };

        return Application {
            listener: self.listener,
            route_collection: RouteHandler { 
                routes: RouteHandler::insert_route(route, Box::new(f))
            }
        }
    }

    pub fn delete<T: resource_sys::system::System + 'static>(self, route: String, f: T) -> Application {
        let route = RouteDetails { method: "DELETE".to_string(), route };

        return Application {
            listener: self.listener,
            route_collection: RouteHandler { 
                routes: RouteHandler::insert_route(route, Box::new(f))
            }
        }
    }

    pub fn new(address: &str, port: &str) -> Self {
        let full_address = address.to_string() + ":" + port;
        let listener = TcpListener::bind(&full_address);

        Application {
            listener,
            route_collection: RouteHandler {
                routes: HashMap::default()
            }
        }
    
    }

    pub fn run<F: Fn()> (self: &mut Self, F: F) {


        match &self.listener {
            Result::Ok(listener) => {
                F();
                for stream in listener.incoming() {
                    match stream {
                        Result::Ok(val) => { 
                            handlers::connection_handler::stream_handler(val, &mut self.route_collection);
                        },
                        Result::Err(err) => {

                        }
                    }
                }

            },
            Result::Err(err) => {
            }
        }
    }
}