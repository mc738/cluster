use std::thread;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:42001").expect("Could not connection.");

    println!("Waiting for a connection...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("Connection!");

        let mut buffer = [0; 2048]; 

        loop {
            let mut request = String::new();

            println!("Waiting for a request...");
            
            stream.read(&mut buffer).unwrap();

            let request_str = String::from_utf8_lossy(&buffer).into_owned();
        
            println!("Request recevied: {}", request_str);

            stream.write("Hello from controller".as_bytes());

            thread::sleep_ms(1000);

        }
    }

    println!("Hello, world!");
}
