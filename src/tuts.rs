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

    let letter: char = 'D';
    println!("letter={}", letter);

    //tuple

    let data = ("Surendar", 13, false);
    let (name, age, is_drive): (&str, u8, bool) = ("Sudhakar", 10, false);
    println!("data={:?}", data);
    println!("{name}, {age}, {is_drive}");
    println!("{:?}", data.0);
    println!("{:#?}", i8::MAX);

    // arrays

    let numbers = [2, 3, 4, 5];
    println!("numbers={:?}", numbers);

    let repeat = ["yama"; 5];
    println!("repeat={:?}", repeat);

    let first_number = numbers[0];
    println!("first_number={:?}", first_number);

    fn say_hi(name: &str) {
        println!("Hi {}...", name);
    }

    say_hi("Surendar");
    say_hi("Kalpana");

    fn repeat_name(name: &str, times: usize) {
        println!("{}", name.repeat(times));
    }

    repeat_name("example ", 3);
    dbg!(repeat);

    // statements vs expression.
    // if code returns value is called expression otherwise
    // it called statement.

    let name = "Lakshan";
    let pn = println!("My Fav name is: {name}");
    println!("{:?}", pn);
    let dbg_value = dbg!(90 + 20);
    println!("{dbg_value}");

    let sum = {
        let x = 10;
        let y = 10;

        x + y
    };

    println!("sum={}", sum);

    // controlflow...

    fn check_password_length(password: &str) {
        let length = password.chars().count();
        if length >= 10 {
            println!("Password length is ok !...");
        } else {
            println!("Your password is not long enough ...");
        }
    }

    check_password_length("password123");

    // elseif

    fn get_response(message: &str) -> &str {
        let message = message.to_lowercase();
        if message == "hello" {
            "Hello"
        } else if message == "vanakkam" {
            "Vanakkam"
        } else {
            "I don't understand..."
        }
    }

    println!("{}", get_response("Hello"));
    println!("{}", get_response("Vanakkam"));
    println!("{}", get_response("Hola"));

    #[derive(Debug)]
    struct Request {
        url: String,
        timeout: u32,
    }

    impl Request {
        fn new(url: &str) -> Self { // Associated Function
            Self {
                url: url.to_lowercase(),
                timeout: 30,
            }
        }
        fn timeout(mut self, seconds: u32) -> Self {
            self.timeout = seconds;
            self
        }

        fn send(self) {
            println!("URL = {} and Timeout = {}", self.url, self.timeout);
        }
    }

    Request::new("https://google.co.in").timeout(40).send();

    println!("{:#?}", Request::new("https://google.co.in"));

    #[derive(Debug)]
    struct Product {
        name: String,
        is_purchased: bool,
    }

    impl Product {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                is_purchased: false,
            }
        }

        fn purchase(mut self: Self, is_purchased: bool) -> Self {
            self.is_purchased = is_purchased;
            self
        }

        fn show_status(self: Self) {
            println!(
                "Product Name {} and is available {}.",
                self.name, self.is_purchased
            );
        }
    }

    Product::new("Iphone").purchase(true).show_status();

    
}
