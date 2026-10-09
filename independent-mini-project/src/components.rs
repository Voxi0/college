use bevy::{ecs::template::FromTemplate, prelude::Component, state::state::States};

// Application state
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
  #[default]
  MainMenu,
  InGame,
  Paused,
  Exit,
}

// Movement speed
#[derive(Component, FromTemplate)]
pub struct MoveSpeed(pub u8);
