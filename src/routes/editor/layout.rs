use std::path::Path;

use crate::app::EventMode;
use crate::pixels::{Layer, PixelColor, PixelGrid, ProjectFile, composite_layers};
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
    Eraser,
    Fill,
}

impl BrushType {
    pub fn name(&self) -> &str {
        match self {
            BrushType::Solid => "Solid",
            BrushType::Dither => "Dither",
            BrushType::Eraser => "Eraser",
            BrushType::Fill => "Fill",
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
    pub palette_scroll: u16,
    pub canvas_area: Option<Rect>,
    pub palette_area: Option<Rect>,
    pub brush_area: Option<Rect>,
    pub brush_type_solid_area: Option<Rect>,
    pub brush_type_dither_area: Option<Rect>,
    pub brush_slider_area: Option<Rect>,
    pub last_paint_pos: Option<(u16, u16)>,
    pub eraser_btn_area: Option<Rect>,
    pub fill_btn_area: Option<Rect>,
    pub layers_card_area: Option<Rect>,
    pub layer_add_area: Option<Rect>,
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
            palette_scroll: 0,
            canvas_area: None,
            palette_area: None,
            brush_area: None,
            brush_type_solid_area: None,
            brush_type_dither_area: None,
            brush_slider_area: None,
            last_paint_pos: None,
            eraser_btn_area: None,
            fill_btn_area: None,
            layers_card_area: None,
            layer_add_area: None,
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
        if self.brush_type == BrushType::Fill {
            layer.grid.flood_fill(cx, cy, color);
            return;
        }
        let w = layer.grid.width;
        let h = layer.grid.height;
        let half = self.brush_size as u16 / 2;

        let x0 = cx.saturating_sub(half);
        let x1 = (cx + half).min(w - 1);
        let y0 = cy.saturating_sub(half);
        let y1 = (cy + half).min(h - 1);

        for x in x0..=x1 {
            for y in y0..=y1 {
                if self.brush_type == BrushType::Eraser {
                    layer.grid.get_mut(x, y).color = PixelColor::new(0, 0, 0, true);
                } else {
                    let use_secondary =
                        self.brush_type == BrushType::Dither && (x as usize + y as usize) % 2 == 1;
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

    pub fn paint_line_primary(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        let primary = self.palette_colors[self.palette_primary_index as usize];
        self.paint_line(x0, y0, x1, y1, primary);
    }

    pub fn paint_line_secondary(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        let secondary = self.palette_colors[self.palette_secondary_index as usize];
        self.paint_line(x0, y0, x1, y1, secondary);
    }

    fn paint_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: PixelColor) {
        let dx = (x1 as i16 - x0 as i16).abs();
        let dy = -(y1 as i16 - y0 as i16).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0 as i16;
        let mut y = y0 as i16;
        loop {
            self.paint(x as u16, y as u16, color);
            if x == x1 as i16 && y == y1 as i16 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn paint_erase(&mut self, cx: u16, cy: u16) {
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
                layer.grid.get_mut(x, y).color = PixelColor::new(0, 0, 0, true);
            }
        }
    }

    pub fn paint_line_erase(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        let dx = (x1 as i16 - x0 as i16).abs();
        let dy = -(y1 as i16 - y0 as i16).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0 as i16;
        let mut y = y0 as i16;
        loop {
            self.paint_erase(x as u16, y as u16);
            if x == x1 as i16 && y == y1 as i16 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
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
        let composite = composite_layers(std::slice::from_ref(&layer));
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
            palette_scroll: 0,
            canvas_area: None,
            palette_area: None,
            brush_area: None,
            brush_type_solid_area: None,
            brush_type_dither_area: None,
            brush_slider_area: None,
            last_paint_pos: None,
            eraser_btn_area: None,
            fill_btn_area: None,
            layers_card_area: None,
            layer_add_area: None,
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
                Layout::vertical([Constraint::Min(0), Constraint::Length(5)]).split(areas[0]);
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
                scroll: self.palette_scroll,
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

            // Type buttons + Eraser + Fill
            let type_y = brush_card_inner.y;
            let solid_label = "[Solid]";
            let dither_label = "[Dither]";
            let eraser_label = "[Eraser]";
            let fill_label = "[Fill]";
            let total_btn_w =
                solid_label.len() + dither_label.len() + eraser_label.len() + fill_label.len() + 3;
            let btn_x = brush_card_inner.x
                + (brush_card_inner.width.saturating_sub(total_btn_w as u16)) / 2;

            let solid_active = matches!(self.brush_type, BrushType::Solid);
            render_button(buf, btn_x, type_y, solid_label, solid_active);
            self.brush_type_solid_area =
                Some(Rect::new(btn_x, type_y, solid_label.len() as u16, 1));

            let dx = btn_x + solid_label.len() as u16 + 1;
            let dither_active = matches!(self.brush_type, BrushType::Dither);
            render_button(buf, dx, type_y, dither_label, dither_active);
            self.brush_type_dither_area = Some(Rect::new(dx, type_y, dither_label.len() as u16, 1));

            let ex = dx + dither_label.len() as u16 + 1;
            let eraser_active = matches!(self.brush_type, BrushType::Eraser);
            render_button(buf, ex, type_y, eraser_label, eraser_active);
            self.eraser_btn_area = Some(Rect::new(ex, type_y, eraser_label.len() as u16, 1));

            let fx = ex + eraser_label.len() as u16 + 1;
            let fill_active = matches!(self.brush_type, BrushType::Fill);
            render_button(buf, fx, type_y, fill_label, fill_active);
            self.fill_btn_area = Some(Rect::new(fx, type_y, fill_label.len() as u16, 1));

            // Size slider
            let slider_y = type_y + 1;
            let min_label = "1";
            let max_label = "21";
            let gap_s: usize = 1;
            let size_label = format!(" {}px", self.brush_size);
            let track_w = brush_card_inner.width as usize
                - 3
                - min_label.len()
                - max_label.len()
                - gap_s * 2
                - size_label.len();
            let track_w = track_w.max(3);

            let slider_x = brush_card_inner.x + 1;
            let min_x = slider_x;
            let track_x = min_x + min_label.len() as u16 + gap_s as u16;
            let track_len = track_w as u16;
            let max_x = track_x + track_len + gap_s as u16;

            let slider_style = Style::default().fg(Color::Gray);
            for (ci, ch) in min_label.chars().enumerate() {
                if let Some(c) = buf.cell_mut(Position::new(min_x + ci as u16, slider_y)) {
                    c.set_char(ch);
                    c.set_style(slider_style);
                }
            }
            for (ci, ch) in max_label.chars().enumerate() {
                if let Some(c) = buf.cell_mut(Position::new(max_x + ci as u16, slider_y)) {
                    c.set_char(ch);
                    c.set_style(slider_style);
                }
            }

            for (ci, ch) in size_label.chars().enumerate() {
                let x = max_x + max_label.len() as u16 + ci as u16;
                if let Some(c) = buf.cell_mut(Position::new(x, slider_y)) {
                    c.set_char(ch);
                    c.set_style(slider_style);
                }
            }

            let thumb_col = track_x + (self.brush_size - 1) as u16 * (track_len - 1) / 20;
            for ci in 0..track_len {
                let x = track_x + ci;
                let ch = if x == thumb_col { '■' } else { '─' };
                let style = if x == thumb_col {
                    Style::default().fg(Color::White)
                } else {
                    slider_style
                };
                if let Some(c) = buf.cell_mut(Position::new(x, slider_y)) {
                    c.set_char(ch);
                    c.set_style(style);
                }
            }
            self.brush_slider_area = Some(Rect::new(track_x, slider_y, track_len, 1));

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
            self.layers_card_area = Some(layers_card_inner);

            let max_name_w = layers_card_inner.width.saturating_sub(5) as usize;
            for (i, layer) in self.layers.iter().enumerate() {
                let ly = layers_card_inner.y + i as u16;
                let style = if i == self.active_layer {
                    Style::default().fg(Color::White).bg(Color::Rgb(60, 60, 60))
                } else {
                    Style::default().fg(Color::Gray)
                };

                let vis_str = if layer.visible { "[v]" } else { "[ ]" };
                for (ci, ch) in vis_str.chars().enumerate() {
                    if let Some(c) =
                        buf.cell_mut(Position::new(layers_card_inner.x + ci as u16, ly))
                    {
                        c.set_char(ch);
                        c.set_fg(if layer.visible {
                            Color::White
                        } else {
                            Color::Gray
                        });
                    }
                }

                for (ci, ch) in layer.name.chars().enumerate().take(max_name_w) {
                    if let Some(c) =
                        buf.cell_mut(Position::new(layers_card_inner.x + 4 + ci as u16, ly))
                    {
                        c.set_char(ch);
                        c.set_style(style);
                    }
                }

                if let Some(c) = buf.cell_mut(Position::new(
                    layers_card_inner.x + layers_card_inner.width - 1,
                    ly,
                )) {
                    c.set_char('×');
                    c.set_style(style);
                }
            }

            // Add layer button
            let add_ly = layers_card_inner.y + self.layers.len() as u16 + 1;
            if add_ly < layers_card_inner.y + layers_card_inner.height {
                let add_label = "+ Add Layer";
                let add_x = layers_card_inner.x
                    + (layers_card_inner
                        .width
                        .saturating_sub(add_label.len() as u16))
                        / 2;
                let add_style = Style::default().fg(Color::Gray);
                for (ci, ch) in add_label.chars().enumerate() {
                    if let Some(c) = buf.cell_mut(Position::new(add_x + ci as u16, add_ly)) {
                        c.set_char(ch);
                        c.set_style(add_style);
                    }
                }
                self.layer_add_area = Some(Rect::new(add_x, add_ly, add_label.len() as u16, 1));
            } else {
                self.layer_add_area = None;
            }

            // --- Center: Canvas ---
            let grid_w = self.canvas.grid.width;
            let grid_h_rows = self.canvas.grid.height.div_ceil(2);
            let bw = grid_w + 2;
            let bh = grid_h_rows + 2;
            if bw <= areas[1].width && bh <= areas[1].height {
                let bx = areas[1].x + (areas[1].width - bw) / 2;
                let by = areas[1].y + (areas[1].height - bh) / 2;
                let border_rect = Rect::new(bx, by, bw, bh);
                let canvas_block = Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Gray));
                let canvas_inner = canvas_block.inner(border_rect);
                canvas_block.render(border_rect, buf);
                let (x_off, y_off, _, _) =
                    self.canvas.render_with(canvas_inner, buf, self.brush_size);
                self.canvas_area = Some(Rect::new(x_off, y_off, grid_w, grid_h_rows));
            } else {
                let (x_off, y_off, _, _) = self.canvas.render_with(areas[1], buf, self.brush_size);
                self.canvas_area = Some(Rect::new(x_off, y_off, grid_w, grid_h_rows));
            }
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
