use crate::components::{AppState, button};
use bevy::{prelude::*, app::AppExit};

#[derive(Component, FromTemplate)]
#[require(Camera2d)]
struct MainMenu;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu_scene.spawn());
    }
}

fn main_menu_scene() -> impl SceneList {
    bsn_list! {
        DespawnOnExit<AppState>(AppState::MainMenu)
        MainMenu
        @main_menu()
    }
}

fn main_menu() -> impl Scene {
    bsn! {
        BackgroundColor(Color::srgb_u32(0x1A1E2B))
        Node {
            width: px(500),
            height: px(400),
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
        }
        Children [
            @button("Play".to_string(), Vec2::new(150.0, 50.0), Color::srgb_u32(0x0D4378), Color::BLACK)
            on(|_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
                next_state.set(AppState::InGame);
            })

            --

            @button("Exit".to_string(), Vec2::new(150.0, 50.0), Color::WHITE, Color::BLACK)
            on(|_: On<PointerClick>, mut msg_writer: MessageWriter<AppExit>| {
                msg_writer.write(AppExit::Success);
            })
        ]
    }
}
