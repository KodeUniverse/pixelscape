use crate::app::{App, Route};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;
pub fn handle_events(app: &mut App) -> io::Result<()> {
    match read_event()? {
        Some(event) => match app.route {
            Route::Home => handle_home(app, event),
            Route::Editor => handle_editor(app, event),
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

fn handle_editor(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Up => app.editor.canvas.move_select_up(1),
        KeyCode::Down => app.editor.canvas.move_select_down(1),
        KeyCode::Left => app.editor.canvas.move_select_left(1),
        KeyCode::Right => app.editor.canvas.move_select_right(1),

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
