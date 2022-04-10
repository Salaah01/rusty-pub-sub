//! Client
//! This binary handles client connections to the server.
//! It is the main entry point for the client and handles sending and receiving
//! messages to and from the server.

use client::cli::{Options, Parser};
use client::client::Client;

fn main() {
    let options = Options::new();
    let mut client = Client::new(options.host.clone(), options.port.clone());
    let mut parser = Parser::new(&options, &mut client);
    parser.parse_args();

    // let mut client = Client::new("localhost".to_string(), 7878);
    // // client.send("Hello, world!".to_string());
    // client.ping();
    // client.subscribe("test".to_string());
    // // client.receive(|message| print!("got single message {}", message));
    // // client.unsubscribe("test".to_string());

    // // client.subscribe("test2".to_string());
    // client.publish("test".to_string(), "Hello world!\n".to_string());
    // client.listen(|message| println!("got message {}", message));
    // // client.unsubscribe("test".to_string());

    // client.disconnect();
}
