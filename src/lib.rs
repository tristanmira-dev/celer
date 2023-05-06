use std::{net::TcpListener, io::{Error}};
use handlers::route_handler::RouteHandler;

pub mod handlers {
    pub mod route_handler;
    pub mod connection_handler;
}
pub mod http_methods;


pub struct Application {
    listener: Result<TcpListener, Error>
}

impl Application {
    pub fn new(address: &str, port: &str) -> Self {
        let full_address = address.to_string() + ":" + port;
        let listener = TcpListener::bind(&full_address);
        let mut test =  RouteHandler::new();


        Application {
            listener
        }
    
    }

    pub fn run<F: Fn()> (self: &Self, F: F) {
        match &self.listener {
            Result::Ok(listener) => {
                F();
                for stream in listener.incoming() {
                    match stream {
                        Result::Ok(val) => { 
                            handlers::connection_handler::stream_handler(val);
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