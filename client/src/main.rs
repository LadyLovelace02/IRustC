use common::Message;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::TcpStream;
fn connect_to_server(ip: String, port: String) {
    let connection_ip_and_port: String = format!("{}:{}", ip, port);
    if let Ok(stream) = TcpStream::connect(connection_ip_and_port) {
        println!("CONNECTED"); //testing connection
    } else {
        println!("CONNECTION FAILED");
    }
}

fn send_a_message(mut stream: TcpStream, message: Message) {
    stream.write(&serde_json::to_string(&message).unwrap().as_bytes()); //send a message to the server
}

fn main() {
    connect_to_server("127.0.0.1".to_string(), "2234".to_string());
    println!("Hello, world!");
}
