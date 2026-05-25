use std::path::Path;

use crate::app::EventMode;
use crate::routes::editor::pixel_canvas::PixelCanvas;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

pub struct Editor {
    pub canvas: PixelCanvas,
    pub saving: bool,
    pub exporting: bool,
    pub input: String,
    pub event_mode: EventMode,
}
impl Editor {
    pub fn start_with_file(file: &Path) -> Self {
        let canvas = PixelCanvas::default();
        canvas.grid.read_from_file(file).unwrap();
        Self {
            canvas,
            saving: false,
            exporting: false,
            input: String::default(),
            event_mode: EventMode::Normal,
        }
    }
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas: PixelCanvas::default(),
            saving: false,
            exporting: false,
            input: String::default(),
            event_mode: EventMode::Normal,
        }
    }
}

impl Widget for &mut Editor {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title_top(" Pixel Editor ")
            .title_alignment(HorizontalAlignment::Center)
            .border_type(BorderType::Thick);
        (&block).render(area, buf);
        let inner = block.inner(area);

        let chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(inner);
        let canvas_area = chunks[0];
        if !self.saving && !self.exporting {
            (&mut self.canvas).render(inner, buf);
        } else if self.saving {
            let save_area = chunks[1];
            let save_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            (&save_block).render(save_area, buf);

            Paragraph::new(format!("Save project: {}.pxsc", self.input))
                .render(save_block.inner(save_area), buf);

            (&mut self.canvas).render(canvas_area, buf);
        } else if self.exporting {
            let export_area = chunks[1];
            let export_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            (&export_block).render(export_area, buf);

            Paragraph::new(format!("Export to PNG: {}.png", self.input))
                .render(export_block.inner(export_area), buf);

            (&mut self.canvas).render(canvas_area, buf);
        }
    }
}
