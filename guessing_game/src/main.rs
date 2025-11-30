// use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    print!("Guess the number between 1 and 100!\n");
    let random_num = rand::rng().random_range(0..=5);

    loop {
        let mut input_msg: String = String::new();

        io::stdin()
            .read_line(&mut input_msg)
            .expect("Cannot read number");

        let input_num: u32 = match input_msg.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // if input_num == random_num {
        //     print!("You guessed it right!");
        //     break;
        // } else {
        //     print!("Wrong guess, try again!\n");
        // }

        match input_num.cmp(&random_num) {
            Ordering::Less => println!("{}","Too small! Try again.\n".red()),
            Ordering::Greater => println!("{}","Too big".red()),
            Ordering::Equal => {
                println!("{}","You guessed it right!\n".blue());
                break;
            }
        }
    }
}
