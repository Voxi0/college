use crate::{
    app_state::AppState,
    box::Box,
}

use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
};

pub struct App {
    state: AppState,
    title: Option<String>,
    productSelectionMenu: Option<Menu>,
}

impl App {
    // Create a new application with defaults
    pub fn new() -> App {
        return App {
            state: AppState::ProductSelection,
            title: None,
        };
    }

    // Initialization and loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        // Main loop
        while self.state != AppState::Quit {
            terminal.draw(|frame| self.render(frame))?;
            self.handleEvents()?;
        }

        return Ok(());
    }

    // Rendering
    fn render(&self, frame: &mut Frame) {
        frame.render_widget("balls", frame.area());
    }

    // Event handling
    fn handleEvents(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match self.state {
                    AppState::ProductSelection => self.productSelectionMenu.handleEvents(),
                    _ => {},
                }

                self.handleKeyEvent(key.code);
            },
            _ => {},
        }
        return Ok(());
    }
    fn handleKeyEvent(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => self.state = AppState::Quit,
            _ => {},
        };
    }

    // Setters
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        return self;
    }
}
