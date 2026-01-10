#![warn(clippy::all,clippy::pedantic,clippy::print_stdout,clippy::arithmetic_side_effects,clippy::as_conversion,clippy::integer_division)] // Enable linter in pedantic mode.
mod editor;

use editor::Editor;
//use std::io::{self, Read};
//use crossterm::terminal::enable_raw_mode;
//use crossterm::terminal::disable_raw_mode;

fn main() {
    Editor::default().run();
}
