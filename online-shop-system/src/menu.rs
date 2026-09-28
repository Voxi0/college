use crossterm::event::{KeyCode};
use ratatui::{
    Frame,
    layout::Rect,
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
    pub chosenItem: Option<&'static str>,
}

impl Menu {
    // Create a new menu list
    pub fn new(items: Vec<&'static str>) -> Self {
        // Create the menu list
        let mut menu = Self {
            items,
            chosenItem: None,
            state: ListState::default(),
            width: 0,
            height: 0,
        };

        // Set default menu state and size of the menu
        menu.state.select_first();
        menu.height = menu.items.len() as u16;
        menu.width = menu.items
            .iter()
            .map(|item| item.len())
            .max()
            .unwrap_or(0) as u16;

        // Return the menu list
        return menu;
    }

    // Handle menu navigation
    pub fn handleKeyEvent(&mut self, keyEventCode: KeyCode) -> color_eyre::Result<()> {
        // Menu navigation if no item has been chosen yet
        match keyEventCode {
            KeyCode::Up | KeyCode::Char('k') => self.state.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => self.state.select_next(),
            KeyCode::Enter => self.handleChoice(),
            _ => {},
        }

        return Ok(());
    }

    // What to do when the user picks something from the menu list
    fn handleChoice(&mut self) {
        if let Some(index) = self.state.selected() {
            self.chosenItem = Some(self.items[index]);
        }
    }

    // Render the menu list
    pub fn render(&mut self, frame: &mut Frame, menuArea: Rect) {
        let menu: List = List::new(self.items.clone())
            .style(Color::White)
            .highlight_style(Style::default()
                .bg(Color::from_u32(0x532326))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
            );

        frame.render_stateful_widget(menu, menuArea, &mut self.state);
    }
}
