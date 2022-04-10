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
}
