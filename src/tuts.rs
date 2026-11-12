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
}
