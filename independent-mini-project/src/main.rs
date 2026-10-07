mod app;
mod menu;
use iced::window;
use app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("kewl app")
        .window(window::Settings {
            resizable: true,
            decorations: true,
            ..Default::default()
        })
        .theme(iced::Theme::Oxocarbon)
        .centered()
        .run()
}
