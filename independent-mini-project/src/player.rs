use crate::components::{AppState, MoveSpeed};
use bevy::prelude::*;

// Marker for our player
#[derive(Component, FromTemplate)]
#[require(
    MoveSpeed(200),
    Transform,
    Visibility
)]
struct Player;

// Player plugin
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), player.spawn())
            .add_systems(Update, player_movement.run_if(in_state(AppState::InGame)));
    }
}

// Spawn a player
fn player() -> impl Scene {
    bsn! {
        Player
        Camera2d
        Mesh2d(asset_value(Circle::new(40.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 0.0, 0.0)))
    }
}

// Handle player movement
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Single<(&mut Transform, &MoveSpeed), With<Player>>
) {
    // Get our data from the query
    let (mut transform, move_speed) = query.into_inner();

    // Figure out movement direction
    let mut dir: Vec2 = Vec2::ZERO;
    for key in input.get_pressed() {
        dir += match key {
            KeyCode::ArrowRight | KeyCode::KeyD => Vec2::X,
            KeyCode::ArrowLeft | KeyCode::KeyA => Vec2::NEG_X,
            KeyCode::ArrowUp | KeyCode::KeyW => Vec2::Y,
            KeyCode::ArrowDown | KeyCode::KeyS => Vec2::NEG_Y,
            _ => Vec2::ZERO,
        };
    }

    // Calculate movement speed by multiplying with delta-time
    let move_delta = move_speed.0 as f32 * time.delta_secs();

    // Update player position
    // We use `extend` to turn `movement` into a `Vec3` since `translation` is a `Vec3`
    let movement = dir.normalize_or_zero() * move_delta;
    transform.translation += movement.extend(0.);
}
