use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct LogItem {
    item_type: ItemType,
    pub from: String,
    pub message: String
}

enum ItemType {
    Info,
    Error,
    Success,
    Warning
}

impl LogItem {
    pub fn create_info(from: String, message: String) -> LogItem {
        LogItem {
            item_type: ItemType::Info,
            from,
            message
        }
    }
    pub fn create_success(from: String, message: String) -> LogItem {
        LogItem {
            item_type: ItemType::Success,
            from,
            message
        }
    }

    pub fn create_error(from: String, message: String) -> LogItem {
        LogItem {
            item_type: ItemType::Error,
            from,
            message
        }
    }

    pub fn create_warning(from: String, message: String) -> LogItem {
        LogItem {
            item_type: ItemType::Warning,
            from,
            message
        }
    }
}

pub fn run(receiver: Receiver<LogItem>) {
    loop {
        let item = receiver.recv();

        match item {
            Ok(item) => {
                match item.item_type {
                    ItemType::Info => {
                        println!("INFO {}:{}", item.from, item.message)
                    }
                    ItemType::Error => {
                        println!("ERRO {}:{}", item.from, item.message)
                    }
                    ItemType::Success => {
                        println!("SUCC {}:{}", item.from, item.message)
                    }
                    ItemType::Warning => {
                        println!("WARN {}:{}", item.from, item.message)
                    }
                }
            }
            Err(item) => {
                println!("Critical Error: {}", item);
            }
        }
    }
}