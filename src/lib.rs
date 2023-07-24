use std::{net::TcpListener, io::{Error}, collections::HashMap};

use handlers::route_handler::{RouteDetails, RouteHandler};
use resource_sys::system::{Req, Res, IntoSystem, System};

pub mod utils {
    pub mod parser;
}

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


impl Application {

    pub fn new(address: &str, port: &str, route: RouteHandler) -> Self {
        let full_address = address.to_string() + ":" + port;
        let listener = TcpListener::bind(&full_address);

        Application {
            listener,
            route_collection: route
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