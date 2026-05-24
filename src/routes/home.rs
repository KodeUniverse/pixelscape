use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget};
#[derive(Default)]
pub struct Home;

impl StatefulWidget for &Home {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState)
    where
        Self: Sized,
    {
        let items = [
            ListItem::new("Create new project"),
            ListItem::new("Open existing project"),
            ListItem::new("Settings"),
        ];
        let vert = Layout::vertical([Constraint::Fill(1), Constraint::Percentage(40)]).split(area);
        let block_slot = Rect::new(area.width / 2 - 25, vert[1].y, 50, 10);
        let block = Block::bordered().border_set(border::THICK);
        Line::from("PIXELSCAPE")
            .centered()
            .render(vert[0].centered_vertically(Constraint::Percentage(30)), buf);

        StatefulWidget::render(
            List::new(items)
                .highlight_symbol(" > ")
                .block(block)
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black)),
            block_slot,
            buf,
            state,
        );
    }
}
