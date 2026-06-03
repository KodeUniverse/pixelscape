use std::path::Path;

use crate::app::EventMode;
use crate::pixels::{composite_layers, Layer, PixelColor, PixelGrid, ProjectFile};
use crate::routes::editor::color_palette::{
    ColorPalette, ColorPaletteGrid, PaletteGridBlock, PaletteGridState,
};
use crate::routes::editor::pixel_canvas::PixelCanvas;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, HorizontalAlignment, Layout, Margin, Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrushType {
    Solid,
    Dither,
}

impl BrushType {
    pub fn name(&self) -> &str {
        match self {
            BrushType::Solid => "Solid",
            BrushType::Dither => "Dither",
        }
    }
}

pub struct Editor {
    pub canvas: PixelCanvas,
    pub layers: Vec<Layer>,
    pub active_layer: usize,
    pub saving: bool,
    pub exporting: bool,
    pub input: String,
    pub event_mode: EventMode,
    pub palette_colors: Vec<PixelColor>,
    pub palette_primary_index: u8,
    pub palette_secondary_index: u8,
    pub brush_size: u8,
    pub brush_type: BrushType,
    pub canvas_area: Option<Rect>,
    pub palette_area: Option<Rect>,
    pub brush_area: Option<Rect>,
    pub brush_type_solid_area: Option<Rect>,
    pub brush_type_dither_area: Option<Rect>,
    pub brush_size_dec_area: Option<Rect>,
    pub brush_size_inc_area: Option<Rect>,
}
impl Editor {
    pub fn start_with_file(file: &Path) -> Self {
        let project = ProjectFile::read_from_file(file).unwrap_or_else(|e| {
            log::error!("Failed to read project file: {:?}", e);
            panic!("Failed to read project file: {:?}", e);
        });
        let layers: Vec<Layer> = project.layers.into_iter().map(Layer::from).collect();
        let composite = composite_layers(&layers);
        let palette = ColorPalette::default();
        Self {
            canvas: PixelCanvas::from_grid(composite),
            layers,
            active_layer: 0,
            saving: false,
            exporting: false,
            input: String::default(),
            event_mode: EventMode::Normal,
            palette_colors: palette.colors,
            palette_primary_index: 0,
            palette_secondary_index: 0,
            brush_size: 1,
            brush_type: BrushType::Solid,
            canvas_area: None,
            palette_area: None,
            brush_area: None,
            brush_type_solid_area: None,
            brush_type_dither_area: None,
            brush_size_dec_area: None,
            brush_size_inc_area: None,
        }
    }

    pub fn composite_and_render(&mut self) {
        self.canvas.grid = composite_layers(&self.layers);
    }

    pub fn paint_primary(&mut self, cx: u16, cy: u16) {
        let primary = self.palette_colors[self.palette_primary_index as usize];
        self.paint(cx, cy, primary);
    }

    pub fn paint_secondary(&mut self, cx: u16, cy: u16) {
        let secondary = self.palette_colors[self.palette_secondary_index as usize];
        self.paint(cx, cy, secondary);
    }

    fn paint(&mut self, cx: u16, cy: u16, color: PixelColor) {
        let layer = &mut self.layers[self.active_layer];
        let w = layer.grid.width;
        let h = layer.grid.height;
        let half = self.brush_size as u16 / 2;

        let x0 = cx.saturating_sub(half);
        let x1 = (cx + half).min(w - 1);
        let y0 = cy.saturating_sub(half);
        let y1 = (cy + half).min(h - 1);

        for x in x0..=x1 {
            for y in y0..=y1 {
                let use_secondary = self.brush_type == BrushType::Dither
                    && (x as usize + y as usize) % 2 == 1;
                let c = if use_secondary {
                    self.palette_colors[self.palette_secondary_index as usize]
                } else {
                    color
                };
                layer.grid.get_mut(x, y).color = c;
            }
        }
    }
}
impl Default for Editor {
    fn default() -> Self {
        let canvas = PixelCanvas::default();
        let w = canvas.grid.width;
        let h = canvas.grid.height;
        let grid = PixelGrid::new_transparent(w, h);
        let layer = Layer::new("Layer 1", grid);
        let composite = composite_layers(&[layer.clone()]);
        let palette = ColorPalette::default();
        Self {
            canvas: PixelCanvas::from_grid(composite),
            layers: vec![layer],
            active_layer: 0,
            saving: false,
            exporting: false,
            input: String::default(),
            event_mode: EventMode::Normal,
            palette_colors: palette.colors,
            palette_primary_index: 0,
            palette_secondary_index: 0,
            brush_size: 1,
            brush_type: BrushType::Solid,
            canvas_area: None,
            palette_area: None,
            brush_area: None,
            brush_type_solid_area: None,
            brush_type_dither_area: None,
            brush_size_dec_area: None,
            brush_size_inc_area: None,
        }
    }
}

fn render_button(buf: &mut Buffer, x: u16, y: u16, label: &str, active: bool) {
    let style = if active {
        Style::default().fg(Color::White).bg(Color::Rgb(60, 60, 60))
    } else {
        Style::default().fg(Color::Gray)
    };
    for (ci, ch) in label.chars().enumerate() {
        if let Some(c) = buf.cell_mut(Position::new(x + ci as u16, y)) {
            c.set_char(ch);
            c.set_style(style);
        }
    }
}

impl Widget for &mut Editor {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.composite_and_render();

        let block = Block::bordered()
            .title_top(" Pixel Editor ")
            .title_alignment(HorizontalAlignment::Center)
            .border_type(BorderType::Thick);
        (&block).render(area, buf);
        let inner = block.inner(area);

        let outer_layout =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(inner);
        let inner_layout_def = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ]);
        let inner_layout = inner_layout_def.split(inner);
        let inner_layout_save = inner_layout_def.split(outer_layout[0]);

        let mut render_all = |areas: &[Rect], buf: &mut Buffer| {
            let left_panel_layout =
                Layout::vertical([Constraint::Percentage(87), Constraint::Percentage(13)])
                    .split(areas[0]);
            let right_panel_layout =
                Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(areas[2]);

            // --- Left panel: Palette ---
            let palette_card = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(" Color Palette ")
                .title_alignment(HorizontalAlignment::Center);
            let palette_card_inner = palette_card.inner(left_panel_layout[0]);
            palette_card.render(left_panel_layout[0], buf);

            let mut color_blocks = Vec::<PaletteGridBlock>::new();
            self.palette_colors
                .iter()
                .for_each(|px| color_blocks.push(PaletteGridBlock::new(*px, 3)));

            let palette_state = PaletteGridState {
                selected: self.palette_primary_index,
                secondary: self.palette_secondary_index,
            };
            let color_palette = ColorPaletteGrid::new(color_blocks, 1, palette_state);
            color_palette.render(palette_card_inner, buf);

            self.palette_area = Some(palette_card_inner.inner(Margin::new(1, 1)));

            // --- Left panel: Brush ---
            let brush_card = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(" Brush ")
                .title_alignment(HorizontalAlignment::Center);
            let brush_card_inner = brush_card.inner(left_panel_layout[1]);
            brush_card.render(left_panel_layout[1], buf);

            self.brush_area = Some(brush_card_inner);

            // Type buttons
            let type_y = brush_card_inner.y + 1;
            let solid_label = " [Solid] ";
            let dither_label = " [Dither] ";
            let total_type_w = solid_label.len() + dither_label.len() + 1;
            let type_x = brush_card_inner.x
                + (brush_card_inner.width.saturating_sub(total_type_w as u16)) / 2;

            let solid_active = matches!(self.brush_type, BrushType::Solid);
            render_button(buf, type_x, type_y, solid_label, solid_active);
            self.brush_type_solid_area =
                Some(Rect::new(type_x, type_y, solid_label.len() as u16, 1));

            let dx = type_x + solid_label.len() as u16 + 1;
            let dither_active = matches!(self.brush_type, BrushType::Dither);
            render_button(buf, dx, type_y, dither_label, dither_active);
            self.brush_type_dither_area =
                Some(Rect::new(dx, type_y, dither_label.len() as u16, 1));

            // Size controls
            let size_y = type_y + 2;
            let size_dec_label = " [<] ";
            let size_val_label = format!(" {}px ", self.brush_size);
            let size_inc_label = " [>] ";
            let total_size_w =
                size_dec_label.len() + size_val_label.len() + size_inc_label.len();
            let size_x = brush_card_inner.x
                + (brush_card_inner.width.saturating_sub(total_size_w as u16)) / 2;

            render_button(buf, size_x, size_y, size_dec_label, false);
            self.brush_size_dec_area =
                Some(Rect::new(size_x, size_y, size_dec_label.len() as u16, 1));

            let svx = size_x + size_dec_label.len() as u16;
            let val_style = Style::default().fg(Color::White);
            for (ci, ch) in size_val_label.chars().enumerate() {
                if let Some(c) = buf.cell_mut(Position::new(svx + ci as u16, size_y)) {
                    c.set_char(ch);
                    c.set_style(val_style);
                }
            }

            let six = svx + size_val_label.len() as u16;
            render_button(buf, six, size_y, size_inc_label, false);
            self.brush_size_inc_area =
                Some(Rect::new(six, size_y, size_inc_label.len() as u16, 1));

            // --- Right panel: Keybinds ---
            let keybinds_card = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(" Keybinds ")
                .title_alignment(HorizontalAlignment::Center);
            let keybinds_card_inner = keybinds_card.inner(right_panel_layout[0]);
            keybinds_card.render(right_panel_layout[0], buf);

            let keybinds = [
                ("Space", "Paint primary"),
                ("BckSp", "Paint secondary"),
                ("x", "Erase"),
                ("F", "Fill"),
                ("B", "Brush type"),
                ("+/-", "Brush size"),
                ("L", "New layer"),
                ("Del", "Del layer"),
                ("[/]", "Cycle layer"),
                ("V", "Toggle layer"),
                ("E", "Edit color"),
                ("Q", "Swap colors"),
                ("Tab", "Next color"),
                ("S", "Save"),
                ("X", "Export PNG"),
                ("q", "Quit"),
            ];
            let kb_style = Style::default().fg(Color::Gray);
            for (i, (key, desc)) in keybinds.iter().enumerate() {
                let ky = keybinds_card_inner.y + i as u16;
                let label = format!(" {:<6} {}", key, desc);
                for (ci, ch) in label.chars().enumerate() {
                    if let Some(c) =
                        buf.cell_mut(Position::new(keybinds_card_inner.x + ci as u16, ky))
                    {
                        c.set_char(ch);
                        c.set_style(kb_style);
                    }
                }
            }

            // --- Right panel: Layers ---
            let layers_card = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_top(" Layers ")
                .title_alignment(HorizontalAlignment::Center);
            let layers_card_inner = layers_card.inner(right_panel_layout[1]);
            layers_card.render(right_panel_layout[1], buf);

            for (i, layer) in self.layers.iter().enumerate() {
                let vis = if layer.visible { "v" } else { " " };
                let marker = if i == self.active_layer { ">" } else { " " };
                let label = format!(" {} [{}] {}", marker, vis, layer.name);
                let style = if i == self.active_layer {
                    Style::default().fg(Color::White).bg(Color::Rgb(60, 60, 60))
                } else {
                    Style::default().fg(Color::Gray)
                };
                if buf.cell_mut(
                    ratatui::layout::Position::new(layers_card_inner.x, layers_card_inner.y + i as u16),
                ).is_some() {
                    for (ci, ch) in label.chars().enumerate() {
                        if let Some(c) = buf.cell_mut(
                            ratatui::layout::Position::new(
                                layers_card_inner.x + ci as u16,
                                layers_card_inner.y + i as u16,
                            ),
                        ) {
                            c.set_char(ch);
                            c.set_style(style);
                        }
                    }
                }
            }

            // --- Center: Canvas ---
            let (x_off, y_off, _, _) = self.canvas.render_with(areas[1], buf, self.brush_size);
            self.canvas_area = Some(Rect::new(x_off, y_off, self.canvas.grid.width, (self.canvas.grid.height + 1) / 2));
        };

        if !self.saving && !self.exporting {
            render_all(&inner_layout, buf);
        } else if self.saving {
            let save_area = outer_layout[1];
            let save_prompt = Paragraph::new(format!("Save: {}.pxsc", self.input));
            (&save_prompt).render(save_area, buf);
            render_all(&inner_layout_save, buf);
        } else if self.exporting {
            let export_area = outer_layout[1];
            let export_text = Paragraph::new(format!("Export: {}.png", self.input));
            (&export_text).render(export_area, buf);
            render_all(&inner_layout_save, buf);
        }
    }
}
