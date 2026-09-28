#![allow(non_snake_case)]
mod app;
mod menu;
mod menu_box;
use app::App;

pub const BORDER_COLOR: u32 = 0x2B385E;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let appResult = App::default().run(&mut terminal)?;
    ratatui::restore();
    return Ok(appResult);
}
