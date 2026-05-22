use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Cell, Row, StatefulWidget, Table, TableState, Widget};

use crate::pixels::PixelGrid;

pub struct Editor {
    pub pixel_grid: PixelGrid,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            pixel_grid: PixelGrid::new(64, 64),
        }
    }
}

impl StatefulWidget for &Editor {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TableState)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title_top(" Pixel Editor ")
            .title_alignment(HorizontalAlignment::Center)
            .border_type(BorderType::Thick);
        (&block).render(area, buf);

        let inner = block.inner(area);
        let row_size = self.pixel_grid.grid.len();

        // Half-block "▀" renders 2 pixel rows per terminal row (fg = upper, bg = lower)
        let half_row_size = (row_size + 1) / 2; // (x+1) / 2 to handle odd sizes
        let rows: Vec<Row> = (0..half_row_size)
            .map(|row_y| {
                // upper_y, lower_y  multiplied by 2 due to rendering two Pixel structs per
                // terminal cell.
                let (upper_y, lower_y) = (row_y * 2, row_y * 2 + 1);
                let cells: Vec<Cell> = (0..row_size)
                    .map(|x| {
                        let (upper, lower) = (
                            &self.pixel_grid.grid[x][upper_y],
                            &self.pixel_grid.grid[x][lower_y],
                        );
                        let (fg, bg) = (
                            Color::Rgb(upper.color.red, upper.color.green, upper.color.blue),
                            Color::Rgb(lower.color.red, lower.color.green, lower.color.blue),
                        );
                        let style = if lower_y < row_size {
                            Style::default().fg(fg).bg(bg)
                        } else {
                            Style::default().fg(fg).bg(fg)
                        };
                        Cell::from("▀").style(style)
                    })
                    .collect();
                Row::new(cells)
            })
            .collect();

        let widths = vec![Constraint::Length(1); row_size];

        let table = Table::new(rows, widths)
            .column_spacing(0)
            .flex(Flex::Center);

        StatefulWidget::render(table, inner, buf, state);
    }
}
