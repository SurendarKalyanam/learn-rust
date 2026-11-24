// youtube video playlist link => https://www.youtube.com/watch?v=jpjo1HW5Fnk&list=PLAscMa3kKhHjBYOTrT20kpYMNQX95KU6i&index=2

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Read;

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
        fn new(url: &str) -> Self {
            // Associated Function
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

        fn show_status(self: Self) -> Self {
            println!(
                "Product Name {} and is available {}.",
                self.name, self.is_purchased
            );
            self
        }

        fn log() {
            println!("Log Product ....");
        }
    }

    Product::new("Iphone").purchase(true).show_status();
    Product::log();

    // loop

    let mut count = 10;

    let result_of_loop = loop {
        println!("Countdown {}", count);
        count -= 1;
        if count == 0 {
            break "COUNTDOWN DONE";
        }
    };

    println!("{}", result_of_loop);

    // while loop

    let mut while_loop_count = 6;

    while while_loop_count > 0 {
        while_loop_count -= 1;

        if while_loop_count == 3 {
            println!("Skip 3");
            continue;
        }
        println!("While Loop {while_loop_count}");
    }

    // for loop

    let mut names = vec!["Surendar", "Sudhakar", "Puppy"];
    names.push("Kalyanam");
    for name in names {
        dbg!(name);
    }

    let mut main_count = 0;

    'main: loop {
        println!("Outer loop or Main loop: {main_count}");
        let mut inner_count = 0;
        'inner: loop {
            println!("inner loop: {inner_count}");
            if inner_count == 3 {
                break 'inner;
            }
            inner_count += 1;

            if main_count == 3 {
                break 'main;
            }
        }
        main_count += 1;
    }
    //clone

    let username_one = String::from("Surendar");
    let username_two = username_one.clone();

    dbg!(username_one);
    dbg!(username_two);

    fn set_brightness(brightness: Option<i32>) {
        if let Some(value) = brightness {
            println!("Brightness get the value {value}%");
        }
    }

    set_brightness(Some(50));

    enum Crypto {
        Btc(i32),
        Eth(i32),
    }

    let coin = Crypto::Btc(3);

    if let Crypto::Btc(amount) = coin {
        println!("your bitcoin is {amount}");
    }

    if let Crypto::Eth(amount) = Crypto::Eth(5) {
        println!("Your Etherium is {amount}")
    }

    // Error handling

    // panic!("Custom or manual error");

    let path = "./src/test.txt";
    let txt = File::open(path);

    match txt {
        Ok(mut file) => {
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);
            println!("successfully read the content. {}", contents)
        }
        Err(e) => println!("{:?}", e),
    }

    // fn get_data() -> Result<String, io::Error> {
    //     let mut data = String::new();
    //     File::open("./src/test.txt")?.read_to_string(&mut data)?;
    //     Ok(data)
    // }

    // dbg!(get_data());

    // Vector

    let mut players = vec!["Virat", "Yuvi", "Dhoni"];
    players.push("Surendar");

    dbg!(players);

    // format...
    let s1 = String::from("Surendar");
    let s2 = String::from("Yama");
    let s3 = String::from("Kalyanam");

    let s4 = format!("{s1}-{s2}-{s3}");
    dbg!(s4, s1);

    let mut scores = HashMap::new();
    scores.insert(String::from("Surendar"), 90);
    scores.insert(String::from("Sudhakar"), 100);

    // dbg!(scores);
    // let scores = scores.get("Surendar").copied().unwrap_or(0);
    // dbg!(scores);

    for (k, v) in &scores {
        println!("{k} score is {v}");
    }

    trait Log {
        fn log(&self) -> String;
    }

    struct Cricketer {
        name: String,
    }

    struct Youtuber {
        name: String,
    }

    impl Log for Cricketer {
        fn log(&self) -> String {
            format!("Cricketer Name is: {}", self.name)
        }
    }

    impl Log for Youtuber {
        fn log(&self) -> String {
            format!("Youtuber Name is: {}", self.name)
        }
    }

    let cricketer = Cricketer {
        name: String::from("Virat"),
    };

    let youtuber = Youtuber {
        name: String::from("Theo"),
    };

    dbg!(cricketer.log());
    dbg!(youtuber.log());

    fn func<T: Display + Clone, U: Clone + Debug>() -> i32 {
        unimplemented!()
    }

    fn func2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        unimplemented!()
    }

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    struct Tweet {
        content: String,
        username: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{} is {}", self.username, self.content)
        }
    }

    fn returns_summarizabke() -> impl Summary {
        Tweet {
            content: "Learning Rust Programming Language".to_string(),
            username: "SurendarYama".to_string(),
        }
    }

    let item = returns_summarizabke();
    println!("{:?}", item.summarize());
}
