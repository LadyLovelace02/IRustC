use std::io::prelude::*;
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use common::Message;
fn connect_to_server(ip: String, port: String) {
    let connection_ip_and_port: String = format!("{}:{}", ip, port);
    if let Ok(stream) = TcpStream::connect(connection_ip_and_port) {
        println!("CONNECTED");//testing connection
    }
    else {
        println!("CONNECTION FAILED");
    }
}

fn send_a_message(stream: TcpStream, message: Message) {
    stream.write(serde_json::to_string(&message).unwrap());//send a message to the server
}

fn main() {
    println!("Hello, world!");
}
