//! # Server
//! This binary listens for connections and handle messages to and from the
//! client.
//! It is the main entry point for the server and is ultimately responsible for
//! facilitating the communication between the client and the server.

use server::{consumer, state};
use std::net::TcpListener;
use std::thread;

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
    // Get host and port from command line arguments or use defaults
    let args: Vec<String> = std::env::args().collect();

    let host = if args.len() > 1 {
        &args[1]
    } else {
        "localhost"
    };
    let port = if args.len() > 2 { &args[2] } else { "7878" };

    let listener: TcpListener =
        TcpListener::bind(format!("{}:{}", host, port)).expect("Could not bind to port");
    let server = Server::new(listener);
    server.run();
}
