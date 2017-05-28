use std::{thread};
use std::io::ErrorKind;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::string::String;
use std::collections::HashMap;

fn main () {
    let mut server = Whisky::new("9080");
    server.get("ping", ping_handler);

    server.run()
    
}

fn ping_handler(c: Context) {
    println!("I have been routed and now I just need to be handled")
}

struct Whisky {
    server: TcpListener,
    port: String,
    handlers: HashMap<String, WhiskyHandler >
}

type WhiskyHandler = fn(Context) -> ();

impl Whisky {
    fn new(port: &str) -> Whisky {
        let listener = match TcpListener::bind("127.0.0.1:".to_string() + port){
            Ok(listener) => listener,
            Err(e) => panic!("There was an issue {}", e)
        };
        Whisky{server: listener, port: String::from(port), handlers: std::collections::HashMap::new()}
    }

    fn run(&self) {
        println!("listening on port {}", self.port);
        for stream in self.server.incoming() {
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

    fn get(&mut self,route: &str, f: WhiskyHandler) {
        println!("install a handler");
        self.handlers.insert(String::from(route), f);
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
                let mut parts = fl.split_whitespace();
                method = match parts.nth(0) {
                    Some(m) => String::from(m),
                    None => { println!("Request missing method"); String::from("") }
                };
                url = match parts.nth(0) {
                    Some(u) => String::from(u),
                    None => { println!("Request missing url"); String::from("") }
                };
                proto = match parts.nth(0) {
                    Some(p) => { String::from(p) },
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
        Ok(_/*bytew_written*/) => (),
        Err(e) => println!("Error while writing result: {}", e)
    };
}
