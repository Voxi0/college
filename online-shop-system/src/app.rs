use crate::{
    BORDER_COLOR, HIGHLIGHT_BG_COLOR, HIGHLIGHT_FG_COLOR,
    utils,
    app_state::AppState,
    container_box::ContainerBox, menu::Menu,
};

use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

#[derive(Default)]
pub struct App {
    state: AppState,
    title: Option<String>,
    containerBox: ContainerBox,
    productSelectionMenu: Menu,
    deliveryMethodSelectionMenu: Menu,
}

impl App {
    // Create a new application with defaults
    pub fn new() -> App {
        let mut app = App {
            state: AppState::ProductSelection,
            title: None,
            ..Default::default()
        };

        // Create the container box
        app.containerBox = ContainerBox::new()
            .title(app.state.title())
            .size(30, 5)
            .borderStyle(
                Style::default()
                    .fg(Color::from_u32(BORDER_COLOR))
            );

        app.productSelectionMenu = Menu::new()
            .items(["Keyboard - £25", "Mouse - £15", "Headset - £40"])
            .highlightStyle(
                Style::default()
                    .bg(Color::from_u32(HIGHLIGHT_BG_COLOR))
                    .fg(Color::from_u32(HIGHLIGHT_FG_COLOR))
                    .add_modifier(Modifier::BOLD)
            );
        app.deliveryMethodSelectionMenu = Menu::new()
            .items(["Standard Delivery", "Premium Delivery"])
            .highlightStyle(
                Style::default()
                    .bg(Color::from_u32(HIGHLIGHT_BG_COLOR))
                    .fg(Color::from_u32(HIGHLIGHT_FG_COLOR))
                    .add_modifier(Modifier::BOLD)
            );

        return app;
    }

    // Main loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        // Main loop
        while self.state != AppState::Quit {
            terminal.draw(|frame| self.render(frame).expect("Fuck"))?;
            self.handleEvents()?;
        }

        return Ok(());
    }

    // Rendering
    fn render(&mut self, frame: &mut Frame) -> color_eyre::Result<()> {
        match self.state {
            AppState::ProductSelection => {
                // Resize container box to fit the product selection menu before rendering the menu
                let (menuWidth, menuHeight) = self.productSelectionMenu.getSize()?;
                self.containerBox.setSize((menuWidth as u16) + 10, (menuHeight as u16) + 2);
                let containerArea = utils::getCenterArea(frame, self.containerBox.getSize().0, self.containerBox.getSize().1);
                self.productSelectionMenu.render(frame, self.containerBox.getInnerArea(containerArea));

                // Render container box
                let containerArea = utils::getCenterArea(frame, self.containerBox.getSize().0, self.containerBox.getSize().1);
                self.containerBox.render(frame, containerArea);
            }

            AppState::QuantitySelection => {
                // Resize container box to fit the product selection menu before rendering the menu
                let paragraph = Paragraph::new();

                // Render container box
                let containerArea = utils::getCenterArea(frame, self.containerBox.getSize().0, self.containerBox.getSize().1);
                self.containerBox.render(frame, containerArea);
            },

            AppState::DeliveryMethodSelection => {
                // Resize container box to fit the product selection menu before rendering the menu
                let (menuWidth, menuHeight) = self.deliveryMethodSelectionMenu.getSize()?;
                self.containerBox.setSize((menuWidth as u16) + 10, (menuHeight as u16) + 2);
                let containerArea = utils::getCenterArea(frame, self.containerBox.getSize().0, self.containerBox.getSize().1);
                self.deliveryMethodSelectionMenu.render(frame, self.containerBox.getInnerArea(containerArea));

                // Render container box
                let containerArea = utils::getCenterArea(frame, self.containerBox.getSize().0, self.containerBox.getSize().1);
                self.containerBox.render(frame, containerArea);
            },

            _ => {},
        }

        return Ok(());
    }

    // Event handling
    fn handleEvents(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match self.state {
                    AppState::ProductSelection => self.productSelectionMenu.handleKeyEvents(key.code),
                    AppState::DeliveryMethodSelection => self.deliveryMethodSelectionMenu.handleKeyEvents(key.code),
                    AppState::QuantitySelection => {},
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
            KeyCode::Enter => self.state.next(),
            KeyCode::Backspace => self.state.prev(),
            _ => {},
        };
    }

    // Setters
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        return self;
    }
}
