use crate::{
    BORDER_COLOR,
    utils,
    app_state::AppState,
    container_box::ContainerBox,
    menu::Menu,
};

use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    style::{Style, Color},
};

pub struct App {
    state: AppState,
    title: Option<String>,
    containerBox: Option<ContainerBox>,
}

impl App {
    // Create a new application with defaults
    pub fn new() -> App {
        return App {
            state: AppState::ProductSelection,
            title: None,
            containerBox: None,
        };
    }

    // Initialization and loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        // Create the container box
        self.containerBox = Some(
            ContainerBox::new()
                .title(" Hello World ")
                .size(40, 20)
                .borderStyle(
                    Style::default()
                        .fg(Color::from_u32(BORDER_COLOR))
                )
        );

        // Main loop
        while self.state != AppState::Quit {
            terminal.draw(|frame| self.render(frame))?;
            self.handleEvents()?;
        }

        return Ok(());
    }

    // Rendering
    fn render(&self, frame: &mut Frame) {
        if let Some(container) = &self.containerBox {
            let (containerWidth, containerHeight) = container.getSize();
            container.render(frame, utils::getCenterArea(frame, containerWidth, containerHeight));
        }
    }

    // Event handling
    fn handleEvents(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match self.state {
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
