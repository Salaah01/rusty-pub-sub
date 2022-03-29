/**
 * Contains a hashmap of the clients and their subscriptions.
 */
use std::collections::{HashSet, Hashmap};
use std::net::TcpStream;

// A client is a TCP Stream and a set of subscriptions.
static mut _clients: Hashmap<TcpStream, HashSet<String>> = Hashmap::new();

// A subscription is a channel and a set of clients.
static mut _subscriptions: Hashmap<String, HashSet<TcpStream>> = Hashmap::new();

pub struct Client {}

impl Client {
    /**
     * Adds a client to the hashmap of clients.
     *
     * @param client The client to add.
     */
    fn add_client(stream: TcpStream) {
        // Check if the client is already in the hashmap
        if _clients.contains_key(&stream) {
            return;
        }

        // Add the client to the hashmap
        _clients.insert(stream, HashSet::new());
    }

    /**
     * Removes a client from the hashmap of clients.
     *
     * @param client The client to remove.
     */
    fn remove_client(stream: TcpStream) {
        // Check if the client is in the hashmap
        if !_clients.contains_key(&stream) {
            return;
        }

        // Remove all subscriptions for the client
        let subscriptions = _clients.get(&stream).unwrap();
        for subscription in subscriptions.iter() {
            Subscription::remove_subscription(subscription, &stream);
        }

        // Remove the client from the hashmap
        _clients.remove(&stream);
    }
}

pub struct Subscription {}

impl Subscription {
    /**
     * Subscribe a client to a channel.
     * @param client The client to subscribe.
     * @param channel The channel to subscribe to.
     */
    fn add_subscription(client: TcpStream, channel: String) {
        // Check if the a key for the channel already exists. If not create it.
        if !_subscriptions.contains_key(&channel) {
            _subscriptions.insert(channel, HashSet::new());
        }

        // Add the client to the channel's set of clients.
        _subscriptions.get_mut(&channel).unwrap().insert(client);
    }

    /**
     * Unsubscribe a client from a channel.
     * @param client The client to unsubscribe.
     * @param channel The channel to unsubscribe from.
     */
    fn remove_subscription(client: TcpStream, channel: String) {
        // Check if the channel is in the subscriptions set of channels.
        if !_subscriptions.contains_key(&channel) {
            return;
        }

        // Remove the client from the subscriptions if it the client exists.
        _subscriptions.get_mut(&channel).unwrap().remove(&client);
    }
}
