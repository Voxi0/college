use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
    value: i32,
}
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
}

#[test]
fn testCounter() {
    let mut counter: Counter = Counter::default();
    counter.update(Message::Increment);
    assert_eq!(counter.value, 1);
    counter.update(Message::Decrement);
    assert_eq!(counter.value, 0);
}

fn main() {
    println!("Hello, world!");
}
