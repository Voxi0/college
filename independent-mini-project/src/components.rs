use bevy::{prelude::*, state::state::States, ecs::template::FromTemplate, ui_widgets::Button};

// Application state
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
  #[default]
  MainMenu,
  InGame,
  Paused,
}

// Movement speed
#[derive(Component, FromTemplate)]
pub struct MoveSpeed(pub u8);

// A button scene
pub fn button(
    text: String, 
    size: Vec2,
    bg_color: Color,
    fg_color: Color,
) -> impl Scene {
    bsn! {
        Button
        BackgroundColor(bg_color)
        BorderColor::all(Color::BLACK)
        Node {
            width: px(size.x),
            height: px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(4.)),
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
