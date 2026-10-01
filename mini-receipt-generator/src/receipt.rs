#[derive(Default)]
pub struct Receipt {
    items: Vec<String>,
    item_prices: Vec<f32>,
    item_quantities: Vec<i32>,
    total_price: f32,
}

impl Receipt {
    pub fn add_item(&mut self, item_name: &str, item_price: f32, item_quantity: i32) {
        self.items.push(item_name.to_string());
        self.item_prices.push(item_price);
        self.item_quantities.push(item_quantity);
    }

    pub fn print(&mut self) {
        println!("\tRECEIPT");
        for (index, _item) in self.items.iter().enumerate() {
            println!("{} x {} = {}", self.items[index], self.item_quantities[index], self.item_prices[index]);
            self.total_price += self.item_prices[index] * self.item_quantities[index] as f32;
        }
        println!("");
        println!("Total: {}", self.total_price);
    }
}
