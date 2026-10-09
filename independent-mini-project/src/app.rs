use bevy::prelude::*;
// use bevy_ecs_tiled::prelude::*;

use crate::{
    main_menu::MainMenuPlugin, player::PlayerPlugin,
};

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(TiledPlugin::default())
            .add_plugins((MainMenuPlugin, PlayerPlugin))
            .add_systems(Startup, app_startup);
    }
}

fn app_startup(_commands: Commands, _asset_server: Res<AssetServer>) {
    // Load and display map
    // commands.spawn((
    //     TiledMap(asset_server.load("map.tmx")),
    //     TilemapAnchor::Center,
    // ));
}
