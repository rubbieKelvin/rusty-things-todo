mod app;
mod widgets;
use crate::app::Application;

fn main() -> std::io::Result<()> {
    let mut teminal = ratatui::init();
    teminal.clear()?;

    let mut app = Application::new();
    let result = app.run(&mut teminal);

    ratatui::restore();

    return result;
}