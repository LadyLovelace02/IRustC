/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io::{self, Write}, net::TcpStream};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

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


/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            messages: Vec::new(),
        }
    }
}

impl App {
    /// Connect to the server
    fn connect_to_server(ip: String, port: String) {
        let connection_ip_and_port: String = format!("{}:{}", ip, port);
        if let Ok(stream) = TcpStream::connect(connection_ip_and_port) {
            println!("CONNECTED"); //testing connection
        } else {
            println!("CONNECTION FAILED");
        }
    }

    fn send_a_message(stream: TcpStream, message: Message) {
        // stream.write(serde_json::to_string(&message).unwrap());//send a message to the server
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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
    
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
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
enum ConnectionError {
    Failed
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                // When the enter key is pressed, send the message to the server
                KeyCode::Enter => {
                    app.messages.push(app.input.drain(..).collect());
                }
                // If any character key is pressed, add it to a buffer
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                // Escape exits the application
                KeyCode::Esc => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // There will be 2 chu
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let mut text = Text::from(Spans::from(vec![
        Span::raw("Press "),
        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to exit"),
    ]));

    text.patch_style(Style::default());
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    // Show the cursor
    f.set_cursor(
        // Put cursor past the end of the input text
        chunks[2].x + app.input.width() as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[2].y + 1,
    );

    // Render the messages
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();

    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));

    f.render_widget(messages, chunks[1]);

    // Render the input box
    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, chunks[2]);
}
