use std::{thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

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
    stream.set_read_timeout(Some(Duration::from_millis(1)));
    let mut request = Vec::new();
    match stream.read_to_end(&mut request) {
        Ok(bytes_read) => println!("We have read {} bytes", bytes_read),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::WouldBlock => {
                    println!("would have blocked ");
                },
                _ => panic!("somhow a non byte came through: {}", e)
            }
        }
    }
    let string_request = String::from_utf8(request);

    match string_request {
        Ok(sr) => println!("Here is the message we recived {:?}", sr),
        Err(e) => println!("The request is not in utf8: {}", e)
    };

    stream.write(b"404 page not found");
}
