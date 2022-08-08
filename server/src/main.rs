use common::Message;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use std::{io::prelude::*, net::TcpListener};

use std::{
    error::Error,
    io,
    net::TcpStream,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

struct Room {
    pub name: String,
    pub current_active_users: Vec<String>,
    pub message_table: String, //something to refer to table in sql database
}
struct App {
    scroll: u16,
}

impl App {
    fn new() -> App {
        App { scroll: 0 }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }
}
struct Connection{
    stream: TcpStream,
}

impl Connection{
    fn accept_connection(&self, ip: String) {
        let listener = TcpListener::bind(ip).unwrap();
        match listener.accept() {
            Ok((_socket, addr)) => println!("new client: {addr:?}"),
            Err(e) => println!("couldn't connect to client: {e:?}"),
        }
    }

    fn recieve_message(&self, stream: &mut TcpStream) {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        let message: Message = serde_json::from_str(&mut buffer).unwrap();
        print!("Message Recieved: ");
        println!("{}", message.content);
    }

    fn send_client_data(&self, stream: &mut TcpStream, message: Message) {
        stream.write(&serde_json::to_string(&message).unwrap().as_bytes()); //send a message to the server
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a message
    Message {
        id: 1,
        room: "test".to_string(),
        name: "Alice".to_string(),
        content: "Hello, world!".to_string(),
        timestamp: 123456789,
    };
    let listener = TcpListener::bind("127.0.0.1:2234".to_string())?;
    loop {
        let (mut socket, _) = listener.accept()?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket .read(&mut buf)
                    .expect("failed to read data from socket");
                if n == 0 {
                    return;
                }
                socket 
                    .write_all(&buf[0..n])
                    .expect("failed to write data from socket");
            }
        });
    }
    return Ok(());

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(size);

    let text = vec![
        Spans::from("This is a line "),
        Spans::from(Span::styled(
            "This is a line   ",
            Style::default().fg(Color::Red),
        )),
        Spans::from(Span::styled(
            "This is a line",
            Style::default().bg(Color::Blue),
        )),
        Spans::from(Span::styled(
            "This is a longer line",
            Style::default().add_modifier(Modifier::CROSSED_OUT),
        )),
        Spans::from(Span::styled(&long_line, Style::default().bg(Color::Green))),
        Spans::from(Span::styled(
            "This is a line",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Left, no wrap"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Left, wrap"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Center, wrap"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .scroll((app.scroll, 0));
    f.render_widget(paragraph, chunks[2]);
    let paragraph = Paragraph::new(text)
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Right, wrap"))
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[3]);
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use entity::message;
    use sea_orm::EntityTrait;

    use sea_orm::{Database, DatabaseConnection, Set};

    #[tokio::test]
    /// Test creating 100 messages
    async fn create_messages() {
        // Add the env file variables
        dotenv().ok();

        // Get the database url from the env file
        let db_url = std::env::var("DATABASE_URL").unwrap();

        // Connect to the database
        let db: DatabaseConnection = Database::connect(db_url).await.unwrap();

        // Creade 100 messages
        let messages = (0..100)
            .map(|_| message::ActiveModel {
                text: Set("Hello, world!".to_string()),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        // Insert the messages into the database
        message::Entity::insert_many(messages)
            .exec(&db)
            .await
            .unwrap();
    }
}
