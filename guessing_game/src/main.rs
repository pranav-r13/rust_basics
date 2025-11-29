use std::io;

fn main() {
    println!("Hello, world!");

    println("Please input a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(buf: &mut guess)
        .expect("Failed to read line");
}
