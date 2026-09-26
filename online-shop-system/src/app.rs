use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use color_eyre::eyre::{WrapErr};
use ratatui::{
    layout::{Constraint, Direction, Layout, Flex},
    style::{Style, Color, Modifier},
    text::{Line},
    widgets::{
        BorderType, Block,
        List, ListState,
    },
};

#[derive(Debug, Default)]
pub struct App {
    menuItems: Vec<&'static str>,
    menuState: ListState,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::Result<()> {
        // Main loop
        self.menuItems = vec![
            "balls",
            "spaghetti",
            "chicken",
        ];
        self.menuState = ListState::default();
        self.menuState.select_first();

        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handleEvents().wrap_err("Failed to handle events")?;
        }

        return Ok(());
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let block = Block::bordered()
            .title(Line::from(" Hello ").centered())
            .border_type(BorderType::Rounded)
            .border_style(Style::new().red());
        let innerArea = block.inner(frame.area());
        frame.render_widget(block, frame.area());

        let menu_height = self.menuItems.len() as u16;
        let menu_width = 25; // Fixed width in cells

        let [vertical_area] = Layout::vertical([Constraint::Length(menu_height)])
            .flex(Flex::Center)
            .areas(innerArea);

        // 4. Center horizontally inside vertical slice
        let [menu_area] = Layout::horizontal([Constraint::Length(menu_width)])
            .flex(Flex::Center)
            .areas(vertical_area);

        let menu = List::new(self.menuItems.iter().copied())
            .style(Color::White)
            .highlight_symbol("> ")
            .highlight_style(Style::default()
                .bg(Color::Red)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)
            );
        frame.render_stateful_widget(menu, menu_area, &mut self.menuState);
    }

    fn handleEvents(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(keyEvent) if keyEvent.kind == KeyEventKind::Press => self
                .handleKeyEvent(keyEvent)
                .wrap_err_with(|| format!("Failed to handle key event: \n{keyEvent:#?}")),
            _ => Ok(()),
        }
    }

    fn handleKeyEvent(&mut self, keyEvent: KeyEvent) -> color_eyre::Result<()> {
        match keyEvent.code {
            // Menu navigation
            KeyCode::Up => self.menuState.select_previous(),
            KeyCode::Down => self.menuState.select_next(),
            KeyCode::Char('k') => self.menuState.select_previous(),
            KeyCode::Char('j') => self.menuState.select_next(),
            KeyCode::Enter => self.handleChoice(),

            // Exit the program
            KeyCode::Esc => self.exit(),
            KeyCode::Char('q') => self.exit(),

            // Anything else
            _ => {},
        }

        return Ok(());
    }

    fn handleChoice(&mut self) {
        if let Some(index) = self.menuState.selected() {
            println!("You have chosen {}", self.menuItems[index]);
        } else {
            println!("what, how.");
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
