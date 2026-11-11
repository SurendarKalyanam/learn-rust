pub mod enums;
pub mod structs;
pub mod log;

use enums::index::Output;
use structs::index::{Product, User};
use log::console_log;

fn main() {
    let surendar = User {
        username: String::from("Surendar"),
        age: 12,
    };

    let product = Product {
        product_name: String::from("IPhone"),
        id: 1,
    };

    console_log(Output::Product(product));
    console_log(Output::User(surendar));
}
