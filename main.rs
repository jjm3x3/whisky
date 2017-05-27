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

    
    // let mut aCollection: Vec<u8> = Vec::new();
    // for b in stream.bytes() {
    //     match b {
    //         Ok(b) => { aCollection.push(b)  },
    //         Err(e) => {
    //             match e.kind() {
    //                 std::io::ErrorKind::WouldBlock => {
    //                     println!("would have blocked ");
    //                     break
    //                 },
    //                 _ => panic!("somhow a non byte came through: {}", e)
    //             }
    //         }
    //     };

    //     // println!("Here is a byte: {:?}", abyte as char)
    // }

    // let aCollection: Vec<std::io::Result<u8>>  = stream.bytes().collect();
    // println!("what does this look like?: {:?}", aCollection);

    // let aCollection: Vec<u8> = stream.bytes().map(|x| {
    //     match x {
    //         Ok(b) => b,
    //         Err(e) => {
    //             match e.kind() {
    //                 std::io::ErrorKind::WouldBlock => {
    //                     println!("would have blocked ")
    //                 },
    //                 _ => panic!("somhow a non byte came through: {}", e)
    //             }
    //             0
    //         }
    //     }
    // }).collect();
    // println!("what does this look like?: {:?}", aCollection);

    // let string_request = std::str::from_utf8(&aCollection);
    // println!("Here is the message recived: '{:?}'", string_request);

}
