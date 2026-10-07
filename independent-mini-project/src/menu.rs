use iced::{
    Element,
    widget::{column, button},
};

#[derive(Default)]
pub struct Menu {
    visible: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleVisible
}

impl Menu {
    pub fn new() -> Self {
        Self {
            visible: true,
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleVisible => self.visible = !self.visible,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column!(
            if self.visible {
                button("Hide Menu").on_press(Message::ToggleVisible)
            } else {
                button("Show Menu").on_press(Message::ToggleVisible)
            }
        ).into()
    }
}
