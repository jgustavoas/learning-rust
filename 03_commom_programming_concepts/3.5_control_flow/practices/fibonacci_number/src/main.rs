fn main() {
    get_fibonacci_number(10);
}

fn get_fibonacci_number(nth: i32) {
    let mut previous_fibo = 0;
    let mut current_fibo = 1;
    let mut fibo_nth = 1;

    for _i in 2..=nth {
        fibo_nth = current_fibo + previous_fibo;
        previous_fibo = current_fibo;
        current_fibo = fibo_nth;
    }

    println!("The Fibonacci number for {nth} is {fibo_nth}");
}
