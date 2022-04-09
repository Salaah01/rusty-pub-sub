use server::{consumer, state};
use std::{io::Write, net::TcpListener};
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
                    consumer::consumer(&mut stream);
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
