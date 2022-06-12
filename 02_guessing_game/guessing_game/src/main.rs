use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secre_number = rand::thread_rng().gen_range(1..101);
    /* gen_range(start..end) is inclusive on the lower bound but exclusive on the upper bound,
    so we need to specify 1..101 to request a number between 1 and 100.
    Alternatively, we could pass the range 1..=100, which is equivalent. */

    println!("The secret number is {}", secre_number);

    println!("Please input your guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
