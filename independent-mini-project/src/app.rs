use crate::menu::{self, Menu};
use iced::{
    window,
    Task,
    widget::{container, row, column, text, button},
};

#[derive(Default)]
pub struct App {
    menu: Menu,
}

#[derive(Debug, Clone)]
pub enum Message {
    Exit,
    MenuMessage(menu::Message),
}

impl App {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MenuMessage(msg) => self.menu.update(msg),
            Message::Exit => return window::latest().and_then(window::close),
        }

        // For messages that only perform state mutations with no side effects
        // Else we do early returns
        return Task::none();
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        container(
            column![
                text("hello"),
                row![
                    button("Exit").on_press(Message::Exit),
                    self.menu.view().map(Message::MenuMessage),
                ].spacing(10)
            ]
        )
        .into()
    }
}
