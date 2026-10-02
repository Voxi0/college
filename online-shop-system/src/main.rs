#![allow(non_snake_case)]

mod utils;
mod app_state;
mod app;
mod container_box;
mod menu;
use app::App;

// Constants
pub const BORDER_COLOR: u32 = 0x2B385E;
pub const TEXT_COLOR: u32 = 0xffaa00;
pub const HIGHLIGHT_BG_COLOR: u32 = 0x532326;
pub const HIGHLIGHT_FG_COLOR: u32 = 0xffffff;

fn main() -> color_eyre::Result<()> {
    // Install `color-eyre` panic/error handlers and whatnot
    color_eyre::install()?;

    // Create the application
    let mut app: App = App::new()
        .title("Hello World");

    // Initialize Ratatui and run the application
    let mut terminal = ratatui::init();
    let appResult = app.run(&mut terminal);

    // Restore terminal state and terminate program
    ratatui::restore();
    return appResult;
}
