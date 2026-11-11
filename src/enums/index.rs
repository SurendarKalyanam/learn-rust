use crate::structs::index::{Product, User};

#[derive(Debug)]
pub enum Output {
    User(User),
    Product(Product),
}
