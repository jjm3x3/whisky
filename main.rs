use std::{thread, time};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main () {

    let listener = match TcpListener::bind("127.0.0.1:80"){
        Ok(listener) => listener,
        Err(e) => panic!("There was an issue {}", e)
    };
    println!("listening on port 80");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => panic!("an error has occured {}", e)
        }
    }

}

fn handle_client(mut stream: TcpStream) {
    println!("handling request");
    let mut request = Vec::new();
    match stream.read_to_end(&mut request) {
        Ok(bytes_read) => println!("We have read {} bytes", bytes_read),
        Err(e) => println!("There was an error reading clients message: {}", e)
    }
    let string_request = String::from_utf8(request);
    println!("Here is the message recived: '{:?}'", string_request);
    let ten_millis = time::Duration::from_millis(1000);
    for i in 0..10 {
        thread::sleep(ten_millis);
        println!("wake up check number {}", i);
    }
}
