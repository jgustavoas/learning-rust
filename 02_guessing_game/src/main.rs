use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    /* gen_range(start..end) is inclusive on the lower bound but exclusive on the upper bound,
    so we need to specify 1..101 to request a number between 1 and 100.
    Alternatively, we could pass the range 1..=100, which is equivalent. */

    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            /* Remember that parse returns a Result type...
            ...and Result is an enum that has the variants "Ok" and "Err". */
            Ok(num) => num,
            Err(_) => continue,
            /* The underscore, _, is a catchall value;
            in this example, we’re saying we want to match all Err values,
            no matter what information they have inside them. */
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
                /* "break" makes the program exit the loop when the user guesses the secret number correctly.
                Exiting the loop also means exiting the program, BECAUSE THE LOOP IS THE LAST PART OF MAIN. */
            }
        }
    }
}
