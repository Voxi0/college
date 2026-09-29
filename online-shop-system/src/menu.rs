use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{List, ListDirection, ListState},
};

#[derive(Default)]
pub struct Menu {
    items: Vec<String>,
    chosenItem: String,
    state: ListState,
    highlightStyle: Style,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            highlightStyle: Style::default(),
            ..Default::default()
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let widget = List::new(self.items.iter().map(|s| s.as_str()))
            .direction(ListDirection::TopToBottom)
            .highlight_symbol("> ")
            .highlight_style(self.highlightStyle);
        frame.render_stateful_widget(widget, area, &mut self.state);
    }

    pub fn handleKeyEvents(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up | KeyCode::Char('k') => self.state.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => self.state.select_next(),
            KeyCode::Enter => {
                if let Some(index) = self.state.selected() {
                    self.chosenItem = self.items[index].to_string();
                }
            },
            _ => {},
        }
    }

    // Setters
    pub fn items<T: Into<String>>(mut self, items: impl IntoIterator<Item = T>) -> Self {
        let itemList: Vec<String> = items.into_iter().map(Into::into).collect();
        if !itemList.is_empty() {
            self.items = itemList;
            self.state.select_first();
        }
        return self;
    }
    pub fn highlightStyle(mut self, style: Style) -> Self {
        self.highlightStyle = style;
        return self;
    }

    // Getters
    pub fn getChosenItem(&self) -> &str {
        return self.chosenItem.as_str();
    }
}
