use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::net::{TcpListener, TcpStream};

fn main () {

    let listener = match TcpListener::bind("127.0.0.1:80"){
        Ok(listener) => listener,
        Err(e) => panic!("There was an issue {}", e)
    };
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

fn handle_client(stream: TcpStream) {
    let ten_millis = time::Duration::from_millis(1000);
    //let now = time::Instant::now();
    println!("started");
    let path = Path::new("foo.txt");
    // does not open in append mode
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't read file: {}", why),
        Ok(f) => f
    };
    for i in 0..10 {
        thread::sleep(ten_millis);
        match file.write_all(b"Woke up for a sec\n") {
            Ok(worked) => worked,
            Err(why) => panic!("Couldn't write to file: {}", why)
                
        };
        println!("wake up check number {}", i);
    }
    // println!("Do something with a steam")
}
