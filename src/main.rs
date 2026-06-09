mod app;
mod events;
mod pixels;
mod routes;

use crate::app::App;
use clap::Parser;
use crossterm::execute;
use log::{LevelFilter, info};
use simplelog::{Config, WriteLogger};
use std::path::Path;
use std::{fs::File, io};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> io::Result<()> {
    match WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("logs/main.log")?,
    ) {
        Ok(()) => (),
        Err(e) => panic!("ERROR: Failed to create logger: {e}."),
    }

    let args = Args::parse();

    ratatui::run(|terminal| {
        execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;
        let result = match args.file {
            Some(ref path) => App::start_with_file(Path::new(path)).run(terminal),
            None => App::default().run(terminal),
        };
        execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
        result
    })?;

    info!("Started tui app");
    Ok(())
}
