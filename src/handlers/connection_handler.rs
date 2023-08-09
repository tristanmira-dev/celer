use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};
use crate::resource_sys::system::{ArgsCollection, Req, Res, Global};
use crate::utils::parser::parse_request;

use super::route_handler::RouteHandler;

// pub fn route_handler (route_handler: &mut RouteHandler) {

// }

pub fn stream_handler<G: 'static> (mut stream: TcpStream, routes: &mut RouteHandler<G>) {

    let buffer = BufReader::new(&mut stream);

    let mut request_iter = buffer.lines().map(|x| x.unwrap()).into_iter();

    let request_iter = parse_request(&mut request_iter);

    dbg!(&request_iter);

    let mut args = ArgsCollection::init();

    

    //DUMMY DATA
    args.add_param(Req {
        inner: 3
    });

    let opt = routes.execute_route(RouteHandler::<G>::to_route(request_iter), args);

    // dbg!(opt.return_method()());

    let status_line = "HTTP/1.1 200 OK";
    // let content = opt.return_method()();
    let string = "5".to_string();
    let response = format!("{status_line}\r\nContent-Length:{string}\r\n\r\n{}", string.len());


    stream.write_all(response.as_bytes()).unwrap();


    
} 