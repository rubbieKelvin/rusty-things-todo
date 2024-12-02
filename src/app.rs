use crate::widgets::input::{TextInput, TextInputState};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListState, StatefulWidget};
use ratatui::DefaultTerminal;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: String,
    content: String,
    checked: bool,
}

impl Todo {
    fn new(content: &str) -> Self {
        return Todo {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.to_owned(),
            checked: false,
        };
    }
}

pub struct Application {
    running: bool,
    current_text_input: usize,
    text_input_states: Vec<TextInputState>,
    todo_list: Vec<Todo>,
    todo_list_state: ListState,
}

impl Application {
    pub fn new() -> Application {
        return Application {
            running: true,
            current_text_input: 1,
            todo_list: vec![],
            todo_list_state: ListState::default(),
            text_input_states: vec![
                TextInputState::default()
                    .set_placeholder("Search todo by title")
                    .set_title("[Search]"),
                TextInputState::default().set_placeholder("What is your name?"),
            ],
        };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| {
                frame.render_widget(&mut *self, frame.area());
            })?;

            let events = event::read()?;
            self.handle_events(&events);

            if !self.running {
                return Ok(());
            }
        }
    }

    fn handle_events(&mut self, events: &Event) {
        if let Event::Key(key) = *events {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        self.running = false;
                    }
                    KeyCode::Tab | KeyCode::Right => {
                        self.current_text_input =
                            (self.current_text_input + 1) % (self.text_input_states.len() + 1);
                    }
                    KeyCode::BackTab | KeyCode::Left => {
                        if self.current_text_input == 0 {
                            self.current_text_input = self.text_input_states.len() + 1;
                        }

                        self.current_text_input -= 1
                    }
                    KeyCode::Char(character) => {
                        let input_state = self.text_input_states.get_mut(self.current_text_input);
                        if let Some(state) = input_state {
                            state.text.push(character);
                        }
                    }
                    KeyCode::Delete | KeyCode::Backspace => {
                        if self.current_text_input == self.text_input_states.len() {
                            // delete the last todo if available
                            if let Some(index) = self.todo_list_state.selected() {
                                self.todo_list.remove(index);
                                // if let Some(todo) = self.todo_list.get_mut(index) {

                                // }
                                self.save_todo_list().unwrap();
                            }
                        } else {
                            // delete the last text in the current input state
                            let input_state =
                                self.text_input_states.get_mut(self.current_text_input);
                            if let Some(state) = input_state {
                                state.text.pop();
                            }
                        }
                    }
                    KeyCode::Down => {
                        if self.current_text_input == 1 && self.todo_list.len() > 0 {
                            self.current_text_input = self.text_input_states.len();
                        } else if self.current_text_input == self.text_input_states.len() {
                            self.todo_list_state.select_next();
                        }
                    }
                    KeyCode::Up => {
                        if self.todo_list.len() > 0
                            && self.todo_list_state.selected().unwrap_or(0) == 0
                        {
                            self.current_text_input = self.text_input_states.len() - 1;
                        } else if self.current_text_input == self.text_input_states.len() {
                            self.todo_list_state.select_previous();
                        }
                    }
                    KeyCode::Enter => {
                        if self.current_text_input == 1 {
                            self.add_todo();
                        } else {
                            self.toggle_todo();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn toggle_todo(&mut self) {
        if let Some(index) = self.todo_list_state.selected() {
            if let Some(todo) = self.todo_list.get_mut(index) {
                todo.checked = !todo.checked;
                self.save_todo_list().unwrap();
            }
        }
    }

    fn add_todo(&mut self) {
        let new_todo_input_state = &mut self.text_input_states[1];
        let text = new_todo_input_state.text.trim();

        if text.len() > 0 {
            let todo = Todo::new(text);
            self.todo_list.push(todo);
            new_todo_input_state.text.clear();
            self.save_todo_list().unwrap();
        }
    }

    fn save_todo_list(&mut self) -> io::Result<()> {
        let json_content = serde_json::to_string(&self.todo_list)?;
        let file = File::create("todo.json");
        file?.write(json_content.as_bytes())?;
        return Ok(());
    }

    pub fn read_todo_list(&mut self) -> io::Result<()> {
        let mut file = File::open("todo.json")?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        self.todo_list = serde_json::from_str::<Vec<Todo>>(contents.as_str())?;

        return Ok(());
    }
}

impl Widget for &mut Application {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let app_block = Block::default()
            .borders(Borders::ALL)
            .title_top(Line::from(vec![" Rusty things todo ".into()]))
            .title_bottom(Line::from(vec![
                Span::from(" Quit "),
                Span::from("<Escape> ").style(Style::default().fg(Color::Blue)),
            ]));

        let inner_app_lock_area = app_block.inner(area);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
            .split(inner_app_lock_area);

        let text_top_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(layout[0]);

        let search_input = TextInput::new();
        let new_todo_input = TextInput::new();

        // set the focused state for all inputs
        for (index, state) in self.text_input_states.iter_mut().enumerate() {
            if index == self.current_text_input {
                state.set_focused(true);
            } else {
                state.set_focused(false);
            }
        }

        // todo list
        let list = List::new(self.todo_list.iter().enumerate().map(|(index, todo)| {
            Line::from(vec![
                Span::from((index + 1).to_string()),
                Span::from(if todo.checked { " - [x] " } else { " - [ ] " }),
                Span::from(todo.content.clone()),
            ])
        }))
        .highlight_style(
            Style::default()
                .bg(if self.current_text_input == self.text_input_states.len() {
                    Color::Yellow
                } else {
                    Color::Gray
                })
                .fg(Color::Black),
        );

        if self.todo_list_state.selected().is_none() && self.todo_list.len() > 0 {
            self.todo_list_state.select(Some(0));
        }

        app_block.render(area, buf);
        search_input.render(text_top_layout[0], buf, &mut self.text_input_states[0]);
        new_todo_input.render(text_top_layout[1], buf, &mut self.text_input_states[1]);
        StatefulWidget::render(list, layout[1], buf, &mut self.todo_list_state);
    }
}
