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
    Ok(())
}

fn handle_home(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Up => {}
        _ => {}
    }
    Ok(())
}
