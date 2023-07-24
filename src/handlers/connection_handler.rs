use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};
use crate::resource_sys::system::{ArgsCollection, Req, Res};
use crate::utils::parser::parse_request;

use super::route_handler::RouteHandler;

pub fn route_handler (route_handler: &mut RouteHandler) {
    
}

pub fn stream_handler (mut stream: TcpStream, routes: &mut RouteHandler) {

    let buffer = BufReader::new(&mut stream);

    let mut request_iter = buffer.lines().map(|x| x.unwrap()).into_iter();

    let request_iter = parse_request(&mut request_iter);

    dbg!(&request_iter);

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