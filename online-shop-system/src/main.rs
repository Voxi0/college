#![allow(non_snake_case)]
mod app;
use app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let appResult = App::default().run(&mut terminal)?;
    ratatui::restore();
    return Ok(appResult);
}
