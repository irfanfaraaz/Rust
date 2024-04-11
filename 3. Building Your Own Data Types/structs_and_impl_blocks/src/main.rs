struct Product{
    name: String,
    price: f64,
    in_stock: bool,
}

impl Product{
    fn new(name: String, price: f64) -> Product {
        Product {
            name,
            price,
            in_stock:true,
        }
    }
    fn get_default_sales_tax() -> f64 {
        0.06
    }
    fn calculate_sales_tax(&self) -> f64 {
        self.price * Product::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    fn buy(self)->i32{
        let name = self.name;
        println!("You bought a {}", name);
        123
    }
}
fn main() {
    let mut book:Product = Product::new(String::from("Rust Programming"), 49.99);
    let price = book.price;
    book.in_stock = false;

    let sales_tax = book.calculate_sales_tax();
    println!("The sales tax for the book is: {}", sales_tax);

    book.set_price(29.99);
    book.buy();
    // below block throws an error as buy takes self and wnership has been transferred and dropped in the above line hence it cannot be changed again. Exactly what we want
    // book.set_price(39.99);
     
}

