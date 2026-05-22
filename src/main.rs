mod app;
mod events;
mod pixels;
mod routes;

use crate::app::App;
use log::{LevelFilter, info};
use simplelog::{Config, WriteLogger};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    match WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("logs/main.log")?,
    ) {
        Ok(()) => (),
        Err(e) => panic!("ERROR: Failed to create logger: {e}."),
    }
    ratatui::run(|terminal| App::default().run(terminal))?;
    info!("Started tui app");
    Ok(())
}
