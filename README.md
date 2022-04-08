# Rusty Pub-Sub (In Development)
A simple pub-sub implementation in Rust.

Data will be stored in memory and so will be lost if the program is terminated.

There are two main apps in this project: server and client.

The server is a simple TCP server that listens for connections on a port. It has the ability to accept connections and send data to clients.

Client is a simple TCP client that connects to a server and sends data.

The client is able to subscribe to channels and receive publish messages from the server.
