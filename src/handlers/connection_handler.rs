use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};
use super::route_handler::RouteHandler;

pub fn route_handler (route_handler: &mut RouteHandler) {
    
}

pub fn stream_handler (mut stream: TcpStream, routes: &RouteHandler) {

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

                
                dbg!(&val);

                request_vec.push(val);

                continue;
            },
            None => {
                break;
            }
        };
        


    }

    stream.write(b"Hardcoded").unwrap();


    
} 