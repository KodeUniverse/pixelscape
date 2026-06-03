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
                // Grays (10)
                PixelColor::new(255, 255, 255, false),
                PixelColor::new(221, 221, 221, false),
                PixelColor::new(187, 187, 187, false),
                PixelColor::new(153, 153, 153, false),
                PixelColor::new(119, 119, 119, false),
                PixelColor::new(85, 85, 85, false),
                PixelColor::new(68, 68, 68, false),
                PixelColor::new(51, 51, 51, false),
                PixelColor::new(34, 34, 34, false),
                PixelColor::new(0, 0, 0, false),
                // Reds (8)
                PixelColor::new(255, 68, 68, false),
                PixelColor::new(238, 34, 34, false),
                PixelColor::new(204, 0, 0, false),
                PixelColor::new(170, 0, 0, false),
                PixelColor::new(136, 0, 0, false),
                PixelColor::new(102, 0, 0, false),
                PixelColor::new(255, 153, 153, false),
                PixelColor::new(255, 204, 204, false),
                // Oranges (8)
                PixelColor::new(255, 136, 0, false),
                PixelColor::new(238, 119, 0, false),
                PixelColor::new(204, 102, 0, false),
                PixelColor::new(170, 85, 0, false),
                PixelColor::new(255, 170, 85, false),
                PixelColor::new(255, 204, 136, false),
                PixelColor::new(255, 238, 204, false),
                PixelColor::new(255, 102, 68, false),
                // Yellows (6)
                PixelColor::new(255, 221, 51, false),
                PixelColor::new(238, 204, 34, false),
                PixelColor::new(204, 170, 0, false),
                PixelColor::new(255, 238, 102, false),
                PixelColor::new(255, 245, 170, false),
                PixelColor::new(255, 250, 221, false),
                // Greens (14)
                PixelColor::new(68, 204, 68, false),
                PixelColor::new(51, 170, 51, false),
                PixelColor::new(34, 136, 34, false),
                PixelColor::new(17, 102, 17, false),
                PixelColor::new(0, 68, 0, false),
                PixelColor::new(136, 221, 136, false),
                PixelColor::new(187, 238, 187, false),
                PixelColor::new(221, 255, 221, false),
                PixelColor::new(102, 204, 102, false),
                PixelColor::new(0, 170, 68, false),
                PixelColor::new(0, 136, 51, false),
                PixelColor::new(68, 187, 153, false),
                PixelColor::new(102, 221, 170, false),
                PixelColor::new(170, 238, 204, false),
                // Teals/Cyans (8)
                PixelColor::new(51, 187, 187, false),
                PixelColor::new(34, 153, 153, false),
                PixelColor::new(17, 119, 119, false),
                PixelColor::new(0, 85, 85, false),
                PixelColor::new(102, 204, 204, false),
                PixelColor::new(153, 221, 221, false),
                PixelColor::new(204, 238, 238, false),
                PixelColor::new(0, 204, 204, false),
                // Blues (12)
                PixelColor::new(51, 102, 221, false),
                PixelColor::new(34, 85, 204, false),
                PixelColor::new(17, 68, 170, false),
                PixelColor::new(0, 51, 136, false),
                PixelColor::new(0, 34, 102, false),
                PixelColor::new(102, 153, 238, false),
                PixelColor::new(153, 187, 255, false),
                PixelColor::new(187, 212, 255, false),
                PixelColor::new(221, 238, 255, false),
                PixelColor::new(68, 119, 255, false),
                PixelColor::new(0, 102, 204, false),
                PixelColor::new(0, 68, 170, false),
                // Purples (10)
                PixelColor::new(153, 68, 238, false),
                PixelColor::new(136, 51, 221, false),
                PixelColor::new(119, 34, 204, false),
                PixelColor::new(102, 17, 187, false),
                PixelColor::new(85, 0, 170, false),
                PixelColor::new(187, 119, 238, false),
                PixelColor::new(204, 153, 255, false),
                PixelColor::new(221, 187, 255, false),
                PixelColor::new(238, 221, 255, false),
                PixelColor::new(102, 68, 255, false),
                // Pinks (8)
                PixelColor::new(238, 68, 170, false),
                PixelColor::new(221, 51, 153, false),
                PixelColor::new(204, 34, 136, false),
                PixelColor::new(187, 17, 119, false),
                PixelColor::new(255, 119, 187, false),
                PixelColor::new(255, 170, 221, false),
                PixelColor::new(255, 204, 238, false),
                PixelColor::new(255, 238, 247, false),
                // Browns (8)
                PixelColor::new(139, 94, 60, false),
                PixelColor::new(119, 80, 50, false),
                PixelColor::new(99, 66, 40, false),
                PixelColor::new(79, 52, 30, false),
                PixelColor::new(58, 34, 16, false),
                PixelColor::new(160, 120, 80, false),
                PixelColor::new(200, 160, 120, false),
                PixelColor::new(240, 200, 160, false),
                // Skin tones (6)
                PixelColor::new(255, 212, 184, false),
                PixelColor::new(240, 190, 160, false),
                PixelColor::new(220, 168, 136, false),
                PixelColor::new(200, 146, 112, false),
                PixelColor::new(180, 124, 88, false),
                PixelColor::new(160, 102, 64, false),
            ],
        }
    }
}

#[derive(Default)]
pub struct PaletteGridState {
    pub selected: u8,
    pub secondary: u8,
    pub scroll: u16,
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

#[allow(clippy::too_many_arguments)]
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
                } else { (on_left || on_right) && !on_top && !on_bottom && row % 2 == 0 }
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

        let bw = self.blocks[0].term_width as u16;
        let bh = self.blocks[0].term_height as u16;
        let gap = self.gap as u16;

        let blocks_per_row = ((palette_area.width + gap) / (bw + gap)).max(1);
        let total_rows = (self.blocks.len() as u16).div_ceil(blocks_per_row);
        let visible_area_h = palette_area.height;
        let visible_rows = visible_area_h / (bh + gap);
        let max_scroll = total_rows.saturating_sub(visible_rows);
        let scroll = self.state.scroll.min(max_scroll);

        // Draw scrollbar on the right padding column
        let show_scrollbar = total_rows > visible_rows;
        if show_scrollbar {
            let sb_x = palette_area.x + palette_area.width;
            let sb_h = palette_area.height;
            let thumb_h = (sb_h as u32 * visible_rows as u32 / total_rows as u32).max(1) as u16;
            let thumb_y = palette_area.y + (sb_h - thumb_h) * scroll / max_scroll;
            for sy in 0..sb_h {
                let y = palette_area.y + sy;
                if let Some(cell) = buf.cell_mut(Position::new(sb_x, y)) {
                    if y >= thumb_y && y < thumb_y + thumb_h {
                        cell.set_char('█').set_fg(Color::DarkGray);
                    } else {
                        cell.set_char('▒').set_fg(Color::Gray);
                    }
                }
            }
        }

        for (block_idx, block) in self.blocks.iter().enumerate() {
            let row_idx = block_idx as u16 / blocks_per_row;
            let col_idx = block_idx as u16 % blocks_per_row;

            if row_idx < scroll {
                continue;
            }
            let vis_row = row_idx - scroll;
            let base_y = palette_area.y + vis_row * (bh + gap);
            if base_y + bh > palette_area.y + palette_area.height {
                continue;
            }

            let base_x = palette_area.x + col_idx * (bw + gap);

            let is_primary = block_idx == self.state.selected as usize;
            let is_secondary = block_idx == self.state.secondary as usize;

            for row in 0..bh {
                for col in 0..bw {
                    if let Some(cell) =
                        buf.cell_mut(Position::new(base_x + col, base_y + row))
                    {
                        cell.set_char('▀')
                            .set_fg(block.color.into())
                            .set_bg(block.color.into());
                    }
                }
            }

            if is_primary {
                render_block_border(
                    buf, base_x, base_y, bw, bh,
                    Color::White, block.color, false,
                );
            }

            if is_secondary && !is_primary {
                render_block_border(
                    buf, base_x, base_y, bw, bh,
                    Color::Blue, block.color, false,
                );
            }
        }
    }
}
