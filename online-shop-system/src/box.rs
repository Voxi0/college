use ratatui::{
    Frame,
    style::{Style, Color},
    layout::{Layout, Constraint, Flex, Rect},
    widgets::{Block, BorderType},
};

pub struct Box {
    title: Option<String>,
    width: f32,
    height: u32,
}

impl Box {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let [verticalLayout] = Layout::new()
            .areas(frame.area());
        let [boxArea] = Layout::new()
            .areas(verticalLayout);

        let box = Block::bordered()
            .title(self.title.unwrap_or("default title".to_string()));

        frame.render_widget(box, area);
    }
}
