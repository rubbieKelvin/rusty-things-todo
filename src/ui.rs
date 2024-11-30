use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, StatefulWidget, Widget},
};

use crate::app::Application;

pub fn render_main_ui(area: Rect, buffer: &mut Buffer, app: &mut Application) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    // Create the title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new("A rusty app")
        .fg(Color::Yellow)
        .block(title_block)
        .centered();

    // Create the body
    let body_border = Block::default().borders(Borders::ALL);
    let items: Vec<ListItem> = app
        .todos
        .list
        .iter()
        .map(|td| ListItem::from(td.text.clone()))
        .collect();
    let list = List::new(items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow))
        .block(body_border);

    // Footer
    let footer_chunk_items: [&str; 4] = [
        "New todo <N>",
        "Delete todo <Del>",
        "Check/Uncheck <Return>",
        "Quit <Q> or <Esc>",
    ];

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            (0..footer_chunk_items.len())
                .map(|_x| Constraint::Percentage((100 / footer_chunk_items.len()) as u16)),
        )
        .split(chunks[2]);

    for (i, text_string) in footer_chunk_items.iter().enumerate() {
        let blk = Block::default().borders(Borders::ALL);
        let text = Paragraph::new(*text_string)
            .fg(Color::Yellow)
            .block(blk)
            .centered();
        text.render(footer_chunks[i], buffer);
    }

    title.render(chunks[0], buffer);
    // frame.render_widget(body_border, chunks[1]);
    StatefulWidget::render(list, chunks[1], buffer, &mut app.todos.state);
}

pub fn render_editing_ui(area: Rect, buffer: &mut Buffer, app: &Application) {
    let placeholder = String::from("Enter a text...");

    let title = Line::from(vec!["Create a new todo!".into()]).fg(Color::Yellow);
    let footer = Line::from(vec![
        "Save".into(),
        "<Enter> ".fg(Color::Blue).bold(),
        "Discard".into(),
        "<Esc>".fg(Color::Blue).bold(),
    ]);

    let block = Block::default()
        .title_top(title)
        .title_bottom(footer)
        .borders(Borders::ALL)
        .padding(Padding::uniform(2));
    let text = Paragraph::new(if app.todo_input_text.len() > 0 {
        app.todo_input_text.clone()
    } else {
        placeholder
    })
    .fg(if app.todo_input_text.len() > 0 {
        Color::Yellow
    } else {
        Color::Gray
    })
    .block(block);

    text.render(area, buffer);
}
