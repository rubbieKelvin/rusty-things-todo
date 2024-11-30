use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{ListState, Widget},
    DefaultTerminal,
};

use crate::ui::{render_editing_ui, render_main_ui};

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub struct Todo {
    pub id: String,
    pub text: String,
    pub checked: bool,
}

impl Todo {
    fn new(text: &str) -> Self {
        let id = uuid::Uuid::new_v4();
        return Todo {
            id: id.to_string(),
            text: text.to_string(),
            checked: false,
        };
    }
}

pub struct TodoList {
    pub list: Vec<Todo>,
    pub state: ListState,
}

impl TodoList {
    fn new() -> Self {
        return TodoList {
            list: Vec::new(),
            state: ListState::default(),
        };
    }
}

pub struct Application {
    running: bool,
    pub todos: TodoList,
    pub todo_input_text: String,
    pub current_screen: CurrentScreen,
}

impl Application {
    pub fn new() -> Self {
        return Application {
            running: true,
            todos: TodoList::new(),
            todo_input_text: String::new(),
            current_screen: CurrentScreen::Main,
        };
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match self.current_screen {
                        // handle main screen event
                        CurrentScreen::Main => match key.code {
                            KeyCode::Char('n') | KeyCode::Char('N') => {
                                self.current_screen = CurrentScreen::Editing;
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                                self.running = false;
                            }
                            KeyCode::Up => {
                                self.todos.state.select_previous();
                            }
                            KeyCode::Down => {
                                self.todos.state.select_next();
                            }
                            _ => {}
                        },
                        CurrentScreen::Editing => match key.code {
                            KeyCode::Esc => {
                                self.current_screen = CurrentScreen::Main;
                                self.todo_input_text.clear();
                            }
                            KeyCode::Tab => {
                                self.todo_input_text.push_str("   ");
                            }
                            KeyCode::Char(n) => {
                                self.todo_input_text.push(n);
                            }
                            KeyCode::Delete | KeyCode::Backspace => {
                                self.todo_input_text.pop();
                            }
                            KeyCode::Enter => {
                                self.current_screen = CurrentScreen::Main;

                                let todo = Todo::new(&self.todo_input_text);
                                self.todos.list.push(todo);

                                self.todo_input_text.clear();
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            let e = event::read()?;

            self.handle_event(e);
            if !self.running {
                return Ok(());
            }
        }
    }
}

impl Widget for &mut Application {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        match self.current_screen {
            CurrentScreen::Main => render_main_ui(area, buffer, self),
            CurrentScreen::Editing => render_editing_ui(area, buffer, self),
            _ => {}
        }
    }
}
