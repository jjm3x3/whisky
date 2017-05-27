use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:80") {
        Ok(stream) => stream,
        Err(e) => panic!("Error establishing a connection: {}",e)
    };

    stream.write(&[1]);
}

    
