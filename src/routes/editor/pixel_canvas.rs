use crate::pixels::PixelGrid;
use log::info;

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
    pub fn move_select_up(&mut self, by: u16) {
        self.y = self.y.saturating_sub(by);
    }
    pub fn move_select_down(&mut self, by: u16) {
        self.y = self.y.saturating_add(by).min(self.grid.height - 1);
    }
    pub fn move_select_right(&mut self, by: u16) {
        self.x = self.x.saturating_add(by).min(self.grid.width - 1);
    }
    pub fn move_select_left(&mut self, by: u16) {
        self.x = self.x.saturating_sub(by);
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

impl Widget for &mut PixelCanvas {
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
                let mut upper_color: Color = upper.color.into();
                let mut lower_color = if row_lower < self.grid.height as usize {
                    let lower = &self.grid.grid[idx_col][row_lower];
                    lower.color.into()
                } else {
                    Color::Reset
                };

                if col == self.x {
                    if row_upper == self.y as usize {
                        upper_color = upper.color.invert().into();
                    } else if row_lower == self.y as usize {
                        lower_color = self.grid.grid[idx_col][row_lower].color.invert().into();
                    }
                }

                if let Some(cell) = buf.cell_mut(Position::new(x_off + col, y_off + row)) {
                    cell.set_char('▀');
                    cell.fg = upper_color;
                    cell.bg = lower_color;
                }
            }
        }
    }
}
