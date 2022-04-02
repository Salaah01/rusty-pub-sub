use lazy_static::lazy_static;
/// Contains a hashmap of the clients and their subscriptions.
use std::collections::{HashMap, HashSet};
use std::net::TcpStream;
use std::sync::Mutex;

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<String, HashSet<String>>> = Mutex::new(HashMap::new());
}

lazy_static! {
    static ref SUBSCRIPTIONS: Mutex<HashMap<String, HashSet<String>>> = Mutex::new(HashMap::new());
}

/// Get a hash-able string from a TcpStream.
fn get_client_id(stream: &TcpStream) -> String {
    let addr = stream.peer_addr().unwrap();
    format!("{}:{}", addr.ip(), addr.port())
}

pub struct Client {}

impl Client {
    /// Checks if a client is already registered.
    fn is_registered(&self, client: &TcpStream) -> bool {
        CLIENTS.lock().unwrap().contains_key(&get_client_id(client))
    }

    /// Adds a client to the hashmap of clients.
    ///
    /// @param client The client to add.
    ///
    pub fn add_client(&self, client: &TcpStream) {
        // Check if the client is already in the hashmap
        if self.is_registered(&client) {
            return;
        }

        // Add the client to the hashmap
        CLIENTS
            .lock()
            .unwrap()
            .insert(get_client_id(&client), HashSet::new());
    }

    /// Removes a client from the hashmap of clients.
    ///
    /// @param client The client to remove.
    ///
    pub fn remove_client(&self, stream: &TcpStream) {
        // Check if the client is in the hashmap (this is unsafe)
        if !self.is_registered(&stream) {
            return;
        }

        let client_id = get_client_id(&stream);
        let mut clients = CLIENTS.lock().unwrap();

        // Remove the client from the hashmap
        let subscribed_channels = clients.get(&client_id).unwrap();
        let subscription = Subscription {};

        // Remove all subscriptions for the client
        for channel in subscribed_channels.iter() {
            subscription.remove_subscription(&stream, &channel);
        }

        // Remove the client from the hashmap
        clients.remove(&client_id);
    }
}

pub struct Subscription {}

impl Subscription {
    /// Checks if a subscription is already registered.
    /// @param channel The channel to check.
    ///
    fn is_channel_registered(&self, channel: &String) -> bool {
        SUBSCRIPTIONS.lock().unwrap().contains_key(channel)
    }

    /// Subscribe a client to a channel.
    /// @param client The client to subscribe.
    /// @param channel The channel to subscribe to.
    ///
    pub fn add_subscription(&self, client: &TcpStream, channel: &String) {
        let mut subscriptions = SUBSCRIPTIONS.lock().unwrap();

        // Check if the a key for the channel already exists. If not create it.
        if !self.is_channel_registered(&channel) {
            subscriptions.insert(channel.to_string(), HashSet::new());
        }

        // Add the client to the channel's set of clients.
        subscriptions
            .get_mut(channel)
            .unwrap()
            .insert(get_client_id(&client));
    }

    /// Unsubscribe a client from a channel.
    /// @param client The client to unsubscribe.
    /// @param channel The channel to unsubscribe from.
    pub fn remove_subscription(&self, client: &TcpStream, channel: &String) {
        // Check if the channel is in the subscriptions set of channels.
        if !self.is_channel_registered(&channel) {
            return;
        }

        // Remove the client from the subscriptions if it the client exists.
        SUBSCRIPTIONS
            .lock()
            .unwrap()
            .get_mut(channel)
            .unwrap()
            .remove(&get_client_id(&client));
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;

    /// Test that we are able to get a string presentation of a client.
    #[test]
    fn test_add_client() {
        let client = TcpStream::connect("localhost:8080").unwrap();
        let client_id = get_client_id(&client);
        assert!(client_id.is_ascii());
    }
}

/// Client specific tests
#[cfg(test)]
mod client_tests {
    use super::*;
    use std::net::TcpStream;

    /// Helper function to create a client.
    fn get_client() -> TcpStream {
        TcpStream::connect("localhost:8080").unwrap()
    }

    /// Helper function to clear the client hashmap.
    fn flush_clients_hashmap() {
        let mut clients = CLIENTS.lock().unwrap();
        clients.clear();
        drop(clients);
    }

    /// Test that the `is_registered` function returns false if the client has
    /// not been registered.
    #[test]
    fn test_is_not_registered() {
        flush_clients_hashmap();
        assert!(!Client {}.is_registered(&get_client()));
    }

    /// Test that the `is_registered` function returns true if the client has
    /// been registered.
    #[test]
    fn test_is_registered() {
        let client = get_client();
        flush_clients_hashmap();
        Client {}.add_client(&client);
        assert!(Client {}.is_registered(&client));
    }

    /// Test the `add_client` function. It should register the client and add
    /// it to the hashmap of clients.
    #[test]
    fn test_add_client() {
        let client = get_client();
        flush_clients_hashmap();
        Client {}.add_client(&client);
        assert!(CLIENTS
            .lock()
            .unwrap()
            .contains_key(&get_client_id(&client)));
    }

    /// Test the `remove_client` function. It should remove the client from the
    /// hashmap of clients.
    #[test]
    fn test_remove_client() {
        let client = get_client();
        flush_clients_hashmap();
        Client {}.add_client(&client);
        Client {}.remove_client(&client);
        assert!(!CLIENTS
            .lock()
            .unwrap()
            .contains_key(&get_client_id(&client)));
    }
}

/// Subscription specific tests
#[cfg(test)]
mod subscription_tests {
    use super::*;
    use std::net::TcpStream;

    /// Helper function to create a client.
    fn get_client() -> TcpStream {
        TcpStream::connect("localhost:8080").unwrap()
    }

    /// Helper function to create a channel.
    fn get_channel(channel_name: Option<&str>) -> String {
        match channel_name {
            Some(channel_name) => channel_name.to_string(),
            None => "test_channel".to_string(),
        }
    }

    /// Helper function to clear the subscriptions hashmap.
    fn flush_subscriptions_hashmap() {
        let mut subscriptions = SUBSCRIPTIONS.lock().unwrap();
        subscriptions.clear();
        // Drop the guard
        drop(subscriptions);
    }

    /// Test that the `is_channel_registered` function returns false if the
    /// channel has not been registered.
    #[test]
    fn test_is_not_channel_registered() {
        flush_subscriptions_hashmap();
        assert!(!Subscription {}.is_channel_registered(&get_channel(Some("unregistered_channel"))));
    }

    /// Test that the `is_channel_registered` function returns true if the
    /// channel has been registered.
    #[test]
    fn test_is_channel_registered() {
        flush_subscriptions_hashmap();
        let channel: String = get_channel(None);
        SUBSCRIPTIONS
            .lock()
            .unwrap()
            .insert(channel.clone(), HashSet::new());
        assert!(Subscription {}.is_channel_registered(&channel));
    }

    /// Test the `add_subscription` function. It should add the client to the
    /// channel's set of clients.
    #[test]
    fn test_add_subscription() {
        let client = get_client();
        let channel: String = get_channel(None);
        // flush_subscriptions_hashmap();
        Subscription {}.add_subscription(&client, &channel);
        assert!(SUBSCRIPTIONS
            .lock()
            .unwrap()
            .get(&channel)
            .unwrap()
            .contains(&get_client_id(&client)));
    }
}
