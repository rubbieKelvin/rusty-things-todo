use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use symbols::border;

#[derive(Debug)]
struct Application {
    count: u8,
    exit: bool,
}

impl Application {
    fn default() -> Self {
        return Application {
            count: 0,
            exit: false,
        };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('Q') => self.exit(),
                    KeyCode::Esc => self.exit(),
                    KeyCode::Left => self.decrement(),
                    KeyCode::Right => self.increment(),
                    _ => (),
                }
            }
        }
        return Ok(());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn decrement(&mut self) {
        if self.count != 0 {
            self.count -= 1;
        }
    }
}

impl Widget for &Application {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Counter shit".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            " <Left> ".bold().blue(),
            " Increment ".into(),
            " <Right> ".blue().bold(),
            " Quit ".into(),
            " <Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "value: ".into(),
            self.count.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app = Application::default().run(&mut terminal);
    ratatui::restore();
    return app;
}
