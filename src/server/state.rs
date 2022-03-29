use lazy_static::lazy_static;
/**
 * Contains a hashmap of the clients and their subscriptions.
 */
use std::collections::{HashMap, HashSet};
use std::net::TcpStream;
use std::sync::Mutex;

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<String, HashSet<String>>> = Mutex::new(HashMap::new());
}

lazy_static! {
    static ref SUBSCRIPTIONS: Mutex<HashMap<String, HashSet<String>>> = Mutex::new(HashMap::new());
}

/**
 * Get a hash-able string from a TcpStream.
 */
fn get_client_id(stream: &TcpStream) -> String {
    let addr = stream.peer_addr().unwrap();
    format!("{}:{}", addr.ip(), addr.port())
}

pub struct Client {}

impl Client {
    /**
     * Checks if a client is already registered.
     */
    fn is_registered(&self, client: &TcpStream) -> bool {
        CLIENTS.lock().unwrap().contains_key(&get_client_id(client))
    }

    /**
     * Adds a client to the hashmap of clients.
     *
     * @param client The client to add.
     */
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

    /**
     * Removes a client from the hashmap of clients.
     *
     * @param client The client to remove.
     */
    pub fn remove_client(&self, stream: TcpStream) {
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
    /**
     * Checks if a subscription is already registered.
     * @param channel The channel to check.
     */
    fn is_channel_registered(&self, channel: &String) -> bool {
        SUBSCRIPTIONS.lock().unwrap().contains_key(channel)
    }

    /**
     * Subscribe a client to a channel.
     * @param client The client to subscribe.
     * @param channel The channel to subscribe to.
     */
    pub fn add_subscription(&self, client: TcpStream, channel: &String) {
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
    /**
     * Unsubscribe a client from a channel.
     * @param client The client to unsubscribe.
     * @param channel The channel to unsubscribe from.
     */
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
