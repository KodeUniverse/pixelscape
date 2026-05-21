use ratatui::buffer::Buffer;
use ratatui::layout::{HorizontalAlignment, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Widget};

#[derive(Default)]
pub struct Editor;

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title_top(" Pixel Editor ")
            .title_alignment(HorizontalAlignment::Center)
            .border_type(BorderType::Thick)
            .render(area, buf);
    }
}
