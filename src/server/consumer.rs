use super::state;
use std::{
    io::{Read, Write},
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
