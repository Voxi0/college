#![allow(non_snake_case)]

mod utils;
mod app_state;
mod app;
mod container_box;
mod menu;
use app::App;

// Constants
pub const BORDER_COLOR: u32 = 0x22ffaa;
pub const TEXT_COLOR: u32 = 0xffaa00;

fn main() -> color_eyre::Result<()> {
    // Install `color-eyre` panic/error handlers and whatnot
    color_eyre::install()?;

    // Create the application
    let mut app: App = App::new()
        .title("Hello World");

    let mut terminal = ratatui::init();
    let appResult = app.run(&mut terminal);
    ratatui::restore();
    return appResult;
}
