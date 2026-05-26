use std::path::Path;

use crate::app::EventMode;
use crate::pixels::PixelGrid;
use crate::routes::editor::color_palette::{ColorPaletteGrid, PaletteGridBlock, PaletteGridState};
use crate::routes::editor::pixel_canvas::PixelCanvas;
use rand::random_range;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};

pub struct Editor {
    pub canvas: PixelCanvas,
    pub saving: bool,
    pub exporting: bool,
    pub input: String,
    pub event_mode: EventMode,
}
impl Editor {
    pub fn start_with_file(file: &Path) -> Self {
        let px_grid: PixelGrid = PixelGrid::read_from_file(file).unwrap();
        let canvas = PixelCanvas::from_grid(px_grid);
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

        let outer_layout =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(inner);
        let inner_layout_def = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ]);
        let inner_layout = inner_layout_def.split(inner);
        let inner_layout_save = inner_layout_def.split(outer_layout[0]);

        let mut default_render = |areas: &[Rect], buf: &mut Buffer| {
            let left_panel_layout =
                Layout::vertical([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(areas[0]);
            let right_panel_layout =
                Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(areas[2]);

            let left_panel = Block::default();
            let palette_card = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(" Color Palette ")
                .title_alignment(HorizontalAlignment::Center);
            let brush_size_card = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(" Brush Size ")
                .title_alignment(HorizontalAlignment::Center);
            left_panel.render(areas[0], buf);
            let palette_card_inner = palette_card.inner(left_panel_layout[0]);
            palette_card.render(left_panel_layout[0], buf);

            let color_blocks = vec![
                PaletteGridBlock::new(
                    Color::Rgb(
                        random_range(0..=255),
                        random_range(0..=255),
                        random_range(0..=255),
                    ),
                    3,
                );
                16
            ];

            let palette_state = PaletteGridState::default();
            let color_palette = ColorPaletteGrid::new(color_blocks, 1, palette_state);
            color_palette.render(palette_card_inner, buf);
            brush_size_card.render(left_panel_layout[1], buf);

            let right_panel = Block::default();
            let layers_card = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(" Layers ")
                .title_alignment(HorizontalAlignment::Center);
            let sprite_card = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(" Sprites ")
                .title_alignment(HorizontalAlignment::Center);
            sprite_card.render(right_panel_layout[0], buf);
            layers_card.render(right_panel_layout[1], buf);
            right_panel.render(areas[2], buf);

            (&mut self.canvas).render(areas[1], buf);
        };

        if !self.saving && !self.exporting {
            default_render(&inner_layout, buf);
        } else if self.saving {
            let save_area = outer_layout[1];
            let save_prompt = Paragraph::new(format!("Save project: {}.pxsc", self.input));
            (&save_prompt).render(save_area, buf);

            default_render(&inner_layout_save, buf);
        } else if self.exporting {
            let export_area = outer_layout[1];
            let export_text = Paragraph::new(format!("Export to PNG: {}.png", self.input));
            (&export_text).render(export_area, buf);
            default_render(&inner_layout_save, buf);
        }
    }
}
