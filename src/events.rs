use crate::app::{App, EventMode, Route};
use crate::pixels::{Layer, PixelColor, PixelGrid, ProjectFile, SerializedLayer};
use crate::routes::editor::layout::BrushType;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};
use std::io;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match read_event()? {
        Some(Event::Mouse(me)) => {
            if matches!(app.route, Route::Editor) {
                handle_mouse_editor(app, me);
            }
            Ok(())
        }
        Some(Event::Key(key_event)) => match app.route {
            Route::Home => handle_home(app, key_event),
            Route::Editor => match app.editor.event_mode {
                EventMode::Normal => handle_editor(app, key_event),
                EventMode::Input => handle_input_editor(app, key_event),
            },
        },
        None => Ok(()),
        _ => Ok(()),
    }
}

fn read_event() -> io::Result<Option<Event>> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            Ok(Some(Event::Key(key_event)))
        }
        Event::Mouse(me) => Ok(Some(Event::Mouse(me))),
        _ => Ok(None),
    }
}

fn handle_mouse_editor(app: &mut App, me: crossterm::event::MouseEvent) {
    let mx = me.column;
    let my = me.row;

    match me.kind {
        MouseEventKind::Up(_) => {
            app.editor.last_paint_pos = None;
            app.editor.mouse_down = false;
        }
        MouseEventKind::Down(btn) | MouseEventKind::Drag(btn) => {
            if matches!(me.kind, MouseEventKind::Down(_)) {
                app.editor.mouse_down = true;
            }
            // Check palette click/drag
            if let Some(palette_area) = app.editor.palette_area {
                if mx >= palette_area.x
                    && mx < palette_area.x + palette_area.width
                    && my >= palette_area.y
                    && my < palette_area.y + palette_area.height
                {
                    let block_count = app.editor.palette_colors.len() as u16;
                    let bw: u16 = 6;
                    let bh: u16 = 3;
                    let gap: u16 = 1;
                    let blocks_per_row = (palette_area.width + gap) / (bw + gap);
                    if blocks_per_row == 0 {
                        return;
                    }
                    let rel_x = mx - palette_area.x;
                    let rel_y = my - palette_area.y;
                    let col = rel_x / (bw + gap);
                    let row = rel_y / (bh + gap);
                    let index = row * blocks_per_row + col;
                    if index < block_count {
                        match btn {
                            MouseButton::Left => {
                                app.editor.palette_primary_index = index as u8;
                            }
                            MouseButton::Right => {
                                app.editor.palette_secondary_index = index as u8;
                            }
                            _ => {}
                        }
                    }
                    return;
                }
            }

            // Check brush type buttons (+ Eraser)
            if let Some(area) = app.editor.brush_type_solid_area {
                if mx >= area.x && mx < area.x + area.width && my == area.y {
                    if let MouseButton::Left = btn {
                        app.editor.brush_type = BrushType::Solid;
                    }
                    return;
                }
            }
            if let Some(area) = app.editor.brush_type_dither_area {
                if mx >= area.x && mx < area.x + area.width && my == area.y {
                    if let MouseButton::Left = btn {
                        app.editor.brush_type = BrushType::Dither;
                    }
                    return;
                }
            }
            if let Some(area) = app.editor.eraser_btn_area {
                if mx >= area.x && mx < area.x + area.width && my == area.y {
                    if let MouseButton::Left = btn {
                        app.editor.brush_type = BrushType::Eraser;
                    }
                    return;
                }
            }

            // Check brush slider
            if let Some(area) = app.editor.brush_slider_area {
                if mx >= area.x && mx < area.x + area.width && my == area.y {
                    if let MouseButton::Left = btn {
                        let rel_x = mx - area.x;
                        let track_len = area.width;
                        let value_idx = rel_x * 10 / (track_len - 1).max(1);
                        let value_idx = value_idx.min(10);
                        app.editor.brush_size = value_idx as u8 * 2 + 1;
                    }
                    return;
                }
            }

            // Layer clicks - one-shot on Down only
            if matches!(me.kind, MouseEventKind::Down(_)) {
                if let Some(la) = app.editor.layers_card_area {
                    let add_y = la.y + app.editor.layers.len() as u16 + 1;
                    if my == add_y && mx >= la.x && mx < la.x + la.width {
                        let w = app.editor.canvas.grid.width;
                        let h = app.editor.canvas.grid.height;
                        let new_grid = PixelGrid::new_transparent(w, h);
                        let name = format!("Layer {}", app.editor.layers.len() + 1);
                        let layer = Layer::new(&name, new_grid);
                        app.editor.active_layer = app.editor.layers.len();
                        app.editor.layers.push(layer);
                        return;
                    }
                    for i in 0..app.editor.layers.len() as u16 {
                        let ly = la.y + i;
                        if my == ly && mx >= la.x && mx < la.x + la.width {
                            if mx >= la.x && mx < la.x + 3 {
                                app.editor.layers[i as usize].visible =
                                    !app.editor.layers[i as usize].visible;
                                return;
                            }
                            if mx == la.x + la.width - 1 {
                                if app.editor.layers.len() > 1 {
                                    app.editor.layers.remove(i as usize);
                                    if app.editor.active_layer >= app.editor.layers.len() {
                                        app.editor.active_layer = app.editor.layers.len() - 1;
                                    }
                                }
                                return;
                            }
                            app.editor.active_layer = i as usize;
                            return;
                        }
                    }
                }
            }
        }
        _ => {}
    }

    // Canvas — always update cursor, paint on Down/Drag with line interpolation
    if let Some(canvas_area) = app.editor.canvas_area {
        let w = app.editor.canvas.grid.width;
        let h = app.editor.canvas.grid.height;
        let terminal_rows = (h + 1) / 2;

        if mx >= canvas_area.x
            && mx < canvas_area.x + w
            && my >= canvas_area.y
            && my < canvas_area.y + terminal_rows
        {
            let px = mx - canvas_area.x;
            let mut py = (my - canvas_area.y) * 2;
            if me.modifiers.contains(KeyModifiers::CONTROL) {
                py = (py + 1).min(h - 1);
            }

            if px < w && py < h {
                app.editor.canvas.cursor.x = px;
                app.editor.canvas.cursor.y = py;

                match me.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        app.editor.last_paint_pos = Some((px, py));
                        app.editor.paint_primary(px, py);
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        if app.editor.mouse_down {
                            if let Some((lx, ly)) = app.editor.last_paint_pos {
                                app.editor.paint_line_primary(lx, ly, px, py);
                            }
                            app.editor.last_paint_pos = Some((px, py));
                        }
                    }
                    MouseEventKind::Down(MouseButton::Right) => {
                        app.editor.last_paint_pos = Some((px, py));
                        app.editor.paint_secondary(px, py);
                    }
                    MouseEventKind::Drag(MouseButton::Right) => {
                        if app.editor.mouse_down {
                            if let Some((lx, ly)) = app.editor.last_paint_pos {
                                app.editor.paint_line_secondary(lx, ly, px, py);
                            }
                            app.editor.last_paint_pos = Some((px, py));
                        }
                    }
                    _ => {}
                }
            }
            return;
        }
    }
}

fn handle_input_editor(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    match key_event.code {
        KeyCode::Esc => {
            app.editor.saving = false;
            app.editor.exporting = false;
            app.editor.input.clear();
            app.editor.event_mode = EventMode::Normal;
        }
        KeyCode::Backspace if !app.editor.input.is_empty() => {
            app.editor.input.pop();
        }
        KeyCode::Enter => {
            let mut filename = std::mem::take(&mut app.editor.input);
            let is_saving = app.editor.saving;
            let is_exporting = app.editor.exporting;
            app.editor.saving = false;
            app.editor.exporting = false;
            app.editor.event_mode = EventMode::Normal;

            if !filename.is_empty() {
                if is_saving {
                    filename += ".pxsc";
                    let path = std::path::Path::new(&filename);
                    let serialized: Vec<SerializedLayer> = app
                        .editor
                        .layers
                        .iter()
                        .map(SerializedLayer::from)
                        .collect();
                    let project = ProjectFile {
                        layers: serialized,
                    };
                    if let Err(e) = project.save_to_file(path) {
                        log::error!("Failed to save: {:?}", e);
                    }
                } else if is_exporting {
                    filename += ".png";
                    let path = std::path::Path::new(&filename);
                    if let Err(e) = app.editor.canvas.grid.export_to_png(path) {
                        log::error!("Failed to export: {:?}", e);
                    }
                } else {
                    // Palette color edit
                    let parts: Vec<&str> = filename.split(',').collect();
                    if parts.len() == 3 {
                        let r = parts[0].trim().parse::<u8>().unwrap_or(0);
                        let g = parts[1].trim().parse::<u8>().unwrap_or(0);
                        let b = parts[2].trim().parse::<u8>().unwrap_or(0);
                        let idx = app.editor.palette_primary_index as usize;
                        if idx < app.editor.palette_colors.len() {
                            app.editor.palette_colors[idx] =
                                PixelColor::new(r, g, b, false);
                        }
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            app.editor.input.push(c);
        }
        _ => {}
    }
    Ok(())
}

fn handle_editor(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    let (cx, cy) = (
        app.editor.canvas.cursor.x,
        app.editor.canvas.cursor.y,
    );

    match key_event.code {
        KeyCode::Char('q') => app.exit(),

        // Cursor movement
        KeyCode::Up => app.editor.canvas.move_select_up(1),
        KeyCode::Down => app.editor.canvas.move_select_down(1),
        KeyCode::Left => app.editor.canvas.move_select_left(1),
        KeyCode::Right => app.editor.canvas.move_select_right(1),
        KeyCode::Char('G') => {
            app.editor
                .canvas
                .move_select_down(app.editor.canvas.grid.height - 1);
        }

        // Paint
        KeyCode::Char(' ') => {
            app.editor.paint_primary(cx, cy);
        }
        KeyCode::Backspace => {
            app.editor.paint_secondary(cx, cy);
        }

        // Erase
        KeyCode::Char('x') => {
            app.editor.paint_erase(cx, cy);
        }

        // Save / Export
        KeyCode::Char('S') => {
            app.editor.saving = true;
            app.editor.event_mode = EventMode::Input;
            app.editor.input.clear();
        }
        KeyCode::Char('X') => {
            app.editor.exporting = true;
            app.editor.event_mode = EventMode::Input;
            app.editor.input.clear();
        }

        // Palette selection
        KeyCode::Tab => {
            let len = app.editor.palette_colors.len() as u8;
            app.editor.palette_primary_index =
                (app.editor.palette_primary_index + 1) % len;
        }
        KeyCode::BackTab => {
            let len = app.editor.palette_colors.len() as u8;
            app.editor.palette_primary_index =
                (app.editor.palette_primary_index + len - 1) % len;
        }

        // Swap primary/secondary
        KeyCode::Char('Q') => {
            let tmp = app.editor.palette_primary_index;
            app.editor.palette_primary_index = app.editor.palette_secondary_index;
            app.editor.palette_secondary_index = tmp;
        }

        // Brush size
        KeyCode::Char('=') => {
            app.editor.brush_size = (app.editor.brush_size + 2).min(21);
        }
        KeyCode::Char('-') => {
            app.editor.brush_size = app.editor.brush_size.saturating_sub(2).max(1);
        }

        // Brush type
        KeyCode::Char('B') => {
            app.editor.brush_type = match app.editor.brush_type {
                BrushType::Solid => BrushType::Dither,
                BrushType::Dither => BrushType::Eraser,
                BrushType::Eraser => BrushType::Solid,
            };
        }

        // Fill
        KeyCode::Char('F') => {
            let primary = app.editor.palette_colors[app.editor.palette_primary_index as usize];
            let layer = &mut app.editor.layers[app.editor.active_layer];
            layer.grid.flood_fill(cx, cy, primary);
        }

        // Layers
        KeyCode::Char('L') => {
            let w = app.editor.canvas.grid.width;
            let h = app.editor.canvas.grid.height;
            let new_grid = PixelGrid::new_transparent(w, h);
            let name = format!("Layer {}", app.editor.layers.len() + 1);
            let layer = Layer::new(&name, new_grid);
            app.editor.active_layer = app.editor.layers.len();
            app.editor.layers.push(layer);
        }
        KeyCode::Delete => {
            if app.editor.layers.len() > 1 && app.editor.active_layer < app.editor.layers.len() {
                app.editor.layers.remove(app.editor.active_layer);
                if app.editor.active_layer >= app.editor.layers.len() {
                    app.editor.active_layer = app.editor.layers.len() - 1;
                }
            }
        }
        KeyCode::Char('[') => {
            if app.editor.active_layer > 0 {
                app.editor.active_layer -= 1;
            }
        }
        KeyCode::Char(']') => {
            if app.editor.active_layer + 1 < app.editor.layers.len() {
                app.editor.active_layer += 1;
            }
        }
        KeyCode::Char('V') => {
            let layer = &mut app.editor.layers[app.editor.active_layer];
            layer.visible = !layer.visible;
        }

        // Palette color edit
        KeyCode::Char('E') => {
            let idx = app.editor.palette_primary_index as usize;
            let c = app.editor.palette_colors[idx];
            app.editor.input = format!("{},{},{}", c.red, c.green, c.blue);
            app.editor.saving = false;
            app.editor.exporting = false;
            app.editor.event_mode = EventMode::Input;
        }

        // Escape cancels
        KeyCode::Esc if app.editor.saving => app.editor.saving = false,
        KeyCode::Esc if app.editor.exporting => app.editor.exporting = false,

        _ => {}
    }
    Ok(())
}

fn handle_home(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Up => {
            app.home_list_state.scroll_up_by(1);
        }
        KeyCode::Down => {
            app.home_list_state.scroll_down_by(1);
        }
        KeyCode::Enter => {
            let selection = app.home_list_state.selected_mut().unwrap_or(usize::MAX);
            match selection {
                0 => app.route = Route::Editor,
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
