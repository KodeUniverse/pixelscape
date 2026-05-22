use ratatui::widgets::{ListState, StatefulWidget, TableState};
use ratatui::{DefaultTerminal, Frame};
use std::io;

use crate::events::handle_events;
use crate::routes::editor::Editor;
use crate::routes::{editor, home};

#[derive(Debug, Default)]
pub enum Route {
    #[default]
    Home,
    Editor,
}

pub struct App<'a> {
    pub route: Route,
    pub home: home::Home,
    pub editor: editor::Editor,
    pub home_list_state: ListState,
    pub pixel_select_state: <&'a Editor as StatefulWidget>::State,
    exit: bool,
}

impl Default for App<'_> {
    fn default() -> Self {
        let mut out = Self {
            route: Route::Home,
            home: home::Home::default(),
            editor: editor::Editor::default(),
            home_list_state: ListState::default(),
            pixel_select_state: TableState::default(),
            exit: false,
        };
        out.home_list_state.select_first();
        out.pixel_select_state.select_first();
        return out;
    }
}
impl App<'_> {
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
                frame.render_stateful_widget(
                    &self.editor,
                    frame.area(),
                    &mut self.pixel_select_state,
                );
            }
        }
    }
}
