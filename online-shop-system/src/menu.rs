use ratatui::{
    Frame,
    style::{Style, Color},
    layout::Rect,
    widgets::{List, ListState},
};

pub struct Menu {
    items: Option<Vec<String>>,
    state: ListState,
}

impl Menu {
    pub fn new() -> Self {
        return Self {
            items: None,
            state: ListState::default(),
        };
    }

    pub fn render(&mut self, frame: &mut Frame, area: &Rect) {
        if let Some(items) = self.items.as_deref() {
            let widget = List::new(items.iter().map(|s| s.as_str()))
                .style(Style::default());
            frame.render_stateful_widget(widget, frame.area(), &mut self.state);
        }
    }
}
