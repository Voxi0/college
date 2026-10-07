mod app;
mod menu;
use app::App;

use iced::{Pixels, Settings, Theme, application, window};

fn main() -> iced::Result {
    application(App::new, App::update, App::view)
        .title("kewl app")
        .theme(Theme::Oxocarbon)
        .centered()
        .settings(Settings {
            antialiasing: false,
            default_text_size: Pixels(24.0),
            ..Default::default()
        })
        .window(window::Settings {
            resizable: true,
            decorations: true,
            ..Default::default()
        })
        .run()
}
