#[derive(Default, PartialEq)]
pub enum AppState {
    #[default]
    ProductSelection,
    QuantitySelection,
    DeliveryMethodSelection,
    Receipt,
    End,
    Quit,
}

impl AppState {
    pub fn next(&mut self) {
        *self = match self {
            AppState::ProductSelection => AppState::QuantitySelection,
            AppState::QuantitySelection => AppState::DeliveryMethodSelection,
            AppState::DeliveryMethodSelection => AppState::Receipt,
            AppState::Receipt => AppState::End,
            AppState::End => AppState::Quit,
            AppState::Quit => todo!(),
        };
    }

    pub fn prev(&mut self) {
        *self = match self {
            AppState::ProductSelection => AppState::ProductSelection,
            AppState::QuantitySelection => AppState::ProductSelection,
            AppState::DeliveryMethodSelection => AppState::QuantitySelection,
            AppState::Receipt => AppState::DeliveryMethodSelection,
            AppState::End => AppState::Receipt,
            AppState::Quit => todo!(),
        };
    }

    pub fn title(&self) -> &str {
        match self {
            AppState::ProductSelection => " CHOOSE PRODUCT ",
            AppState::QuantitySelection => " CHOOSE QUANTITY ",
            AppState::DeliveryMethodSelection => " CHOOSE DELIVERY METHOD ",
            AppState::Receipt => " RECEIPT ",
            AppState::End => " End ",
            AppState::Quit => "",
        }
    }
}
