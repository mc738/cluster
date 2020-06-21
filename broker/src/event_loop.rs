use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::message;
use crate::logger;

pub fn run(
    main_requests: Receiver<message::Request>,
    main_responses: Sender<message::Response>,
    handler_requests: Sender<message::Request>,
    handler_responses: Receiver<message::Response>,
    logger: Sender<logger::LogItem>
) {

    loop {
        // Look for a message from the main thread.
        let request = main_requests.try_recv();

        match request {
            Ok(r) => {
                // Do anything needed with the message here.
                match &r.request_type {
                    message::RequestType::Job(job) => {
                        handler_requests.send(r);
                    }
                    message::RequestType::Ping(message) => {
                        println!("Event Loop: {}", message);
                        handler_requests.send(r);
                    }
                    message::RequestType::Close() => {
                        println!("Event Loop: Exit recieved");
                        handler_requests.send(r).expect("Couldn't send message.");
                        // On a close message break out the loop
                        // and start the shutdown process
                        break;
                    }
                }
                // If one found pass to the connection handler.
            }
            Err(_) => {}
        };

        // Look for a response from the connection handler.

        let response = handler_responses.try_recv();

        match response {
            Ok(r) => {
                println!("Event Loop: Response Received! Body: {}", r.body);

                // Make a respone for for main.
                main_responses.send(r);
            }
            Err(_) => {}
        };

        // Sleep for a bit.
        thread::sleep_ms(500);
    }

    println!("Event loop: Closing");
}
