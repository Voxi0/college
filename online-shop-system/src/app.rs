use crate::BORDER_COLOR;
use crate::menu::Menu;
use crate::menu_box::MenuBox;

use color_eyre::eyre::{WrapErr};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use ratatui::{
    style::{Style, Color},
    layout::{Layout, Constraint, Flex},
    widgets::{Block, BorderType, Paragraph},
};

#[derive(Debug, Default, PartialEq)]
enum CurrentScreen {
    // Default screen to show where the user picks the item to get
    #[default]
    Menu,

    // Item chosen, now user must pick quantity
    QuantityInput,

    // Let the user decide if they want standard or next-day delivery
    Delivery,

    // Display the receipt (Total price)
    Receipt,
}

// Application state
#[derive(Debug, Default)]
pub struct App {
    currentScreen: CurrentScreen,
    menu: Menu,
    deliveryMenu: Menu,
    menuBox: MenuBox,
    itemQuantity: u32,
    totalPrice: f32,
    exit: bool,
}

impl App {
    // Initialize and run the application
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::Result<()> {
        // The widgets
        self.menu = Menu::new(vec!["Keyboard - £25", "Mouse - £15", "Headset - £40"]);
        self.deliveryMenu = Menu::new(vec!["Standard Delivery - £3.99", "Next-Day Delivery - £7.99"]);

        self.menuBox = MenuBox::new(
            " CHOOSE NOW ".to_string(), self.menu.width.max(32), self.menu.height + 2,
            Style::default().fg(Color::from_u32(BORDER_COLOR)),
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
        match self.currentScreen {
            CurrentScreen::Menu => {
                // Set quantity back to zero if no item is chosen
                self.itemQuantity = 0;

                // Render the box/container and the menu list
                self.menuBox.render(frame);
                let (_block, innerArea) = self.menuBox.getBlockAndInnerArea(frame.area());
                self.menu.render(frame, innerArea);
            },

            CurrentScreen::QuantityInput => {
                self.menuBox.setTitle(" PICK QUANTITY ");
                self.menuBox.setSize(40, 10);
                self.menuBox.render(frame);
                if let Some(item) = self.menu.chosenItem {
                    // Split the item to get item and price and convert the price to float
                    let (item, price) = item.split_once(" - ").unwrap_or((item, ""));
                    let numPrice: f32 = price.trim_start_matches('£').trim().parse().unwrap_or(0.0);

                    // Calculate the total price of the items
                    self.totalPrice = if numPrice > 50.0 {
                        (numPrice + 1.1) * self.itemQuantity as f32
                    } else {
                        numPrice * self.itemQuantity as f32
                    };

                    let (_block, innerArea) = self.menuBox.getBlockAndInnerArea(frame.area());
                    let popup = Paragraph::new(
                        format!(
                            "\nItem: {} (£{})\n\nQuantity:  [ ▲ ]  {}  [ ▼ ]\n\n[Enter] Confirm   [Esc] Cancel",
                            item, self.totalPrice, self.itemQuantity
                        ))
                        .centered();

                    // Pass the full area (ignoring previous layouts)
                    frame.render_widget(popup, innerArea);
                }
            }

            CurrentScreen::Delivery => {
                // Render the box/container and the delivery methods
                self.menuBox.setTitle(" DELIVERY METHOD ");
                self.menuBox.render(frame);
                let (_block, innerArea) = self.menuBox.getBlockAndInnerArea(frame.area());
                self.deliveryMenu.render(frame, innerArea);
            },

            CurrentScreen::Receipt => {
                // Render the box/container and the total price of all items + delivery costs
                self.menuBox.setTitle(" RECEIPT ");
                self.menuBox.render(frame);
                let (_block, innerArea) = self.menuBox.getBlockAndInnerArea(frame.area());
                let receipt = Paragraph::new(format!("Your Total Is {:.2}", self.totalPrice))
                    .centered();
                frame.render_widget(receipt, innerArea);
            },
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
        // Default application keybindings
        match keyEvent.code {
            // Exit the program
            KeyCode::Esc | KeyCode::Char('q') => self.exit(),
            _ => {},
        }

        // Handle key events for specific widgets/screens whatever
        match self.currentScreen {
            CurrentScreen::Menu => {
                self.menu.handleKeyEvent(keyEvent.code).wrap_err("Failed to handle menu key event: \n{keyEvent:#?}")?;
                if self.menu.chosenItem.is_some() {
                    self.currentScreen = CurrentScreen::QuantityInput;
                }
            },

            CurrentScreen::QuantityInput => {
                if self.menu.chosenItem.is_some() {
                    match keyEvent.code {
                        KeyCode::Up | KeyCode::Char('k') => self.itemQuantity += 1,
                        KeyCode::Down | KeyCode::Char('j') => if self.itemQuantity > 0 {self.itemQuantity -= 1},
                        KeyCode::Enter => self.currentScreen = CurrentScreen::Delivery,
                        KeyCode::Backspace => {
                            // Go back to the menu to pick something else
                            self.currentScreen = CurrentScreen::Menu;
                            self.menu.chosenItem = None;
                        },
                        _ => {},
                    }
                }
            },

            CurrentScreen::Delivery => {
                self.deliveryMenu.handleKeyEvent(keyEvent.code).wrap_err("Failed to handle menu key event: \n{keyEvent:#?}")?;
                if let Some(item) = self.deliveryMenu.chosenItem {
                    let (_item, price) = item.split_once(" - ").unwrap_or((item, ""));
                    self.totalPrice += price.trim_start_matches('£').trim().parse().unwrap_or(0.0);
                    self.currentScreen = CurrentScreen::Receipt;
                }
            },

            CurrentScreen::Receipt => {},
        }

        return Ok(());
    }

    // Terminate the application
    fn exit(&mut self) {
        self.exit = true;
    }
}
