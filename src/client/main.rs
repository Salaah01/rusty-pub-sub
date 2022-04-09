//! Client
//! This binary handles client connections to the server.
//! It is the main entry point for the client and handles sending and receiving
//! messages to and from the server.

use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

/// Represents a client connected to the server.
/// # Arguments
/// * `host` - The hostname of the client.
/// * `port` - The port of the client.
/// * `connection` - The TCP connection to the client.
struct Client {
    host: String,
    port: u16,
    connection: Option<TcpStream>,
}

impl Client {
    /// Creates a new client instance and connects to the server.
    /// # Arguments
    /// * `host` - The hostname of the client.
    /// * `port` - The port of the client.
    /// # Returns
    /// A new client instance.
    fn new(host: String, port: u16) -> Client {
        let mut client = Client {
            host,
            port,
            connection: None,
        };
        client.connect();
        client
    }

    /// Connects to the server.
    fn connect(&mut self) {
        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port).as_str()).unwrap();
        self.connection = Some(stream);
        if self.connection.is_none() {
            panic!("Failed to connect to server.");
        }
    }

    /// Disconnects from the server.
    fn disconnect(&mut self) {
        self.send(format!("DISCONNECT").as_str().to_string());
    }

    /// Pings the server
    fn ping(&mut self) {
        self.send(format!("PING").as_str().to_string());
        let mut buffer = String::new();
        let mut reader = BufReader::new(self.connection.as_mut().unwrap());
        reader.read_line(&mut buffer).unwrap();
        if buffer.trim() != "PONG" {
            panic!("Failed to receive PONG reply.");
        }
    }

    /// Sends a message to the server.
    /// # Arguments
    /// * `message` - The message to send.
    /// # Returns
    /// * `bool` - Whether the message was sent successfully.
    fn send(&mut self, message: String) -> bool {
        let connection = self.connection.as_mut().unwrap();

        // The server expects an initial message with the length of the message.
        let message_length = message.len().to_string();
        let mut buffer = [0; 64];

        // Add the message to the buffer and pad it with spaces.
        buffer[0..message_length.len()].copy_from_slice(message_length.as_bytes());
        buffer[message_length.len()..]
            .copy_from_slice(" ".repeat(64 - message_length.len()).as_bytes());

        // Send the message length.
        connection.write(&buffer).unwrap();

        // Send the message.
        connection.write(message.as_bytes()).is_ok()
    }

    /// Receives a message from the server.
    /// # Arguments
    /// * `callback` - The callback to call when a message is received.
    /// # Returns
    /// * `String` - The message received from the server.
    fn receive(&mut self, callback: fn(&String)) {
        let mut buffer = String::new();
        let conn = self.connection.as_mut().unwrap();
        let mut reader = BufReader::new(conn);
        reader.read_line(&mut buffer).unwrap();
        buffer.pop();
        callback(&buffer);
        buffer.clear();
    }

    /// Subscribes to a channel.
    /// # Arguments
    /// * `channel` - The channel to subscribe to.
    /// # Returns
    /// * `bool` - Whether the subscription was successful.
    fn subscribe(&mut self, channel: String) -> bool {
        self.send(format!("SUBSCRIBE {}", channel).as_str().to_string())
    }

    /// Unsubscribes from a channel.
    /// # Arguments
    /// * `channel` - The channel to unsubscribe from.
    /// # Returns
    /// * `bool` - Whether the unsubscription was successful.
    fn unsubscribe(&mut self, channel: String) -> bool {
        self.send(format!("UNSUBSCRIBE {}", channel).as_str().to_string())
    }

    /// Publishes a message to a channel.
    /// # Arguments
    /// * `channel` - The channel to publish to.
    /// * `message` - The message to publish.
    /// # Returns
    /// * `bool` - Whether the publish was successful.
    fn publish(&mut self, channel: String, message: String) -> bool {
        self.send(
            format!("PUBLISH {} {}", channel, message)
                .as_str()
                .to_string(),
        )
    }

    /// Listens for messages from the server. Any messages received are passed
    /// to a callback function.
    /// # Arguments
    /// * `callback` - The function to call when a message is received.
    /// # Remarks
    /// This function will listen forever until the client is disconnected.
    fn listen(&mut self, callback: fn(&String)) {
        let mut buffer = String::new();
        let conn = self.connection.as_mut().unwrap();
        let mut reader = BufReader::new(conn);
        loop {
            reader.read_line(&mut buffer).unwrap();
            if buffer.len() > 0 {
                // Remove the newline character.
                buffer.pop();
                callback(&buffer);
            }
            buffer.clear();
        }
    }
}

fn main() {
    let mut client = Client::new("localhost".to_string(), 7878);
    // client.send("Hello, world!".to_string());
    client.ping();
    client.subscribe("test".to_string());
    // client.receive(|message| print!("got single message {}", message));
    // client.unsubscribe("test".to_string());

    // client.subscribe("test2".to_string());
    client.publish("test".to_string(), "Hello world!\n".to_string());
    client.listen(|message| println!("got message {}", message));
    // client.unsubscribe("test".to_string());

    client.disconnect();
}
