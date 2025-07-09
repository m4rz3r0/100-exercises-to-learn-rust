// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: usize,
    unit_price: usize,
}

impl Order {
    pub fn new(product_name: String, quantity: usize, unit_price: usize) -> Order {
        Self::check_product_name(&product_name);
        Self::check_quantity(quantity);
        Self::check_unit_price(unit_price);

        Self {
            product_name,
            quantity,
            unit_price
        }
    }

    fn check_product_name(product_name: &String) {
        if product_name.is_empty() {
            panic!("The product name is empty");
        }

        if product_name.len() > 300 {
            panic!("The product name is too long!");
        }
    }

    fn check_quantity(quantity: usize) {
        if quantity == 0 {
            panic!("Quantity is zero");
        }
    }

    fn check_unit_price(unit_price: usize) {
        if unit_price <= 0 {
            panic!("The unit price is too small!");
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &usize {
        &self.quantity
    }

    pub fn unit_price(&self) -> &usize {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String) {
        Self::check_product_name(&product_name);
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: usize) {
        Self::check_quantity(quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: usize) {
        Self::check_unit_price(unit_price);
        self.unit_price = unit_price;
    }

    pub fn total(&self) -> usize {
        self.quantity * self.unit_price
    }
}