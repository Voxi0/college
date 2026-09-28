use ratatui::{
    Frame,
    widgets::{BorderType, Block},
    layout::{Rect, Constraint, Layout, Flex},
    text::{Line},
    style::{Style, Styled},
};

// Container/Box for the menu list
#[derive(Debug, Default)]
pub struct MenuBox {
    title: String,
    width: u16,
    height: u16,
    boxStyle: Style,
    titleStyle: Style,
}

impl MenuBox {
    // Create a new container/box for the menu list
    pub fn new(title: String, width: u16, height: u16, boxStyle: Style, titleStyle: Style) -> Self {
        return Self {
            title,
            width,
            height,
            boxStyle,
            titleStyle,
        };
    }

    // Render the container/box
    pub fn render(&mut self, frame: &mut Frame) {
        let (block, _innerArea) = self.getBlockAndInnerArea(frame.area());
        frame.render_widget(block, self.getOuterArea(frame.area()));
    }

    // Getters
    pub fn getOuterArea(&self, area: Rect) -> Rect {
        let [verticalArea] = Layout::vertical([Constraint::Length(self.height)])
            .flex(Flex::Center)
            .areas(area);

        let [blockArea] = Layout::horizontal([Constraint::Length(self.width)])
            .flex(Flex::Center)
            .areas(verticalArea);

        return blockArea;
    }
    pub fn getBlockAndInnerArea(&self, area: Rect) -> (Block, Rect) {
        let block =  Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(self.boxStyle)
            .title(
                Line::from(self.title.as_str())
                    .centered()
                    .set_style(self.titleStyle)
            );

        return (block.clone(), block.inner(self.getOuterArea(area)));
    }

    // Setters
    pub fn setTitle(&mut self, title: &str) {
        self.title = title.to_string();
    }
    pub fn setSize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}
