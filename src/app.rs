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

#[derive(Debug)]
pub struct App {
    pub route: Route,
    exit: bool,
}
impl Default for App {
    fn default() -> Self {
        Self {
            route: Route::Home,
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
