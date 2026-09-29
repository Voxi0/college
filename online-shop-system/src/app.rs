use crate::{
    BORDER_COLOR, HIGHLIGHT_BG_COLOR, HIGHLIGHT_FG_COLOR,
    utils,
    app_state::AppState,
    container_box::ContainerBox, menu::Menu,
};

use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    style::{Color, Modifier, Style, Stylize},
    widgets::Paragraph,
    text::{Line, Text},
};

#[derive(Default)]
pub struct App {
    state: AppState,
    title: String,

    containerBox: ContainerBox,
    productSelectionMenu: Menu,
    deliveryMethodSelectionMenu: Menu,

    itemsQuantity: u32,
    itemsPrice: f32,
    discountPrice: f32,
    deliveryPrice: f32,
    totalPrice: f32,
}

impl App {
    // Create a new application with defaults
    pub fn new() -> App {
        let mut app = App {
            itemsQuantity: 1,
            ..Default::default()
        };

        // Create the container box
        app.containerBox = ContainerBox::new()
            .title(app.state.title())
            .width(60)
            .height(12)
            .borderStyle(
                Style::default()
                    .fg(Color::from_u32(BORDER_COLOR))
            );

        // Create the menus
        let menuHighlightStyle = Style::default()
            .bg(Color::from_u32(HIGHLIGHT_BG_COLOR))
            .fg(Color::from_u32(HIGHLIGHT_FG_COLOR))
            .add_modifier(Modifier::BOLD);

        app.productSelectionMenu = Menu::new()
            .items(["Keyboard - £25", "Mouse - £15", "Headset - £40"])
            .highlightStyle(menuHighlightStyle);
        app.deliveryMethodSelectionMenu = Menu::new()
            .items(["Standard Delivery - £3.99", "Next-Day Delivery - £7.99"])
            .highlightStyle(menuHighlightStyle);

        // Return the new application instance
        return app;
    }

    // Main loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        // Main loop
        while self.state != AppState::Quit {
            terminal.draw(|frame| self.render(frame).expect("[FATAL] Render error"))?;
            self.handleEvents()?;
        }

        return Ok(());
    }

    // Rendering
    fn render(&mut self, frame: &mut Frame) -> color_eyre::Result<()> {
        let containerArea = utils::getCenterArea(frame, &self.containerBox.getRect());
        let containerInnerArea = self.containerBox.getInnerArea(containerArea);
        self.containerBox.render(frame, containerArea);

        match self.state {
            AppState::ProductSelection => {
                self.productSelectionMenu.render(frame, containerInnerArea);
            }

            AppState::QuantitySelection => {
                let text = Text::from(vec![
                    Line::from("Item").bold().dim(),
                    Line::from(self.productSelectionMenu.getChosenItem()),
                    Line::from(""),
                    Line::from("Quantity").bold().dim(),
                    Line::from(self.itemsQuantity.to_string()),
                ])
                .centered();

                frame.render_widget(Paragraph::new(text), containerInnerArea);
            },

            AppState::DeliveryMethodSelection => {
                self.deliveryMethodSelectionMenu.render(frame, containerInnerArea);
            },

            AppState::Receipt => {
                self.totalPrice = self.itemsPrice + self.discountPrice + self.deliveryPrice;
                let text = Text::from(vec![
                    // Customer name
                    Line::from("Customer's Name").centered().bold(),
                    Line::from(""),

                    // Display the item and the quantity it's being bought in along with the total
                    // price of all items
                    Line::from(
                        format!(
                            "{} x {} = {}",
                            utils::splitItemAndPrice(self.productSelectionMenu.getChosenItem()).map_or("", |(item, _price)| item),
                            self.itemsQuantity,
                            self.itemsPrice
                        )
                    ).centered().bold().dim(),
                    
                    // 10% Discount price of all items
                    // Or just show no discount if it ain't available
                    if self.discountPrice > 0.0 {
                        Line::from(format!("Discount of 10% = {}", self.discountPrice)).centered().bold()
                    } else {
                        Line::from("No Discount Available").centered().bold()
                    },

                    // Delivery cost
                    Line::from(""),
                    Line::from(format!("Delivery Cost = {}", self.deliveryPrice)).centered().bold().dim(),

                    // The final cost
                    Line::from(""),
                    Line::from(format!("Final Cost = {:.2}", self.totalPrice)).centered().bold(),
                ]);

                frame.render_widget(Paragraph::new(text), containerInnerArea);
            },

            AppState::End => {
                frame.render_widget(
                    Paragraph::new(
                        Text::from(
                            Line::from("Try Again? (y/n)").centered().bold().style(Style::default().fg(Color::from_u32(HIGHLIGHT_FG_COLOR))),
                        )
                    ),
                    containerInnerArea
                );
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
                    AppState::QuantitySelection => {
                        match key.code {
                            KeyCode::Up | KeyCode::Char('k') => self.itemsQuantity = self.itemsQuantity.saturating_add(1),
                            KeyCode::Down | KeyCode::Char('j') => if self.itemsQuantity > 1 {self.itemsQuantity -= 1},
                            KeyCode::Backspace => self.itemsQuantity = 0,
                            KeyCode::Enter => {
                                if let Some((_item, price)) = utils::splitItemAndPrice(self.productSelectionMenu.getChosenItem()) {
                                    self.itemsPrice = price * self.itemsQuantity as f32;
                                    if self.itemsPrice > 50.0 {
                                        self.discountPrice = (price - (price * 0.1)) * self.itemsQuantity as f32;
                                    }
                                }
                            },
                            _ => {},
                        }
                    },
                    AppState::DeliveryMethodSelection => {
                        self.deliveryMethodSelectionMenu.handleKeyEvents(key.code);
                        match key.code {
                            KeyCode::Enter => {
                                if let Some((_item, price)) = utils::splitItemAndPrice(self.deliveryMethodSelectionMenu.getChosenItem()) {
                                    self.deliveryPrice = price;
                                }
                            },
                            _ => {},
                        }
                    },
                    AppState::End => {
                        match key.code {
                            KeyCode::Char('y') => self.state = AppState::default(),
                            KeyCode::Char('n') => self.state = AppState::Quit,
                            _ => {},
                        }
                    },
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
            // Quit application
            KeyCode::Esc | KeyCode::Char('q') => self.state = AppState::Quit,

            // Go to next/previous state
            KeyCode::Enter => {
                self.state.next();
                self.containerBox.setTitle(self.state.title());
            },
            KeyCode::Backspace => {
                self.state.prev();
                self.containerBox.setTitle(self.state.title());
            },

            _ => {},
        };
    }

    // Setters
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        return self;
    }
}
