use crate::pixels::PixelColor;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::Color;
use ratatui::widgets::{Block, Padding, Widget};

pub struct ColorPalette {
    pub colors: Vec<PixelColor>,
}
impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            colors: vec![
                // Grays (7)
                PixelColor::new(255, 255, 255, false),
                PixelColor::new(212, 212, 212, false),
                PixelColor::new(168, 168, 168, false),
                PixelColor::new(124, 124, 124, false),
                PixelColor::new(80, 80, 80, false),
                PixelColor::new(36, 36, 36, false),
                PixelColor::new(0, 0, 0, false),
                // Reds (3)
                PixelColor::new(255, 85, 85, false),
                PixelColor::new(204, 51, 51, false),
                PixelColor::new(136, 34, 34, false),
                // Oranges (3)
                PixelColor::new(255, 136, 51, false),
                PixelColor::new(204, 102, 34, false),
                PixelColor::new(255, 187, 102, false),
                // Yellows (3)
                PixelColor::new(255, 221, 68, false),
                PixelColor::new(204, 170, 34, false),
                PixelColor::new(255, 244, 176, false),
                // Browns (3)
                PixelColor::new(139, 94, 60, false),
                PixelColor::new(58, 34, 16, false),
                // Greens (5)
                PixelColor::new(68, 204, 68, false),
                PixelColor::new(34, 139, 34, false),
                PixelColor::new(168, 230, 168, false),
                // Teals (2)
                PixelColor::new(51, 170, 170, false),
                PixelColor::new(34, 119, 119, false),
                // Blues (5)
                PixelColor::new(51, 102, 221, false),
                PixelColor::new(102, 153, 238, false),
                PixelColor::new(170, 204, 255, false),
                // Purples (3)
                PixelColor::new(153, 68, 238, false),
                PixelColor::new(102, 34, 170, false),
                PixelColor::new(187, 119, 238, false),
                // Pinks (3)
                PixelColor::new(238, 102, 170, false),
                PixelColor::new(204, 68, 136, false),
                PixelColor::new(255, 136, 187, false),
                // Skin tones (3)
                PixelColor::new(255, 212, 184, false),
                PixelColor::new(212, 160, 120, false),
                PixelColor::new(160, 112, 80, false),
            ],
        }
    }
}

pub struct PaletteGridState {
    pub selected: u8,
    pub secondary: u8,
}

impl Default for PaletteGridState {
    fn default() -> Self {
        Self {
            selected: 0,
            secondary: 0,
        }
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
    pub color: PixelColor,
    pub term_width: u8,
    pub term_height: u8,
}

impl PaletteGridBlock {
    pub fn new(color: PixelColor, size: u8) -> Self {
        Self {
            color,
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
                cell.set_char('▀')
                    .set_fg(self.color.into())
                    .set_bg(self.color.into());
            }
        }
    }
}

fn render_block_border(
    buf: &mut Buffer,
    base_x: u16,
    base_y: u16,
    bw: u16,
    bh: u16,
    border_color: Color,
    block_color: PixelColor,
    dashed: bool,
) {
    for row in 0..bh {
        for col in 0..bw {
            let is_edge = row == 0 || row == bh - 1 || col == 0 || col == bw - 1;
            if !is_edge {
                continue;
            }

            let skip = dashed && {
                let on_top = row == 0;
                let on_bottom = row == bh - 1;
                let on_left = col == 0;
                let on_right = col == bw - 1;

                if (on_top || on_bottom) && !on_left && !on_right && col % 2 == 0 {
                    true
                } else if (on_left || on_right) && !on_top && !on_bottom && row % 2 == 0 {
                    true
                } else {
                    false
                }
            };

            if skip {
                continue;
            }

            if let Some(cell) = buf.cell_mut(Position::new(base_x + col, base_y + row)) {
                let (ch, fg, bg) = if (row == 0 || row == bh - 1) && (col == 0 || col == bw - 1) {
                    ('█', border_color, border_color)
                } else if row == 0 {
                    ('▀', border_color, block_color.into())
                } else if row == bh - 1 {
                    ('▄', border_color, block_color.into())
                } else {
                    ('█', border_color, border_color)
                };
                cell.set_char(ch).set_fg(fg).set_bg(bg);
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

        if self.blocks.is_empty() {
            return;
        }

        let bw = self.blocks[0].term_width;
        let bh = self.blocks[0].term_height;

        let blocks_per_row = (palette_area.width + self.gap as u16) / (bw as u16 + self.gap as u16);

        if blocks_per_row == 0 {
            return;
        }

        for (block_idx, block) in self.blocks.iter().enumerate() {
            let row_idx = block_idx as u16 / blocks_per_row;
            let col_idx = block_idx as u16 % blocks_per_row;

            let base_x = palette_area.x + col_idx * (bw as u16 + self.gap as u16);
            let base_y = palette_area.y + row_idx * (bh as u16 + self.gap as u16);

            let is_primary = block_idx == self.state.selected as usize;
            let is_secondary = block_idx == self.state.secondary as usize;

            for row in 0..bh {
                for col in 0..bw {
                    if let Some(cell) =
                        buf.cell_mut(Position::new(base_x + col as u16, base_y + row as u16))
                    {
                        cell.set_char('▀')
                            .set_fg(block.color.into())
                            .set_bg(block.color.into());
                    }
                }
            }

            if is_primary {
                render_block_border(
                    buf, base_x, base_y, bw as u16, bh as u16,
                    Color::White, block.color, false,
                );
            }

            if is_secondary && !is_primary {
                render_block_border(
                    buf, base_x, base_y, bw as u16, bh as u16,
                    Color::Blue, block.color, false,
                );
            }
        }
    }
}
