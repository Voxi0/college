use crate::menu::Menu;

use color_eyre::eyre::{WrapErr};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Flex},
    style::{Style, Color},
    text::{Line},
    widgets::{BorderType, Block},
};

// Application state
#[derive(Debug, Default)]
pub struct App {
    pub menu: Menu,
    exit: bool,
}

impl App {
    // Initialize and run the application
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::Result<()> {
        self.menu = Menu::new(vec!["List", "of", "balls"]);

        // Main loop
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handleEvents().wrap_err("Failed to handle events")?;
        }

        // End of application
        return Ok(());
    }

    // Draw everything
    fn render(&mut self, frame: &mut ratatui::Frame) {
        // Menu block/box
        let menuBlockWidth = self.menu.width.max(64);
        let menuBlockHeight = self.menu.height + 2;
        let [menuBlockVerticalArea] = Layout::vertical([Constraint::Length(menuBlockHeight)])
            .flex(Flex::Center)
            .areas(frame.area());
        let [menuBlockArea] = Layout::horizontal([Constraint::Length(menuBlockWidth)])
            .flex(Flex::Center)
            .areas(menuBlockVerticalArea);
        let menuBlock = Block::bordered()
            .title(Line::from(" CHOOSE MORTAL ").centered())
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));
        let menuArea = menuBlock.inner(menuBlockArea);
        frame.render_widget(menuBlock, menuBlockArea);

        // Menu list
        self.menu.render(frame, menuArea);
    }

    // Handle user input/events
    fn handleEvents(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(keyEvent) if keyEvent.kind == KeyEventKind::Press => self
                .handleKeyEvent(keyEvent)
                .wrap_err_with(|| format!("Failed to handle key event: \n{keyEvent:#?}")),
            _ => Ok(()),
        }
    }

    // Handle key presses
    fn handleKeyEvent(&mut self, keyEvent: KeyEvent) -> color_eyre::Result<()> {
        self.menu.handleKeyEvent(keyEvent.code)?;
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
