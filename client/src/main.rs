use common::Message;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpStream;

struct Client {
    stream: TcpStream
}
impl Client {
    /// initialize a connection to the server at `ip` and `port`
    /// returns an error if the connection failed
    fn connect_to_server(ip: String, port: String) -> Result<Self, ConnectionError> {
        let connection_ip_and_port: String = format!("{}:{}", ip, port);
        if let Ok(stream) = TcpStream::connect(connection_ip_and_port) {
            println!("CONNECTED"); //testing connection
            // return a client with the new connection
            return Ok(Client { stream })
        } else {
            println!("CONNECTION FAILED");
            // return an error saying we failed
            return Err(ConnectionError::Failed)
        }
    }

    // after looking, this can probably be removed, unless theres a reason to be sending a message with this client, but to a different tcp stream
    fn send_a_message(&self, stream: &mut TcpStream, message: Message) {
        stream.write(&serde_json::to_string(&message).unwrap().as_bytes()); //send a message to the server
    }

    /// use this to send messages to this client
    fn send_a_message_to_self(&mut self, message: Message) {
        self.stream.write(&serde_json::to_string(&message).unwrap().as_bytes()); //send a message to the server
    }
}

#[derive(Debug)]
enum ConnectionError {
    Failed
}


fn main() {
    let test_message:Message = Message {
        id: 1,
        room: "test".to_string(),
        name: "Alice".to_string(),
        content: "Hello, world!".to_string(),
        timestamp: 123456789,
    };

    // connect to the server
    let mut client:Client = Client::connect_to_server("127.0.0.1".to_string(), "2234".to_string()).expect("error connecting to server");
    // send a message to the client
    client.send_a_message_to_self(test_message);
}