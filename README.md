# Rusty Pub-Sub (In Development)
A simple pub-sub implementation in Rust.

Data will be stored in memory and so will be lost if the program is terminated.

There are two main apps in this project: server and client.

The server is a simple TCP server that listens for connections on a port. It has the ability to accept connections and send data to clients.

Client is a simple TCP client that connects to a server and sends data.

The client is able to subscribe to channels and receive publish messages from the server.

The package comes with two binaries, ones for the server, and one for the client.

## Server
The server will listen for connections on a port. It will accept connections and receive/send data to clients.

To start the server:
```
server [host (default=localhost)] [port (default=7878)]
```

## Client
The client is a simple TCP client that connects to a server and sends/receives data.
At present, the client automatically disconnects after sending data.
Alternatively, a client can be used to listen for messages.

```
USAGE:
    client [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -l, --listen     Listens continuously for messages from the server
        --ping       Ping the server
        --recv       Waits to receive a message from the server
    -V, --version    Prints version information

OPTIONS:
    -c, --channel <channel>         Channel on which to send a message. If not
                                    specified, the message will be sent to
                                    the server without a channel specified
        --host <host>               The hostname of the server [default: localhost]
    -m, --msg <message>             Sends a message to the server
    -p, --port <port>               The port of the server [default: 8080]
    -s, --sub <subscribe>...        Channel to subscribe to
        --unsub <unsubscribe>...    Channel to unsubscribe from
```