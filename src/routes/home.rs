use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, List, ListItem, Widget};

#[derive(Default)]
pub struct Home;

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let items = [
            ListItem::new("Create new project"),
            ListItem::new("Open existing project"),
            ListItem::new("Options"),
        ];
        let vert = Layout::vertical([Constraint::Fill(1), Constraint::Percentage(40)]).split(area);
        let block_slot = Rect::new(area.width / 2 - 25, vert[1].y, 50, 10);
        let block = Block::bordered().border_set(border::THICK);
        Line::from("PIXTERM")
            .centered()
            .render(vert[0].centered_vertically(Constraint::Percentage(30)), buf);
        List::new(items)
            .highlight_symbol(">")
            .block(block)
            .render(block_slot, buf);
    }
}
