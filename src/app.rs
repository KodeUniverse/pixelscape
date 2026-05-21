use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};
use std::io;

use crate::events::handle_events;
use crate::routes::{editor, home};

#[derive(Debug, Default)]
pub enum Route {
    #[default]
    Home,
    Editor,
}
pub struct App {
    pub route: Route,
    pub home: home::Home,
    pub editor: editor::Editor,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            route: Route::Home,
            home: home::Home::default(),
            editor: editor::Editor::default(),
            exit: false,
        }
    }
}
impl App {
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
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.route {
            Route::Home => {
                home::Home.render(area, buf);
            }
            Route::Editor => {
                editor::Editor.render(area, buf);
            }
        }
    }
}
