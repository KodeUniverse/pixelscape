use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Text;
use ratatui::widgets::Widget;

#[derive(Default)]
pub struct Editor;

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Text::from("Pixel Editor").render(area, buf);
    }
}
