// youtube video playlist link => https://www.youtube.com/watch?v=jpjo1HW5Fnk&list=PLAscMa3kKhHjBYOTrT20kpYMNQX95KU6i&index=2

pub fn rust_101_tuts() {
    println!("rust 101 tuts ...");

    // variable shadowing...

    let n = 5;
    {
        let n = 10;
        println!("inner n is: {n}");
    }
    println!("outer n is : {n}");

    let spaces = "      ";
    let spaces = spaces.len();
    println!("{spaces}");

    // Data types

    let user_input = "100";
    let converted: u32 = user_input.parse().expect("Please enter valid number");
    println!("converted={converted}");

    let number: u8 = 100; //integer
    let pi: f64 = 3.1415; //floats
    let turned_on: bool = true;
    let delta: char = 'A';

    println!("number is {number}, pi is {pi}, turned_on is {turned_on}, delta is {delta} ");

    let coordinates: (f32, f32) = (1.5, 2.3); // tuples
    let people: [&str; 2] = ["Sudhakar", "Surendar"]; // array

    println!("{:?} , {:?}", coordinates, people);

    println!("{}", i8::MAX);
    println!("{}", i8::MIN);
    println!("{}", 1_00_000);
}
