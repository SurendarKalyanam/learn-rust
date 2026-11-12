pub mod enums;
pub mod log;
pub mod structs;
pub mod tuts;

use enums::index::Output;
use log::console_log;
use structs::index::{Product, User};
use tuts::rust_101_tuts;

fn main() {
    let surendar = User {
        username: String::from("Surendar"),
        age: 12,
    };

    let iphone = Product {
        product_name: String::from("IPhone"),
        id: 1,
    };

    console_log(Output::Product(iphone));
    console_log(Output::User(surendar));

    rust_101_tuts();
}
