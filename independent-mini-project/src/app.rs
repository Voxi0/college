use bevy::prelude::*;
use bevy::input::common_conditions::*;
use bevy_ecs_tiled::prelude::*;

use crate::components::AppState;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TiledPlugin::default())
            .add_plugins(crate::player::PlayerPlugin)
            .add_systems(Startup, app_startup)
            .add_systems(Update, start_game.run_if(input_just_pressed(KeyCode::Enter)));
    }
}

fn app_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Load and display map
    commands.spawn((
        TiledMap(asset_server.load("map.tmx")),
        TilemapAnchor::Center,
    ));
}

// Remove all cameras and then change the app state
// The only camera to be used will be the one attached to the player
fn start_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<Entity, With<Camera>>,
) {
    for camera in &query {commands.entity(camera).despawn()}
    next_state.set(AppState::InGame);
}
