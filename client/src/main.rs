use client::Client;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self},//used to have Write
    net::TcpStream,
};
use tui::{
    backend::{CrosstermBackend}, Terminal,//used to have Backend
};


use common::Message;

// use std::io::prelude::*;

pub mod app;
pub mod ui;
pub mod client;

use app::run_app;

use std::sync::{Arc, Mutex};



/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: Arc<Mutex<Vec<String>>>,
}

impl App {
    /// Create a new App
    fn new(messages: Arc<Mutex<Vec<String>>>) -> App {
        App {
            input: String::new(),
            messages,
        }
    }

    /// Connect to the server
    fn connect_to_server(ip: String, port: String) {
        let connection_ip_and_port: String = format!("{}:{}", ip, port);
        if let Ok(_stream) = TcpStream::connect(connection_ip_and_port) {
            println!("CONNECTED"); //testing connection
        } else {
            println!("CONNECTION FAILED");
        }
    }

    fn send_network_message(_stream: TcpStream, _message: Message) {
        // stream.write(serde_json::to_string(&message).unwrap());//send a message to the server
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let messages = Arc::new(Mutex::new(Vec::new()));

    // Start the client thread
    tokio::spawn(async move {    
        // connect to the server
        let mut client: Client = Client::connect_to_server("127.0.0.1".to_string(), "2234".to_string(), messages.clone())
            .expect("error connecting to server");
    });




    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(messages.clone());
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

#[derive(Debug)]
pub enum ConnectionError {
    Failed,
}



