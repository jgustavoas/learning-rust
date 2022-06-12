use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    /* gen_range(start..end) is inclusive on the lower bound but exclusive on the upper bound,
    so we need to specify 1..101 to request a number between 1 and 100.
    Alternatively, we could pass the range 1..=100, which is equivalent. */

    println!("The secret number is {}", secret_number);

    println!("Please input your guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
