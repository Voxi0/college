use crate::TEXT_COLOR;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Padding},
};

#[derive(Default)]
pub struct ContainerBox {
    title: String,
    rect: Rect,
    borderStyle: Style,
}

impl ContainerBox {
    pub fn new() -> Self {
        return Self {
            rect: Rect {x: 0, y: 0, width: 30, height: 30},
            ..Default::default()
        };
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self.getWidget(), area);
    }

    // Getters
    fn getWidget(&self) -> Block<'_> {
        Block::bordered()
            .title(Line::from(self.title.to_string())
                .centered()
                .style(Style::default().fg(Color::from_u32(TEXT_COLOR))))
            .border_style(self.borderStyle)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(
                // Left and top
                1, 1,

                // Right and bottom
                1, 0
            ))
    }
    pub fn getInnerArea(&self, area: Rect) -> Rect {
        return self.getWidget().inner(area);
    }
    pub fn getRect(&self) -> Rect {
        self.rect
    }

    // Setters
    pub fn setTitle(&mut self, title: &str) {
        self.title = title.to_string();
    }

    // Builder-lite
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        return self;
    }
    pub fn width(mut self, width: u16) -> Self {
        self.rect.width = width;
        return self;
    }
    pub fn height(mut self, height: u16) -> Self {
        self.rect.height = height;
        return self;
    }
    pub fn borderStyle(mut self, style: Style) -> Self {
        self.borderStyle = style;
        return self;
    }
}
