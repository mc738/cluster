use crate::message;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::logger;

pub fn run(
    requests: Receiver<message::Request>,
    responses: Sender<message::Response>,
    logger: Sender<logger::LogItem>,
) {
    let mut buffer = [0; 2048];

    println!("Connection handler running");

    loop {
        // Look for a message.
        let request = requests.try_recv();

        match request {
            Ok(r) => {
                println!("Connection Handler: Request received!");

                match r.request_type {
                    message::RequestType::Job(job) => {

                        // Convert the job to bytes.

                        // Send the bytes.
                    }
                    message::RequestType::Ping(message) => {
                        println!("Connection Handler: {}", message);

                        responses.send(message::Response::new(message));
                    }
                    message::RequestType::Close() => {
                        println!("Connection Handler: Exit recieved");
                        // On a close message break out the loop
                        // and start the shutdown process
                        break;
                    }
                }
            }
            Err(r) => {}
        }

        // Read the response from the controller.
        // let mut response_str = String::new();
        //handler.connection.read(&mut buffer).unwrap();

        //let response_str = String::from_utf8_lossy(&buffer).into_owned();
        // println!("Connection handler: Response! {}", response_str);
        // Send the response

        thread::sleep_ms(500);
    }

    println!("Closing connection handler");
}
