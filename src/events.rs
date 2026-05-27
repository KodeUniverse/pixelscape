use crate::app::{App, EventMode, Route};

use crate::pixels::PixelColor;
use crate::routes::editor::color_palette::ColorPalette;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match read_event()? {
        Some(event) => match app.route {
            Route::Home => handle_home(app, event),
            Route::Editor => match app.editor.event_mode {
                EventMode::Normal => handle_editor(app, event),
                EventMode::Input => handle_input_editor(app, event),
            },
        },
        None => Ok(()),
    }
}
fn read_event() -> io::Result<Option<KeyEvent>> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => Ok(Some(key_event)),
        _ => Ok(None),
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
                    if let Err(e) = app.editor.canvas.grid.save_to_file(path) {
                        log::error!("Failed to save: {:?}", e);
                    }
                } else if is_exporting {
                    filename += ".png";
                    let path = std::path::Path::new(&filename);
                    if let Err(e) = app.editor.canvas.grid.export_to_png(path) {
                        log::error!("Failed to export: {:?}", e);
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
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Up => app.editor.canvas.move_select_up(1),
        KeyCode::Down => app.editor.canvas.move_select_down(1),
        KeyCode::Left => app.editor.canvas.move_select_left(1),
        KeyCode::Right => app.editor.canvas.move_select_right(1),

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

        // Color and Pixel editing
        KeyCode::Char(' ') => {
            let (cur_x, cur_y) = (app.editor.canvas.cursor.x, app.editor.canvas.cursor.y);
            let sel_pix = app.editor.canvas.grid.get_mut(cur_x, cur_y);
            sel_pix.color = PixelColor::from(
                ColorPalette::default().colors[app.editor.palette_selected_index as usize],
            );
        }

        // Escape events
        KeyCode::Esc if app.editor.saving => app.editor.saving = false,
        KeyCode::Esc if app.editor.exporting => app.editor.exporting = false,
        KeyCode::Tab => {
            let palette_len = ColorPalette::default().colors.len() as u8;
            app.editor.palette_selected_index =
                (app.editor.palette_selected_index + 1) % palette_len;
        }
        KeyCode::BackTab => {
            let palette_len = ColorPalette::default().colors.len() as u8;
            app.editor.palette_selected_index =
                (app.editor.palette_selected_index + palette_len - 1) % palette_len;
        }

        // Vim Keybindings
        KeyCode::Char('G') => {
            app.editor
                .canvas
                .move_select_down(app.editor.canvas.grid.height - 1);
        }
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
                0 => app.route = Route::Editor, //create project,
                1 => (),                        //existing
                2 => (),                        //settings
                _ => panic!(),                  // error
            }
        }
        _ => {}
    }
    Ok(())
}
