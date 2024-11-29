use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{style::Stylize, widgets::Paragraph, DefaultTerminal};

const EXIT_KEYS: [char; 2] = ['q', 'Q'];

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        // draw the fucking text in the available frame area
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Fuck you bitch! i wrote this shit in rust.").white();
            frame.render_widget(greeting, frame.area());
        })?;

        // listen for key events
        let mut citer = EXIT_KEYS.iter();
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && loop {
                    match citer.next() {
                        Some(character) => {
                            if key.code == KeyCode::Char(*character) {
                                break true;
                            }
                        }
                        None => {
                            break false;
                        }
                    }
                }
            {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app = run(terminal);
    ratatui::restore();
    return app;
}
