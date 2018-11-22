use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:9080") {
        Ok(stream) => stream,
        Err(e) => panic!("Error establishing a connection: {}",e)
    };

    let _message = "Hi fam".as_bytes();

    match stream.write(_message){
        Ok(_) => println!("Message sent"),
        Err(e) => panic!("Unable to send message: {}", e)
    };
}

    
