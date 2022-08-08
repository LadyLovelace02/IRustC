use common::Message;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpStream;

struct Client {
    stream: TcpStream
}
impl Client {
    fn connect_to_server(&self, ip: String, port: String) {
        let connection_ip_and_port: String = format!("{}:{}", ip, port);
        if let Ok(stream) = TcpStream::connect(connection_ip_and_port) {
            println!("CONNECTED"); //testing connection
        } else {
            println!("CONNECTION FAILED");
        }
    }

    fn send_a_message(&self, stream: &mut TcpStream, message: Message) {
        stream.write(&serde_json::to_string(&message).unwrap().as_bytes()); //send a message to the server
    }
}


fn main() {
    let test_message:Message = Message {
        id: 1,
        room: "test".to_string(),
        name: "Alice".to_string(),
        content: "Hello, world!".to_string(),
        timestamp: 123456789,
    };
    let client:Client;
    client.connect_to_server("127.0.0.1".to_string(), "2234".to_string());
    client.send_a_message(&mut client.stream, test_message);
}
