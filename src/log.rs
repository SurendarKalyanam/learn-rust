use crate::enums::index::Output;
pub fn console_log(value: Output) {
    match value {
        Output::Product(p) => println!("Product details: {:#?}", p),
        Output::User(u) => println!("User details: {:?}", u),
    }
}
