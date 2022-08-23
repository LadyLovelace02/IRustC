
use std::{
    io::{Write},
    net::TcpStream,
};


use common::{Message, NetworkMessage};



use crate::ConnectionError;

pub struct Client {
    stream: TcpStream,
    messages: Arc<Mutex<Vec<String>>>,
}

impl Client {
    /// initialize a connection to the server at `ip` and `port`
    /// returns an error if the connection failed
    pub fn connect_to_server(ip: String, port: String, messages: Arc<Mutex<Vec<String>>>) -> Result<Self, ConnectionError> {
        let connection_ip_and_port: String = format!("{}:{}", ip, port);
        if let Ok(stream) = TcpStream::connect(connection_ip_and_port) {
            println!("CONNECTED"); //testing connection
                                   // return a client with the new connection
            return Ok(Client { stream, messages });
        } else {
            println!("CONNECTION FAILED");
            // return an error saying we failed
            return Err(ConnectionError::Failed);
        }
    }

    // Send a message to the server
    fn send_network_message(&mut self, message: NetworkMessage) {
        self.stream.write(&serde_json::to_string(&message).unwrap().as_bytes()); //send a message to the server
    }

    fn recv_network_message(&mut self) {
        let mut buffer = String::new();
        self.stream.read_to_string(&mut buffer).unwrap();
        let network_message: NetworkMessage = serde_json::from_str(&mut buffer).unwrap();

        match network_message {
            NetworkMessage::Message(message) => {
                self.messages.lock().unwrap().push(message);
            }
            _ => {
            }
        }
    }

    /// use this to send messages to this client
    pub fn send_a_message_to_self(&mut self, message: Message) {
        self.stream
            .write(&serde_json::to_string(&message).unwrap().as_bytes()); //send a message to the server
    }
}