fn main() {
    /*
    Returning Values from Loops:
    One of the uses of a loop is to retry an operation you know might fail,
    such as checking whether a thread has completed its job.
    You might also need to pass the result of that operation out of the loop to the rest of your code.
    To do this, you can add the value you want returned after the break expression you use to stop the loop;
    that value will be returned out of the loop so you can use it, as shown here.
    */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // After the loop, we use a semicolon to end the statement that assigns the value to result.

    println!("The result is {result}");

    /*
    Loop Labels to Disambiguate Between Multiple Loops:
    If you have loops within loops, break and continue apply to the innermost loop at that point.
    You can optionally specify a loop label on a loop that we can then use with break or continue...
    ...to specify that those keywords apply to the labeled loop instead of the innermost loop.
    Loop labels must begin with a single quote.

    Here’s an example with two nested loops:
    */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    /*
    The outer loop has the label 'counting_up, and it will count up from 0 to 2.
    The inner loop without a label counts down from 10 to 9.
    The first break that doesn’t specify a label will exit the inner loop only.
    The break 'counting_up; statement will exit the outer loop.
    */
    println!("End count = {count}");

    /* Conditional Loops with WHILE: */
    let mut number = 3;

    // Using a while loop to run code while a condition holds true:
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    /*
    Looping Through a Collection with FOR
    You can choose to use the while construct to loop over the elements of a collection, such as an array.
    For example, this loop prints each element in the array a.
    */
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    /*
    ...However, this approach is error prone;
    we could cause the program to panic if the index value or test condition are incorrect.

    For example, if you changed the definition of the a array to have four elements...
    ...but forgot to update the condition to while index < 4, the code would panic.

    It’s also slow, because the compiler adds runtime code to perform the conditional check...
    ...of whether the index is within the bounds of the array on every iteration through the loop.

    As a more concise alternative, you can use a for loop and execute some code..
    ...for each item in a collection.

    When we run this code, we’ll see the same output as the previous example.

    More importantly, we’ve now increased the safety of the code and eliminated the chance of bugs...
    ...that might result from going beyond the end of the array or not going far enough...
    ...and missing some items.
    */
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    /*
    The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
    Even in situations in which you want to run some code a certain number of times,
    as in the countdown example that used a while loop, most Rustaceans would use a for loop.

    The way to do that would be to use a Range, provided by the standard library,
    which generates all numbers in sequence starting from one number and ending before another number.

    Here’s what the countdown would look like using a for loop and another method..
    ...we’ve not yet talked about, "rev", to reverse the range:
    */
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    // This code is a bit nicer, isn’t it?
}
