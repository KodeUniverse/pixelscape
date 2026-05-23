use crate::pixels::PixelGrid;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::widgets::Widget;

pub struct PixelCanvas {
    pub grid: PixelGrid,
    pub x: u16,
    pub y: u16,
}

impl PixelCanvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            grid: PixelGrid::new(width, height),
            x: 0,
            y: 0,
        }
    }
    pub fn move_marker_up(&mut self, by: u16) {
        self.y -= by;
    }
    pub fn move_state_down(&mut self, by: u16) {
        self.y += by;
    }
    pub fn move_state_right(&mut self, by: u16) {
        self.x += by;
    }
    pub fn move_state_left(&mut self, by: u16) {
        self.x -= by;
    }
}

impl Default for PixelCanvas {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            grid: PixelGrid::new(64, 64),
        }
    }
}

impl Widget for &PixelCanvas {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = (self.grid.height + 1) / 2;
        let cols = self.grid.width;

        let x_off = area.x + (area.width.saturating_sub(cols)) / 2;
        let y_off = area.y + (area.height.saturating_sub(rows)) / 2;

        for row in 0..rows {
            let row_upper = row as usize * 2;
            let row_lower = row_upper + 1;
            for col in 0..cols {
                let idx_col = col as usize;
                let upper = &self.grid.grid[idx_col][row_upper];
                let upper_color = Color::Rgb(upper.color.red, upper.color.green, upper.color.blue);
                let lower_color = if row_lower < self.grid.height as usize {
                    let lower = &self.grid.grid[idx_col][row_lower];
                    Color::Rgb(lower.color.red, lower.color.green, lower.color.blue)
                } else {
                    Color::Reset
                };

                if let Some(cell) = buf.cell_mut(Position::new(x_off + col, y_off + row)) {
                    cell.set_char('▀');
                    cell.fg = upper_color;
                    cell.bg = lower_color;
                }
            }
        }
    }
}
