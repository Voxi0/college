use crossterm::event::{KeyCode};
use ratatui::{
    Frame,
    layout::{Rect},
    widgets::{List, ListState},
    style::{Style, Color, Modifier},
    text::Line,
};

// The menu list
#[derive(Debug, Default)]
pub struct Menu {
    items: Vec<&'static str>,
    state: ListState,
    pub width: u16,
    pub height: u16,
}

impl Menu {
    // Create a new menu list
    pub fn new(items: Vec<&'static str>) -> Self {
        let mut menu = Self {
            items: items,
            state: ListState::default(),
            width: 0,
            height: 0,
        };

        // Set default menu state and height of the menu
        menu.state.select_first();
        menu.width = menu.items
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(0) as u16;
        menu.height = menu.items.len() as u16;

        return menu;
    }

    // Handle menu navigation
    pub fn handleKeyEvent(&mut self, keyEventCode: KeyCode) -> color_eyre::Result<()> {
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

    // What to do when the user picks something from the menu list
    fn handleChoice(&self) {
        if let Some(index) = self.state.selected() {
            println!("You got {}", self.items[index]);
        }
    }

    // Render the menu list
    pub fn render(&mut self, frame: &mut Frame, menuArea: Rect) {
        // Seperate item and price
        let parsed: Vec<(&str, &str)> = self.items
            .iter()
            .map(|s| s.split_once(" - ").unwrap_or((s, "")))
            .collect();

        // Figure out the maximum string length of both items and prices
        let maxItemLen = parsed.iter().map(|(item, _)| item.len()).max().unwrap_or(0);
        let maxPriceLen = parsed.iter().map(|(_, price)| price.len()).max().unwrap_or(0);

        // List of items
        let items = parsed
            .iter()
            .map(|(item, price)| {
                let formatted = format!(
                    "{:<leftWidth$} \t - \t {:>rightWidth$}",
                    item, price,
                    leftWidth = maxItemLen,
                    rightWidth = maxPriceLen,
                );
                return Line::from(formatted).centered();
            });

        let menu: List = List::new(items)
            .style(Color::White)
            .highlight_style(Style::default()
                .bg(Color::from_u32(0x532326))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
            );
        frame.render_stateful_widget(menu, menuArea, &mut self.state);
    }
}
