#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;

use simplelog::*;
use std::fs::File;
use editor::Editor;

fn main() {

    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("debug.log").unwrap())
        ]
    ).unwrap();

    log::debug!("this is a test entry");
    Editor::new().unwrap().run();
}


