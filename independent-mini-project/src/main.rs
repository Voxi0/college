mod app;
mod components;
mod main_menu;
mod player;

use app::AppPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Superb Cool Game".to_string(),
                    ..Default::default()
                }),
                ..default()
            })
        )
        .init_state::<components::AppState>()
        .add_plugins(AppPlugin)
        .run();
}
