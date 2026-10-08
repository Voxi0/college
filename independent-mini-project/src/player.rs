use crate::components::MoveSpeed;
use bevy::prelude::*;

// Marker for our player
#[derive(Component, Default, Clone)]
struct Player;

// Player plugin
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, player.spawn())
            .add_systems(Update, player_movement);
    }
}

// Spawn a player
fn player() -> impl Scene {
    bsn! {
        Player
        Camera2d
        MoveSpeed(200)
    }
}

// Handle player movement
fn player_movement(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Single<(&mut Transform, &MoveSpeed), With<Player>>
) {
    let (mut transform, move_speed) = query.into_inner();

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

    let move_delta = move_speed.0 as f32 * time.delta_secs();
    let movement = dir.normalize_or_zero() * move_delta;
    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
}
