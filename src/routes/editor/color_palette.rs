use ratatui::layout::{Position, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::Color;
use ratatui::text::Text;
use ratatui::widgets::{Block, Padding, Widget};

pub struct ColorPalette {
    pub colors: Vec<Color>,
}
impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            colors: vec![
                Color::White,
                Color::Black,
                Color::Gray,
                Color::Red,
                Color::LightRed,
                Color::Magenta,
                Color::Blue,
                Color::LightBlue,
                Color::Yellow,
                Color::Green,
                Color::LightGreen,
                Color::LightMagenta,
                Color::Cyan,
            ],
        }
    }
}

pub struct PaletteGridState {
    pub selected: u8,
}

impl Default for PaletteGridState {
    fn default() -> Self {
        Self { selected: 0 }
    }
}
pub struct ColorPaletteGrid {
    pub blocks: Vec<PaletteGridBlock>,
    pub gap: u8,
    pub state: PaletteGridState,
}
impl ColorPaletteGrid {
    pub fn new(blocks: Vec<PaletteGridBlock>, gap: u8, state: PaletteGridState) -> Self {
        Self { blocks, gap, state }
    }
}

#[derive(Clone)]
pub struct PaletteGridBlock {
    pub color: Color,
    pub size: u8,
    pub term_width: u8,
    pub term_height: u8,
}

impl PaletteGridBlock {
    pub fn new(color: Color, size: u8) -> Self {
        Self {
            color,
            size,
            term_width: size * 2,
            term_height: size,
        }
    }
}

impl Widget for PaletteGridBlock {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        for row in 0..self.term_height {
            for col in 0..self.term_width {
                let cell = buf
                    .cell_mut(Position::new(area.x + col as u16, area.y + row as u16))
                    .unwrap();
                cell.set_char('█').set_fg(self.color);
            }
        }
    }
}

impl Widget for ColorPaletteGrid {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let palette_container = Block::default().padding(Padding::uniform(1));
        let palette_area = palette_container.inner(area);
        palette_container.render(area, buf);

        // calculating how many columns of blocks we can fit in a row with a given gap size
        // ratio of (total area width + gap width) / (block_width + gap width)
        let blocks_per_row = (palette_area.width + self.gap as u16)
            / (self.blocks[0].term_width as u16 + self.gap as u16);

        let mut block_count = 0;
        let mut x_offset = 0;
        let mut y_offset = 0;

        for block in self.blocks {
            let mut slot = Rect::from(palette_area);

            if block_count != 0 {
                if block_count % blocks_per_row == 0 {
                    x_offset = 0;
                    y_offset += block.term_height + self.gap;
                } else {
                    x_offset += block.term_width + self.gap;
                }
            }

            slot.x += x_offset as u16;
            slot.y += y_offset as u16;
            slot.width = block.term_width as u16;
            slot.height = block.term_height as u16;

            block.render(slot, buf);
            block_count += 1;
        }
    }
}
