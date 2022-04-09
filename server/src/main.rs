//! # Server
//! This binary listens for connections and handle messages to and from the
//! client.
//! It is the main entry point for the server and is ultimately responsible for
//! facilitating the communication between the client and the server.

use server::{consumer, state};
use std::thread;
use std::{net::TcpListener};

struct Server {
    listener: TcpListener,
}

impl Server {
    fn new(listener: TcpListener) -> Server {
        Server { listener }
    }

    fn run(&self) {
        for stream in self.listener.incoming() {
            let client = state::Client::new();
            match stream {
                Ok(mut stream) => {
                    client.add_client(&stream);
                    println!("New client connected");
                    thread::spawn(move || {
                        consumer::consumer(&mut stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let server = Server::new(listener);
    server.run();
}
