use crate::TEXT_COLOR;

use ratatui::{
    Frame,
    style::{Style, Color},
    layout::Rect,
    widgets::{Block, BorderType},
    text::{Line},
};

pub struct ContainerBox {
    title: Option<String>,
    width: u16,
    height: u16,
    borderStyle: Option<Style>,
}

impl ContainerBox {
    pub fn new() -> Self {
        return Self {
            title: None,
            width: 30,
            height: 30,
            borderStyle: None,
        };
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let widget = Block::bordered()
            .title(Line::from(self.title.as_deref().unwrap_or(" default title "))
                .centered()
                .style(Style::default().fg(Color::from_u32(TEXT_COLOR))))
            .border_style(self.borderStyle.unwrap_or(Style::default()))
            .border_type(BorderType::Rounded);
        frame.render_widget(widget, area);
    }

    // Setters
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        return self;
    }
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        return self;
    }
    pub fn borderStyle(mut self, style: Style) -> Self {
        self.borderStyle = Some(style);
        return self;
    }

    // Getters
    pub fn getSize(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
