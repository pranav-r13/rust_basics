use rand::Rng;
use std::io;

fn main() {
    print!("Guess the number between 1 and 100!\n");
    let random_num = rand::rng().random_range(0..=5);

    loop {
        let mut input_msg: String = String::new();

        io::stdin()
            .read_line(&mut input_msg)
            .expect("Cannot read number");

        let input_num = input_msg.trim().parse::<u32>().expect("Not a number");

        if input_num == random_num {
            print!("You guessed it right!");
            break;
        } else {
            print!("Wrong guess, try again!\n");
        }
    }
}
