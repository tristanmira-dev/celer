use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};
use crate::resource_sys::system::{ArgsCollection, Req, Res};

use super::route_handler::RouteHandler;

pub fn route_handler (route_handler: &mut RouteHandler) {
    
}

pub fn stream_handler (mut stream: TcpStream, routes: &mut RouteHandler) {

    let buffer = BufReader::new(&mut stream);

    let mut request_iter = buffer.lines().map(|x| x.unwrap()).into_iter();

    let mut request_vec: Vec<String> = Vec::new();

    let counter: usize = 0;

    loop {
        match request_iter.next() {
            Some(val) => {
                if val == "" {
                    break
                };

                request_vec.push(val);

                continue;
            },
            None => {
                break;
            }
        };
    
    }
    
    let req_method_vec = &request_vec[0];

    let req_method_vec: Vec<&str> = req_method_vec.split(' ').collect();

    dbg!(&req_method_vec);

    let i: i32 = 50;

    let mut args = ArgsCollection::init();


    //DUMMY DATA
    args.add_param(Req {
        inner: 12
    });

    //DUMMY DATA
    args.add_param(Res {
        inner: 40
    });

    let opt = routes.execute_route("GET /test".to_string(), args);

    // dbg!(opt.return_method()());

    let status_line = "HTTP/1.1 200 OK";
    // let content = opt.return_method()();
    let string = "5".to_string();
    let response = format!("{status_line}\r\nContent-Length:{string}\r\n\r\n{}", string.len());


    stream.write_all(response.as_bytes()).unwrap();


    
} 