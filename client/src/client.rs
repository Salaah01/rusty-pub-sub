//! Client Lib
//! This library provides a way for a client to connect and communicate with
//! the server.

use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    str::FromStr,
};

/// Represents a client connected to the server.
/// # Arguments
/// * `host` - The hostname of the client.
/// * `port` - The port of the client.
/// * `connection` - The TCP connection to the client.
#[derive(Debug)]
pub struct Client {
    host: String,
    port: u16,
    connection: Option<TcpStream>,
}

impl FromStr for Client {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let host = parts.next().unwrap().to_string();
        let port = parts
            .next()
            .ok_or_else(|| "Could not parse host:port".to_string())?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;
        Ok(Client {
            host,
            port,
            connection: None,
        })
    }
}

impl Client {
    /// Creates a new client instance and connects to the server.
    /// # Arguments
    /// * `host` - The hostname of the client.
    /// * `port` - The port of the client.
    /// # Returns
    /// A new client instance.
    pub fn new(host: String, port: u16) -> Client {
        let mut client = Client {
            host,
            port,
            connection: None,
        };
        client.connect();
        client
    }

    /// Connects to the server.
    pub fn connect(&mut self) {
        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port).as_str()).unwrap();
        self.connection = Some(stream);
        if self.connection.is_none() {
            panic!("Failed to connect to server.");
        }
    }

    /// Disconnects from the server.
    pub fn disconnect(&mut self) {
        self.send(format!("DISCONNECT").as_str().to_string());
    }

    /// Pings the server
    pub fn ping(&mut self) -> Result<(), Box<dyn Error>> {
        self.send(format!("PING").as_str().to_string());
        let mut buffer = String::new();
        let mut reader = BufReader::new(self.connection.as_mut().unwrap());
        match reader.read_line(&mut buffer) {
            Ok(_) => {
                if buffer.trim() == "PONG" {
                    Ok(())
                } else {
                    Err("Got a back response back from the server".into())
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Sends a message to the server.
    /// # Arguments
    /// * `message` - The message to send.
    /// # Returns
    /// * `bool` - Whether the message was sent successfully.
    pub fn send(&mut self, message: String) -> bool {
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
    pub fn receive(&mut self, callback: fn(&String)) {
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
    pub fn subscribe(&mut self, channel: String) -> bool {
        self.send(format!("SUBSCRIBE {}", channel).as_str().to_string())
    }

    /// Unsubscribes from a channel.
    /// # Arguments
    /// * `channel` - The channel to unsubscribe from.
    /// # Returns
    /// * `bool` - Whether the unsubscription was successful.
    pub fn unsubscribe(&mut self, channel: String) -> bool {
        self.send(format!("UNSUBSCRIBE {}", channel).as_str().to_string())
    }

    /// Publishes a message to a channel.
    /// # Arguments
    /// * `channel` - The channel to publish to.
    /// * `message` - The message to publish.
    /// # Returns
    /// * `bool` - Whether the publish was successful.
    pub fn publish(&mut self, channel: String, message: String) -> bool {
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
    pub fn listen(&mut self, callback: fn(&String)) {
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
