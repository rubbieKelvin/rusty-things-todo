use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};

pub struct TextInputState {
    pub text: String,
    title: Option<String>,
    focused: bool,
    pub placeholder: String,
}

impl TextInputState {
    pub fn default() -> Self {
        return TextInputState {
            title: None,
            text: String::new(),
            focused: false,
            placeholder: String::from("Enter text..."),
        };
    }

    pub fn set_placeholder(mut self, text: &str) -> Self {
        self.placeholder.clear();
        self.placeholder.push_str(text);
        return self;
    }

    pub fn set_title(mut self, text: &str) -> Self {
        self.title = Some(String::from(text));
        return self;
    }

    pub fn set_focused(&mut self, state: bool) {
        self.focused = state;
    }
}

pub struct TextInput;

impl TextInput {
    pub fn new() -> Self {
        return TextInput;
    }
}

impl StatefulWidget for &TextInput {
    type State = TextInputState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(if state.focused {
                Color::Yellow
            } else {
                Color::White
            }));

        let block = if let Some(title) = &state.title {
            block.title_top(title.as_str())
        } else {
            block
        };

        let text = Paragraph::new(if state.text.is_empty() {
            state.placeholder.clone()
        } else {
            state.text.clone()
        })
        .block(block)
        .fg(if state.text.is_empty() {
            Color::DarkGray
        } else {
            Color::White
        });

        text.render(area, buf);
    }
}
