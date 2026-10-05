use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value = self.value.saturating_add(1),
            Message::Decrement => self.value = self.value.saturating_sub(1),
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
}

#[test]
fn test_counter() {
    let mut counter: Counter = Counter::default();

    counter.update(Message::Increment);
    assert_eq!(counter.value, 1);

    counter.update(Message::Decrement);
    assert_eq!(counter.value, 0);

    counter.update(Message::Decrement);
    assert_eq!(counter.value, -1);
}

fn main() -> iced::Result {
    let counter: Counter = Counter::default();
    iced::run(Counter::update, Counter::view)
}
