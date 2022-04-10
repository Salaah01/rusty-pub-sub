//! Client CLI
//! The CLI for interacting with the client.

use std::io::Write;

use crate::client::Client;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "client")]
pub struct Options {
    /// The hostname of the server
    #[structopt(short="H", long, default_value = "localhost")]
    pub host: String,

    /// The port of the server
    #[structopt(short="P", long, default_value = "7878")]
    pub port: u16,

    /// Interactive mode
    #[structopt(short, long)]
    pub interactive: bool,

    /// Ping the server
    #[structopt(short, long)]
    pub ping: bool,

    /// Channel to subscribe to
    #[structopt(short, long = "sub")]
    pub subscribe: Vec<String>,

    /// Channel to unsubscribe from
    #[structopt(short, long = "unsub")]
    pub unsubscribe: Vec<String>,

    /// Channel on which to send a message. If not specified, the message will
    /// be sent to the server without a channel specified.
    #[structopt(short, long)]
    pub channel: Option<String>,

    /// Sends a message to the server
    #[structopt(short, long = "msg")]
    pub message: Option<String>,

    /// Waits to receive a message from the server
    #[structopt(short, long = "recv")]
    pub recv: bool,

    /// Listens continuously for messages from the server
    #[structopt(short, long)]
    pub listen: bool,
}

impl Options {
    /// Parses and validates the command line arguments and returns a new
    /// `Options` instance.
    /// # Returns
    /// A new `Options` instance.
    /// # Panics
    /// An error if the command line arguments are invalid.
    pub fn new() -> Options {
        let opts = Options::from_args();

        // If there is a channel defined but no message, then panic.
        if opts.channel.is_some() && opts.message.is_none() {
            panic!("You must specify a message to send when using a channel.");
        }
        opts
    }
}

/// This struct holds the user's options and the client object. It is
/// responsible for parsing the user's input and calling the appropriate
/// methods on the client object.
pub struct Parser<'a> {
    options: &'a Options,
    client: &'a mut Client,
}

impl Parser<'_> {
    /// Initializes a new parser instance and validates.
    /// # Arguments
    /// * `options` - The user's options.
    /// * `client` - The client object.
    /// # Returns
    /// A new parser instance.
    pub fn new<'a>(options: &'a Options, client: &'a mut Client) -> Parser<'a> {
        Parser { options, client }
    }

    /// Parses the user's input and calls the appropriate methods on the
    /// client.
    pub fn parse_args(&mut self) {
        self.handle_interactive();
        self.handle_ping();
        self.handle_subscriptions();
        self.handle_messages();
        self.handle_listening(false);
        self.handle_receiving(false);
    }

    /// Interactive mode.
    fn handle_interactive(&mut self) {
        loop {
            let mut line = String::new();
            print!("rusty-pub-sub> {}", line);
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut line).unwrap();
            let line = line.trim().to_string();

            // if empty line, continue
            if line.is_empty() {
                continue;
            }
            println!("{}", line);

            match line.to_uppercase().as_str() {
                "DISCONNECT" | "RECV" | "RECEIVE" | "EXIT" | "CLOSE" | "QUIT" => {
                    self.client.disconnect();
                    break;
                }
                "HELP" => {
                    println!("\nUSAGE:\n\t[COMMAND] [OPTIONS]\n");
                    println!("HELP\t\t\t\t Shows this message");
                    println!("EXIT\t\t\t\t Exits program",);
                    println!("PING\t\t\t\t Pings the server");
                    println!("SUBSCRIBE [channel]\t\t Subscribes to a channel");
                    println!("UNSUBSCRIBE [channel]\t\t Unsubscribes from a channel");
                    println!("PUBLISH [channel] [message]\t Publishes a message to a channel");
                    println!("SEND [message]\t\t\t Publishes a message to the server");
                    println!("RECV\t\t\t\t Receives a message from the server");
                    println!("LISTEN\t\t\t\t Listens for messages from the server");
                }
                "LISTEN" => self.handle_listening(true),
                _ => {
                    let mut msg = line;
                    if msg.to_uppercase().starts_with("PUBLISH") {
                        msg.push_str("\n");
                    }
                    self.client.send(msg);
                }
            };
        }
    }

    /// Pings the server.
    fn handle_ping(&mut self) {
        if self.options.ping {
            self.client.ping().unwrap();
        };
    }

    // Handles subscribing and unsubscribing to channels.
    fn handle_subscriptions(&mut self) {
        if self.options.subscribe.len() > 0 {
            for channel in &self.options.subscribe {
                self.client.subscribe(channel.to_string());
            }
        }

        if self.options.unsubscribe.len() > 0 {
            for channel in &self.options.unsubscribe {
                self.client.unsubscribe(channel.to_string());
            }
        }
    }

    // Handles sending messages. Depending on whether a channel was specified,
    // the message will be sent to the server as a raw message, or sent with
    // the intent for it to be sent to published to a specific channel.
    fn handle_messages(&mut self) {
        if let Some(message) = &self.options.message {
            if let Some(channel) = &self.options.channel {
                let mut msg = message.to_string();
                // "\n" is added to the end of the message to make it easier to
                // allow the clients know that the message is complete.
                msg.push_str("\n");
                self.client.publish(channel.to_string(), msg.to_string());
            } else {
                self.client.send(message.to_string());
            }
        }
    }

    // Handles continuous listening for messages. All messages will be printed
    // to the stdout.
    fn handle_listening(&mut self, force_true: bool) {
        if self.options.listen || force_true {
            self.client.listen(|message| println!("{}", message));
        }
    }

    // Handles receiving messages. If the user specified the `recv` option,
    // then the client will listen for messages and print them to the console.
    fn handle_receiving(&mut self, force_true: bool) {
        if self.options.recv || force_true {
            self.client.receive(|message| println!("{}", message));
        }
    }
}
