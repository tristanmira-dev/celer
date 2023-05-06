use std::net::TcpStream;
use std::io::{BufReader, BufRead};


pub fn stream_handler (mut stream: TcpStream) {
    println!("TEST");
    let buffer = BufReader::new(&mut stream);

    let mut request_iter = buffer.lines().map(|x| x.unwrap()).into_iter();

    let counter: usize = 0;

    loop {
        match request_iter.next() {
            Some(val) => {
                if val == "" {
                    break
                };

                dbg!(val);
                continue;
            },
            None => {
                break;
            }
        };
        


    }


    
} 