use std::io;

use app::Application;

mod app;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut app = Application::new();
    let res = app.run(terminal);

    ratatui::restore();
    return res;
}
