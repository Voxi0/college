mod app;
mod components;
mod main_menu;
mod player;

use app::AppPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AppPlugin))
        .init_state::<components::AppState>()
        .run();
}
