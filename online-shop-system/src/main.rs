#![allow(non_snake_case)]

mod app_state;
mod app;
mod box;
use app::App;

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
