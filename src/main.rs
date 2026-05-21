mod app;
mod events;
mod pixels;
mod routes;

use crate::app::App;
use std::io;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
