use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:42001").expect("Could not connection.");

    println!("Listenering...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        //        let mut buffer = [0; 2048]; 

        loop {
            let mut request = String::new();

            println!("Waiting for a request...");
            
            stream.read_to_string(&mut request);

            println!("Request recevied: {}", request);

            stream.write("Hello from controller".as_bytes());

        }
    }

    println!("Hello, world!");
}
