use crossterm::{
    event::{self, Event, KeyCode},
};
use std::{
    io::{self},
};
use tui::{
    backend::{Backend}, Terminal,
};



use crate::ui::ui;
use crate::App;



pub(crate) fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                // When the enter key is pressed, send the message to the server
                KeyCode::Enter => {
                    app.messages.push(app.input.drain(..).collect());
                    // Send to server
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