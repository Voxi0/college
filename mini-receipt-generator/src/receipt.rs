#[derive(Default)]
pub struct Receipt {
    // Item name, price and quantity per entry
    items: Vec<(String, f32, i32)>,
    total_price: f32,
}

impl Receipt {
    pub fn add_item(&mut self, item_name: &str, item_price: f32, item_quantity: i32) {
        self.items.push((item_name.to_string(), item_price, item_quantity));
    }

    pub fn print(&mut self) {
        println!("\tRECEIPT");
        for item in &self.items {
            let (name, price, quantity) = item;
            println!("{} x {} = {}", name, quantity, price);
            self.total_price += price * *quantity as f32;
        }
        println!("");
        println!("Total: {}", self.total_price);
    }
}
