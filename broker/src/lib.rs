use std::io;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod connection_handler;
mod event_loop;
mod job;
mod logger;
mod message;

pub struct JobResult {
    pub id: uuid::Uuid,
    pub complete: bool,
    pub value: String,
    result: String,
}

impl JobResult {
    pub fn new(job: job::Job) -> JobResult {
        JobResult {
            id: job.id,
            complete: true,
            value: format!("Result! Value: {}", job.value),
            result: "Yes".to_string(),
        }
    }
}

pub fn run() {
    // Create the logger channels and start the logger.
    let (logger_tx, logger_rx): (Sender<logger::LogItem>, Receiver<logger::LogItem>) =
        mpsc::channel();
    
        let logger_thread = thread::spawn(|| {
        logger::run(logger_rx);
    });
 
    let address = "127.0.0.1:42001".to_string();

    // Create the requested channels.
    //let (main_)
    let (main_requests_tx, main_requests_rx) = create_request_channels();
    let (main_responses_tx, main_responses_rx) = create_response_channels();
    let (handler_requests_tx, handler_requests_rx) = create_request_channels();
    let (handler_responses_tx, handler_responses_rx) = create_response_channels();

    let handler_logger = logger_tx.clone();
    let event_loop_logger = logger_tx.clone();
    // Start the handler
    let handler_thread = thread::spawn(|| {
        connection_handler::run(handler_requests_rx, handler_responses_tx, handler_logger);
    });

    // start the event loop.
    let event_loop_thread = thread::spawn(|| {
        event_loop::run(
            main_requests_rx,
            main_responses_tx,
            handler_requests_tx,
            handler_responses_rx,
            event_loop_logger,
        );
    });

    // Go to repl mode.
    loop {
        let input = get_input("> ");

        match &input[..] {
            "test" => println!("Hello World"),
            "exit" => {
                main_requests_tx.send(message::Request::create_close());
                break;
            }
            "ping" => {
                let message = get_input("Message > ");

                main_requests_tx.send(message::Request::create_ping(message));
            }
            _ => println!("Unknown command."),
        }

        // Wait for a response.
        let response = main_responses_rx.recv();

        match response {
            Ok(response) => {
                println!("Response: {}", response.body);
            }
            Err(_) => {}
        };
    }

    println!("Exiting...");
    // Wait for the threads to close.
    handler_thread.join();
    event_loop_thread.join();

    println!("Goodbye!");
}

fn create_request_channels() -> (Sender<message::Request>, Receiver<message::Request>) {
    let channels: (Sender<message::Request>, Receiver<message::Request>) = mpsc::channel();

    channels
}

fn create_response_channels() -> (Sender<message::Response>, Receiver<message::Response>) {
    let channels: (Sender<message::Response>, Receiver<message::Response>) = mpsc::channel();

    channels
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}
