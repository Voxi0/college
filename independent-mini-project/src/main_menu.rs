use crate::components::AppState;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

#[derive(Component, FromTemplate)]
#[require(Camera2d)]
struct MainMenu;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), main_menu_scene.spawn());
    }
}

fn main_menu_scene() -> impl SceneList {
    bsn_list! {
        DespawnOnExit<AppState>(AppState::MainMenu)
        MainMenu
        @main_menu()
    }
}

fn button(
    text: String, 
    size: Vec2,
    bg_color: Color,
    fg_color: Color,
) -> impl Scene {
    bsn! {
        Button
        BackgroundColor(bg_color)
        Node {
            width: px(size.x),
            height: px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius {
                top_left: CornerRadius::circular(px(10.)),
                top_right: CornerRadius::circular(px(10.)),
                bottom_right: CornerRadius::circular(px(10.)),
                bottom_left: CornerRadius::circular(px(10.)),
            },
        }
        Children [
            Text::new(text)
            TextColor(fg_color)
        ]
    }
}

fn main_menu() -> impl Scene {
    bsn! {
        BackgroundColor(Color::BLACK)
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
            @button("Play".to_string(), Vec2::new(150.0, 50.0), Color::WHITE, Color::BLACK)
            on(|_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| next_state.set(AppState::InGame))
            --
            @button("Exit".to_string(), Vec2::new(150.0, 50.0), Color::WHITE, Color::BLACK)
            on(|_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| next_state.set(AppState::Exit))
        ]
    }
}
