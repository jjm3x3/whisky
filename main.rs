use std::{thread};
use std::io::ErrorKind;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::string::String;

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

#[derive(Debug)]
struct Context {
    headers: std::collections::HashMap<String,String>,
    method: String,
    url: String,
    protocol: String,
}

impl Context {
    fn new(request_string: String) -> Context {
        
        let first_line = request_string.lines().nth(0);
        let mut method = String::from("");
        let mut url = String::from("");
        let mut proto = String::from("");
        let mut headers = std::collections::HashMap::new();
        match first_line {
            Some(fl) => {
                // println!("First line: '{}'", fl);
                let mut parts = fl.split_whitespace();
                // println!("how many values: {}", parts.count())
                method = match parts.nth(0) {
                    Some(m) => String::from(m),
                    None => { println!("Request missing method"); String::from("") }
                };
                url = match parts.nth(0) {
                    Some(u) => String::from(u),
                    None => { println!("Request missing url"); String::from("") }
                };
                proto = match parts.nth(0) {
                    Some(p) => { println!("What am I seeing here? {}", p); String::from(p) },
                    None => { println!("Request missing protocol"); String::from("") }
                };
            },
            None => ()
        }
        for l in request_string.lines().skip(1) {
            if l.is_empty() {
                continue
            }
            let colon_index = l.find(':').unwrap_or(l.len()-1);
            let name = String::from(&l[..colon_index]);
            let value = String::from(&l[colon_index+2..]);
            headers.insert(name, value);
            // println!("header name: '{}'", name);
            // println!("header value: '{}'", value);
            
        }
        Context {
            headers: headers,
            method: method,
            url: url,
            protocol: proto,
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("handling request");
    stream.set_read_timeout(Some(Duration::from_millis(1))).unwrap();
    let mut request = Vec::new();
    match stream.read_to_end(&mut request) {
        Ok(bytes_read) => println!("We have read {} bytes", bytes_read),
        Err(e) => {
            match e.kind() {
                ErrorKind::WouldBlock => {
                    println!("would have blocked ");
                },
                _ => panic!("somhow a non byte came through: {}", e)
            }
        }
    }
    let string_request = match String::from_utf8(request){
        Ok(sr) => sr,
        Err(e) => { println!("The request is not in utf8: {}", e); String::from("")}
    };


    let context = Context::new(string_request);
    println!("Built context: {:?}", context);

    match stream.write(b"404 page not found") {
        Ok(_/*bytew_written*/) => {},
        Err(e) => println!("Error while writing result: {}", e)
    };
}
