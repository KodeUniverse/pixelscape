use ratatui::widgets::{ListState, StatefulWidget, TableState};
use ratatui::{DefaultTerminal, Frame};
use std::io;

use crate::events::handle_events;
use crate::pixels::PixelColor;
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
        let mut grid = editor::Editor::default();
        grid.pixel_grid.grid[0][1].color = PixelColor::new(0, 255, 0, None);
        grid.pixel_grid.grid[0][61].color = PixelColor::new(0, 0, 255, None);
        grid.pixel_grid.grid[1][0].color = PixelColor::new(150, 200, 220, None);
        grid.pixel_grid.grid[61][0].color = PixelColor::new(50, 50, 50, None);
        let mut app = Self {
            route: Route::Home,
            home: home::Home::default(),
            editor: grid,
            home_list_state: ListState::default(),
            pixel_select_state: TableState::default(),
            exit: false,
        };
        app.home_list_state.select_first();
        app.pixel_select_state.select_first();
        return app;
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
