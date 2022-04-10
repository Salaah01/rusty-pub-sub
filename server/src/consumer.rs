//! # Server Consumers
//! This library is responsible for handling the communication between the
//! client and the server.
//! It contains a `consumer` function that is responsible for listening
//! handling messages from the client and passing them onto the right function
//! to handle them.

use super::state;
use std::{
    io::{BufWriter, Read, Write},
    net::TcpStream,
};

/// A consumer for handling incoming messages. This is done by calling other
/// functions to handle the message.
/// # Arguments
/// * `client` - The stream to read from.
pub fn consumer(client: &mut TcpStream) {
    let mut connected = true;

    while connected {
        let mut buffer = [0; 64];
        client.read(&mut buffer).unwrap();

        if is_buffer_empty(&buffer) {
            continue;
        }

        // Create a buffer to hold the message. As we know the message length
        // from the initial message, we can create a buffer of the correct
        // size.
        let message_length = match String::from_utf8(buffer.to_vec()) {
            Ok(message) => match message.trim().parse::<usize>() {
                Ok(length) => length,
                Err(e) => {
                    println!("Error: {}", e);
                    client.flush().unwrap();
                    clear_buffer(&mut buffer);
                    continue;
                }
            },
            Err(_) => {
                println!("Error: Failed to parse message length.");
                client.flush().unwrap();
                clear_buffer(&mut buffer);
                continue;
            }
        };

        // Create a buffer to hold the message.
        let mut message = vec![0; message_length.into()];

        // Convert the message to a string.
        client.read(&mut message).unwrap();

        let [handler, message] =
            get_message_components(String::from_utf8(message).unwrap().as_str());

        match handler.as_str() {
            "SUBSCRIBE" => subscribe_handler(&client, &message),
            "UNSUBSCRIBE" => unsubscribe_handler(&client, &message),
            "DISCONNECT" => {
                connected = false;
                disconnect_handler(&client);
            }
            "PUBLISH" => publish_handler(&message),
            "PING" => ping_handler(&client),
            _ => println!("Unknown command: {}", handler),
        }
        client.flush().unwrap();
        // Empty the buffer
        clear_buffer(&mut buffer);
    }
    println!("Client disconnected.");
}

/// Splits a message returning the method and the message.
fn get_message_components(message: &str) -> [String; 2] {
    let mut message_parts = message.split(" ");
    let method = message_parts.next().unwrap();
    let msg = message_parts.collect::<Vec<&str>>().join(" ");

    [method.to_string(), msg.to_string()]
}

/// Checks if buffer is in it's empty state.
/// # Arguments
/// * `buffer` - The buffer to check.
/// # Returns
/// * `bool` - Whether the buffer is empty.
fn is_buffer_empty(buffer: &[u8]) -> bool {
    buffer.iter().all(|&x| x == 0)
}

/// Set the buffer to the empty state.
/// # Arguments
/// * `buffer` - The buffer to set.
fn clear_buffer(buffer: &mut [u8; 64]) {
    for i in 0..64 {
        buffer[i] = 0;
    }
}

/// Subscribes a client to a channel.
/// # Arguments
/// * `client` - The client to subscribe.
/// * `channel` - The channel to subscribe to.
fn subscribe_handler(client: &TcpStream, channel: &String) {
    println!("Subscribing to channel: {}", channel);
    state::Subscription {}.add_subscription(&client, &channel)
}

/// Unsubscribes a client from a channel.
/// # Arguments
/// * `client` - The client to unsubscribe.
/// * `channel` - The channel to unsubscribe from.
fn unsubscribe_handler(client: &TcpStream, channel: &String) {
    println!("Unsubscribing from channel: {}", channel);
    state::Subscription {}.remove_subscription(&client, &channel)
}

/// Removes a client from the collection of clients.
/// # Arguments
/// * `client` - The client to disconnect.
/// * `channel` - The channel to disconnect from.
fn disconnect_handler(client: &TcpStream) {
    println!("{} from {}", "DISCONNECT", client.peer_addr().unwrap());
    state::Client {}.remove_client(client);
}

/// Publishes a messages to all clients subscribed to a channel.
/// # Arguments
/// * `channel` - The channel to publish to.
/// * `message` - The message to publish.
fn publish_handler(message: &String) {
    // The message could contain both the channel name and the actual message.
    // We need to split the message into two parts.
    let msg_split_point = match message.find(" ") {
        Some(point) => point,
        None => {
            println!("Error: Failed to parse message.");
            return;
        }
    };

    let channel = &message[0..msg_split_point];
    let message = &message[msg_split_point + 1..];

    let subscribers = state::Subscription {}.get_subscribers(&channel.to_string());

    // If there are no subscribers, we can return early.
    if subscribers.is_empty() {
        return;
    }

    let message_length = format!("{}", message.len()).to_string();
    let mut msg_size_buffer = [0; 64];

    msg_size_buffer[0..message_length.len()].copy_from_slice(message_length.as_bytes());
    msg_size_buffer[message_length.len()..]
        .copy_from_slice(" ".repeat(64 - message_length.len()).as_bytes());

    let msg_bytes = message.as_bytes();

    for subscriber in subscribers {
        // A subscriber is a memory address, so we need to convert it to a
        // TcpStream.
        let stream = subscriber.parse::<usize>().unwrap() as *mut TcpStream;
        let stream = unsafe { &mut *stream };

        let mut writer = BufWriter::new(&*stream);

        match writer.write_all(msg_bytes) {
            Ok(_) => (),
            Err(_) => state::Subscription {}.remove_subscription(stream, &channel.to_string()),
        };

        // Flush and close the stream.
        match writer.flush() {
            Ok(_) => (),
            Err(_) => state::Subscription {}.remove_subscription(stream, &channel.to_string()),
        };
    }
}

/// Server ping. Responds with a PONG message.
/// # Arguments
/// * `client` - The client to ping.
fn ping_handler(client: &TcpStream) {
    println!("Got ping from {}", client.peer_addr().unwrap());
    let mut writer = BufWriter::new(client);
    match writer.write_all(b"PONG\n") {
        Ok(_) => (),
        Err(_) => println!("WARNING: Failed to write to client."),
    };
    match writer.flush() {
        Ok(_) => (),
        Err(_) => println!("WARNING: Failed to flush writer."),
    };
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};

    /// Helper function to create a client (`TcpStream`).
    fn get_client() -> TcpStream {
        TcpStream::connect("localhost:8080").unwrap()
    }

    /// Test that the function is able to split a message into it's components.
    #[test]
    fn test_get_message_components() {
        let [method, message] = get_message_components("SUBSCRIBE test");
        assert_eq!(method, "SUBSCRIBE");
        assert_eq!(message, "test");
    }

    /// Test that the function is able to split a message into multiple
    /// components even when the message has spaces in it.
    #[test]
    fn test_get_message_components_multi_space() {
        let [method, message] = get_message_components("SUBSCRIBE test channel");
        assert_eq!(method, "SUBSCRIBE");
        assert_eq!(message, "test channel");
    }

    /// Test that the function is able to correctly identify an empty buffer.
    #[test]
    fn test_is_buffer_empty_true() {
        let buffer = [0; 64];
        assert!(is_buffer_empty(&buffer));
    }

    /// Test that the function is able to correctly identify a non-empty
    /// buffer.
    #[test]
    fn test_is_buffer_empty_false() {
        let mut buffer = [0; 64];
        buffer[0] = 1;
        assert!(!is_buffer_empty(&buffer));
    }

    /// Test that the function is able to clear a buffer.
    #[test]
    fn test_clear_buffer() {
        let mut buffer = [0; 64];
        buffer[0] = 1;
        clear_buffer(&mut buffer);
        assert!(buffer.iter().all(|&x| x == 0));
    }

    /// Test that the function is to subscribe a client to a channel.
    #[test]
    fn test_subscribe_handler() {
        let client = get_client();
        let channel = "test".to_string();
        subscribe_handler(&client, &channel);
        assert!(state::Subscription {}.is_subscribed(&client, &channel));
    }

    /// Test that the function is to unsubscribe a client from a channel.
    #[test]
    fn test_unsubscribe_handler() {
        let client = get_client();
        let channel = "test".to_string();
        state::Subscription {}.add_subscription(&client, &channel);
        unsubscribe_handler(&client, &channel);
        assert!(!state::Subscription {}.is_subscribed(&client, &channel));
    }

    /// Test that the function is to disconnect a client.
    #[test]
    fn test_disconnect_handler() {
        let client = get_client();
        state::Client {}.add_client(&client);
        disconnect_handler(&client);
        assert!(!state::Client {}.is_registered(&client));
    }
}
