use crossterm::event::{KeyCode};
use ratatui::{
    Frame,
    layout::{Rect},
    widgets::{List, ListState},
    style::{Style, Color, Modifier},
    text::Line,
};

#[derive(Debug, Default)]
pub struct Menu {
    items: Vec<&'static str>,
    state: ListState,
    pub width: u16,
    pub height: u16,
}

impl Menu {
    pub fn new(items: Vec<&'static str>) -> Self {
        let mut menu = Self {
            items: items,
            state: ListState::default(),
            width: 0,
            height: 0,
        };

        menu.state.select_first();
        menu.height = menu.items.len() as u16;
        menu.width = menu.items
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(0) as u16;

        return menu;
    }

    pub fn handleKeyEvent(&mut self, keyEventCode: KeyCode) -> color_eyre::Result<()> {
        // Menu navigation
        match keyEventCode {
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            KeyCode::Char('k') => self.state.select_previous(),
            KeyCode::Char('j') => self.state.select_next(),
            KeyCode::Enter => self.handleChoice(),
            _ => {},
        }

        return Ok(());
    }

    fn handleChoice(&self) {
        if let Some(index) = self.state.selected() {
            println!("You got {}", self.items[index]);
        }
    }

    pub fn render(&mut self, frame: &mut Frame, menuArea: Rect) {
        let menu: List = List::new(self.items.iter().map(|&item| Line::from(item).centered()))
            .style(Color::White)
            .highlight_style(Style::default()
                .bg(Color::Red)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC)
            );
        frame.render_stateful_widget(menu, menuArea, &mut self.state);
    }
}
