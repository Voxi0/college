use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TiledPlugin::default())
            .add_plugins(crate::player::PlayerPlugin)
            .add_systems(Startup, app_startup)
            .add_systems(Update, app_update);
    }
}

fn app_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load and display map
    let map_handle: Handle<TiledMapAsset> = asset_server.load("map.tmx");
    commands.spawn((
        TiledMap(map_handle),
        TilemapAnchor::Center,
    ));
}

fn app_update(mut _commands: Commands, _asset_server: Res<AssetServer>) {}
