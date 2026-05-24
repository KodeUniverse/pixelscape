use crate::pixels::{PixelColor, PixelGrid};
use log::info;

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::widgets::Widget;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

pub struct PixelCanvas {
    pub grid: PixelGrid,
    pub cursor: Cursor,
}

impl PixelCanvas {
    pub fn new(grid_w: u16, grid_h: u16) -> Self {
        Self {
            grid: PixelGrid::new(grid_w, grid_h),
            cursor: Cursor { x: 0, y: 0 },
        }
    }
    pub fn move_select_up(&mut self, by: u16) {
        self.cursor.y = self.cursor.y.saturating_sub(by);
    }
    pub fn move_select_down(&mut self, by: u16) {
        self.cursor.y = self.cursor.y.saturating_add(by).min(self.grid.height - 1);
    }
    pub fn move_select_right(&mut self, by: u16) {
        self.cursor.x = self.cursor.x.saturating_add(by).min(self.grid.width - 1);
    }
    pub fn move_select_left(&mut self, by: u16) {
        self.cursor.x = self.cursor.x.saturating_sub(by);
    }
}

impl Default for PixelCanvas {
    fn default() -> Self {
        Self {
            cursor: Cursor { x: 0, y: 0 },
            grid: PixelGrid::default(),
        }
    }
}

impl Widget for &mut PixelCanvas {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = (self.grid.height + 1) / 2; // to handle odd values also
        let cols = self.grid.width;

        let x_off = area.x + (area.width.saturating_sub(cols)) / 2;
        let y_off = area.y + (area.height.saturating_sub(rows)) / 2;

        for row in 0..rows {
            let row_upper = row as usize * 2;
            let row_lower = row_upper + 1;
            for col in 0..cols {
                let upper_color: Color = self.grid.get(col as u16, row_upper as u16).color.into();

                let lower_color = if row_lower < self.grid.height as usize {
                    self.grid.get(col as u16, row_lower as u16).color.into()
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
        let px = self.cursor.x;
        let py = self.cursor.y + 1;
        let ty = py / 2;
        let upper = py % 2 == 0;

        info!("pixel ({px}, {py})  terminal ({px}, {ty})");

        let left_neighbor = self
            .grid
            .get(self.cursor.x.saturating_sub(1), self.cursor.y);
        let right_neighbor = self
            .grid
            .get(self.cursor.x.saturating_add(1), self.cursor.y);

        let avg_neighbor_color = PixelColor::new(
            ((left_neighbor.color.red as usize + right_neighbor.color.red as usize) / 2)
                .try_into()
                .unwrap_or(255),
            ((left_neighbor.color.green as usize + right_neighbor.color.green as usize) / 2)
                .try_into()
                .unwrap_or(255),
            ((left_neighbor.color.blue as usize + right_neighbor.color.blue as usize) / 2)
                .try_into()
                .unwrap_or(255),
            None,
        );
        let cur_color: Color = Color::Rgb(
            255 - avg_neighbor_color.red,
            255 - avg_neighbor_color.green,
            255 - avg_neighbor_color.blue,
        );

        let cell = buf.cell_mut(Position::new(
            x_off + px,
            y_off + ty - if upper { 1 } else { 0 },
        ));
        if let Some(cell) = cell {
            if upper {
                cell.bg = cur_color;
            } else {
                cell.fg = cur_color;
            }
        }
    }
}
