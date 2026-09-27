use ratatui::{
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
    menuArea: Option<Rect>,
    boxStyle: Style,
    titleStyle: Style,
}

impl MenuBox {
    // Create a new container/box for the menu list
    pub fn new(title: String, width: u16, height: u16, boxStyle: Style, titleStyle: Style) -> Self {
        return Self {
            title: title,
            width: width,
            height: height,
            boxStyle: boxStyle,
            titleStyle: titleStyle,
            menuArea: None,
        };
    }

    // Render the container/box for the menu list
    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        // Menu block/box
        let [menuBlockVerticalArea] = Layout::vertical([Constraint::Length(self.height)])
            .flex(Flex::Center)
            .areas(frame.area());

        let [menuBlockArea] = Layout::horizontal([Constraint::Length(self.width)])
            .flex(Flex::Center)
            .areas(menuBlockVerticalArea);

        let menuBlock = Block::bordered()
            .title(
                Line::from(self.title.as_str())
                    .centered()
                    .set_style(self.titleStyle)
                )
            .border_type(BorderType::Rounded)
            .border_style(self.boxStyle);

        self.menuArea = Some(menuBlock.inner(menuBlockArea));
        frame.render_widget(menuBlock, menuBlockArea);
    }

    // Getters
    pub fn getMenuArea(&self) -> Option<Rect> {self.menuArea}
}
