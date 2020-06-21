use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};



fn main() {

    println!("Welcome to cluster!");

  

    broker::run();

   
}
