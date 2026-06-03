use ratatui::widgets::ListState;

use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::path::Path;

use crate::events::handle_events;
use crate::routes::{editor, home};

#[derive(Debug, Default)]
pub enum Route {
    #[default]
    Home,
    Editor,
}

pub enum EventMode {
    Normal,
    Input,
}
pub struct App {
    pub route: Route,
    pub home: home::Home,
    pub editor: editor::layout::Editor,
    pub home_list_state: ListState,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            route: Route::Home,
            home: home::Home,
            editor: editor::layout::Editor::default(),
            home_list_state: ListState::default(),
            exit: false,
        };

        app.home_list_state.select_first();
        app
    }
}
impl App {
    pub fn start_with_file(file: &Path) -> Self {
        Self {
            route: Route::Editor,
            home: home::Home,
            editor: editor::layout::Editor::start_with_file(file),
            home_list_state: ListState::default(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            handle_events(self)?;
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
    fn draw(&mut self, frame: &mut Frame) {
        match self.route {
            Route::Home => {
                frame.render_stateful_widget(&self.home, frame.area(), &mut self.home_list_state);
            }
            Route::Editor => {
                frame.render_widget(&mut self.editor, frame.area());
            }
        }
    }
}
