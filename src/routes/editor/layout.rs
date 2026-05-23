use crate::routes::editor::pixel_canvas::PixelCanvas;
use ratatui::buffer::Buffer;
use ratatui::layout::{HorizontalAlignment, Rect};
use ratatui::widgets::{Block, BorderType, Widget};

pub struct Editor {
    pub canvas: PixelCanvas,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas: PixelCanvas::new(64, 64),
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
        (&self.canvas).render(block.inner(area), buf);
    }
}
