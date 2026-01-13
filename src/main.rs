#![warn(
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)] // Enable linter in pedantic mode.
mod editor;

use editor::Editor;

fn main() {
    /* Add a panic hook to handle terminal reset */
    let current_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        Editor::terminate(); // Terminate the terminal.
        current_hook(panic_info);
    }));
    Editor::default().run();
}


