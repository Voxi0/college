use crate::menu::Menu;
use crate::menu_box::MenuBox;

use color_eyre::eyre::{WrapErr};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use ratatui::{
    style::{Style, Color},
};

// Application state
#[derive(Debug, Default)]
pub struct App {
    menu: Menu,
    menuBox: MenuBox,
    exit: bool,
}

impl App {
    // Initialize and run the application
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::Result<()> {
        // The widgets
        self.menu = Menu::new(vec!["Keyboard - £25", "Mouse - £15", "Headset - £40"]);
        self.menuBox = MenuBox::new(
            " CHOOSE NOW ".to_string(), self.menu.width.max(32), self.menu.height + 2,
            Style::default().fg(Color::Red),
            Style::default().fg(Color::Yellow),
        );

        // Main loop
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handleEvents().wrap_err("Failed to handle an event")?;
        }

        // End of application
        return Ok(());
    }

    // Draw everything
    fn render(&mut self, frame: &mut ratatui::Frame) {
        // Render the box for the menu list
        self.menuBox.render(frame);

        // Ensure menu area has been calculated before rendering the menu
        // The menu area is just the area inside of the menu box
        // The menu box must be rendered first for this area to be calculated
        if let Some(menuArea) = self.menuBox.getMenuArea() {
            self.menu.render(frame, menuArea);
        }
    }

    // Handle user input/events
    fn handleEvents(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(keyEvent) if keyEvent.kind == KeyEventKind::Press => self
                .handleKeyEvent(keyEvent)
                .wrap_err_with(|| format!("Failed to handle a key event: \n{keyEvent:#?}")),
            _ => Ok(()),
        }
    }

    // Handle key presses
    fn handleKeyEvent(&mut self, keyEvent: KeyEvent) -> color_eyre::Result<()> {
        self.menu.handleKeyEvent(keyEvent.code).wrap_err("Failed to handle menu key event: \n{keyEvent:#?}")?;
        match keyEvent.code {
            // Exit the program
            KeyCode::Esc => self.exit(),
            KeyCode::Char('q') => self.exit(),
            _ => {},
        }
        return Ok(());
    }

    // Terminate the application
    fn exit(&mut self) {
        self.exit = true;
    }
}
