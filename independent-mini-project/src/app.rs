use iced::{
    window,
    Task,
    Alignment::Center, Length,
    Font, font::Weight,
    widget::{container, column, row, button, text},
};

#[derive(Default)]
pub struct App {
    label: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Touched,
    Hit,
    Exit,
}

impl App {
    pub fn new() -> Self {
        Self {
            label: "Hey there!".to_string(),
            ..Default::default()
        }
    }

    // Handle updates
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Touched => self.label = "ay?".to_string(),
            Message::Hit => self.label = "you madafaka.".to_string(),
            Message::Exit => return window::latest().and_then(window::close),
        }

        // For messages that only perform state mutations with no side effects
        // Else we do early returns
        return Task::none();
    }

    // Render the application
    pub fn view(&self) -> iced::Element<'_, Message> {
        container(
            column![
                text(&self.label).size(40).center(),
                row![
                    button(text("touch me").center()).on_press(Message::Touched),
                    button(text("hit me").center()).on_press(Message::Hit),
                    button(text("exit").center().font(Font {weight: Weight::Bold, ..Default::default()})).on_press(Message::Exit),
                ]
                .spacing(24),
            ]
            .align_x(Center)
            .spacing(20)
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Center)
            .align_y(Center)
            .into()
    }
}
