use crate::pixels::{Pixel, PixelGrid};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, HorizontalAlignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Cell, Row, StatefulWidget, Table, TableState, Widget};

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

        let table_height = ((self.pixel_grid.height + 1) / 2) as usize;
        let table_width = self.pixel_grid.width as usize;

        let rows: Vec<Row> = (0..table_height)
            .map(|row| {
                let (row_upper, row_lower) = (row * 2, row * 2 + 1);
                let cells: Vec<Cell> = (0..table_width)
                    .map(|col| {
                        let upper: &Pixel = &self.pixel_grid.grid[col][row_upper];
                        let upper_color =
                            Color::Rgb(upper.color.red, upper.color.green, upper.color.blue);
                        let lower_color = if row_lower < self.pixel_grid.height as usize {
                            // index guard for odd heights
                            let lower: &Pixel = &self.pixel_grid.grid[col][row_lower];
                            Color::Rgb(lower.color.red, lower.color.green, lower.color.blue)
                        } else {
                            Color::Reset // will render a transparent row on the bottom of table
                            // when table_height is odd
                        };
                        Cell::from("▀").style(Style::default().fg(upper_color).bg(lower_color))
                    })
                    .collect();
                Row::new(cells)
            })
            .collect();

        let widths = vec![Constraint::Length(1); table_width];

        let table = Table::new(rows, widths)
            .column_spacing(0)
            .flex(Flex::Center)
            .cell_highlight_style(Style::default().bg(Color::Red));
        StatefulWidget::render(table, inner, buf, state);
    }
}
