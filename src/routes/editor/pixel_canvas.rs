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

    pub fn from_grid(grid: PixelGrid) -> Self {
        Self {
            grid,
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

fn render_brush_preview(
    buf: &mut Buffer,
    grid: &PixelGrid,
    cursor: &Cursor,
    brush_size: u8,
    x_off: u16,
    y_off: u16,
) {
    if brush_size <= 1 {
        return;
    }
    let half = brush_size as u16 / 2;
    let bx_start = cursor.x.saturating_sub(half);
    let bx_end = (cursor.x + half).min(grid.width - 1);
    let by_start = cursor.y.saturating_sub(half);
    let by_end = (cursor.y + half).min(grid.height - 1);

    let rows = grid.height.div_ceil(2);

    for ty in 0..rows {
        let row_upper = ty as usize * 2;
        let row_lower = row_upper + 1;

        for tx in 0..grid.width {
            let x = tx;
            let yu = row_upper as u16;
            let yl = row_lower as u16;

            let in_brush_upper = x >= bx_start && x <= bx_end && yu >= by_start && yu <= by_end;
            let on_edge_upper =
                in_brush_upper && (x == bx_start || x == bx_end || yu == by_start || yu == by_end);

            let has_lower = yl < grid.height;
            let in_brush_lower =
                has_lower && x >= bx_start && x <= bx_end && yl >= by_start && yl <= by_end;
            let on_edge_lower =
                in_brush_lower && (x == bx_start || x == bx_end || yl == by_start || yl == by_end);

            if !on_edge_upper && !on_edge_lower {
                continue;
            }

            if let Some(cell) = buf.cell_mut(Position::new(x_off + x, y_off + ty)) {
                if on_edge_upper {
                    cell.fg = Color::White;
                }
                if on_edge_lower {
                    cell.bg = Color::White;
                }
            }
        }
    }
}

impl Widget for &mut PixelCanvas {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = self.grid.height.div_ceil(2);
        let cols = self.grid.width;

        let x_off = area.x + (area.width.saturating_sub(cols)) / 2;
        let y_off = area.y + (area.height.saturating_sub(rows)) / 2;

        for row in 0..rows {
            let row_upper = row as usize * 2;
            let row_lower = row_upper + 1;
            for col in 0..cols {
                let upper_pixel = self.grid.get(col, row_upper as u16);
                let upper_color: Color = upper_pixel.color.into();
                let upper_transparent = upper_pixel.color.transparent;

                let (lower_color, lower_transparent) = if row_lower < self.grid.height as usize {
                    let p = self.grid.get(col, row_lower as u16);
                    (p.color.into(), p.color.transparent)
                } else {
                    (Color::Reset, true)
                };

                if let Some(cell) = buf.cell_mut(Position::new(x_off + col, y_off + row)) {
                    let (ch, fg, bg) = match (upper_transparent, lower_transparent) {
                        (false, false) => ('▀', upper_color, lower_color),
                        (false, true)  => ('▀', upper_color, Color::Reset),
                        (true, false)  => ('▄', lower_color, Color::Reset),
                        (true, true)   => (' ', Color::Reset, Color::Reset),
                    };
                    cell.set_char(ch);
                    cell.fg = fg;
                    cell.bg = bg;
                }
            }
        }

        let px = self.cursor.x;
        let ty = self.cursor.y / 2;
        let upper = self.cursor.y.is_multiple_of(2);

        info!("pixel ({px}, {})  terminal ({px}, {ty})", self.cursor.y);

        let left_neighbor = self
            .grid
            .get(self.cursor.x.saturating_sub(1), self.cursor.y);
        let right_neighbor = self.grid.get(
            self.cursor.x.saturating_add(1).min(self.grid.width - 1),
            self.cursor.y,
        );

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
            false,
        );
        let cur_color: Color = Color::Rgb(
            255 - avg_neighbor_color.red,
            255 - avg_neighbor_color.green,
            255 - avg_neighbor_color.blue,
        );

        let cell = buf.cell_mut(Position::new(
            x_off + px,
            y_off + ty,
        ));
        if let Some(cell) = cell {
            let has_lower = self.cursor.y + 1 < self.grid.height;
            let (fg, bg) = if upper {
                let lc: Color = if has_lower {
                    self.grid.get(px, self.cursor.y + 1).color.into()
                } else {
                    Color::Reset
                };
                (cur_color, lc)
            } else {
                let uc: Color = self.grid.get(px, self.cursor.y).color.into();
                (uc, cur_color)
            };
            cell.set_char('▀');
            cell.fg = fg;
            cell.bg = bg;
        }
    }
}

impl PixelCanvas {
    pub fn render_with(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        brush_size: u8,
    ) -> (u16, u16, u16, u16) {
        let rows = self.grid.height.div_ceil(2);
        let cols = self.grid.width;

        let x_off = area.x + (area.width.saturating_sub(cols)) / 2;
        let y_off = area.y + (area.height.saturating_sub(rows)) / 2;

        for row in 0..rows {
            let row_upper = row as usize * 2;
            let row_lower = row_upper + 1;
            for col in 0..cols {
                let upper_pixel = self.grid.get(col, row_upper as u16);
                let upper_color: Color = upper_pixel.color.into();
                let upper_transparent = upper_pixel.color.transparent;

                let (lower_color, lower_transparent) = if row_lower < self.grid.height as usize {
                    let p = self.grid.get(col, row_lower as u16);
                    (p.color.into(), p.color.transparent)
                } else {
                    (Color::Reset, true)
                };

                if let Some(cell) = buf.cell_mut(Position::new(x_off + col, y_off + row)) {
                    let (ch, fg, bg) = match (upper_transparent, lower_transparent) {
                        (false, false) => ('▀', upper_color, lower_color),
                        (false, true)  => ('▀', upper_color, Color::Reset),
                        (true, false)  => ('▄', lower_color, Color::Reset),
                        (true, true)   => (' ', Color::Reset, Color::Reset),
                    };
                    cell.set_char(ch);
                    cell.fg = fg;
                    cell.bg = bg;
                }
            }
        }

        let px = self.cursor.x;
        let ty = self.cursor.y / 2;
        let upper = self.cursor.y.is_multiple_of(2);

        let left_neighbor = self
            .grid
            .get(self.cursor.x.saturating_sub(1), self.cursor.y);
        let right_neighbor = self.grid.get(
            self.cursor.x.saturating_add(1).min(self.grid.width - 1),
            self.cursor.y,
        );

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
            false,
        );
        let cur_color: Color = Color::Rgb(
            255 - avg_neighbor_color.red,
            255 - avg_neighbor_color.green,
            255 - avg_neighbor_color.blue,
        );

        let cell = buf.cell_mut(Position::new(
            x_off + px,
            y_off + ty,
        ));
        if let Some(cell) = cell {
            let has_lower = self.cursor.y + 1 < self.grid.height;
            let (fg, bg) = if upper {
                let lc: Color = if has_lower {
                    self.grid.get(px, self.cursor.y + 1).color.into()
                } else {
                    Color::Reset
                };
                (cur_color, lc)
            } else {
                let uc: Color = self.grid.get(px, self.cursor.y).color.into();
                (uc, cur_color)
            };
            cell.set_char('▀');
            cell.fg = fg;
            cell.bg = bg;
        }

        render_brush_preview(buf, &self.grid, &self.cursor, brush_size, x_off, y_off);

        (x_off, y_off, cols, rows)
    }
}
