fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*
In function signatures, you must declare the type of each parameter.
This is a deliberate decision in Rust’s design:
requiring type annotations in function definitions means the compiler almost never needs you...
...to use them elsewhere in the code to figure out what type you mean.

The compiler is also able to give more helpful error messages...
...if it knows what types the function expects.
*/

/*
Statements and Expressions:
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resulting value.

Let’s look at some examples:
Creating a variable and assigning a value to it with the let keyword is a statement.
let y = 6; is a statement.

Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11.

The 6 in the statement "let y = 6;" is an expression that evaluates to the value 6.

Calling a function is an expression.
Calling a macro is an expression.
A new scope block created with curly brackets is an expression, for example:
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

This expression...
{
    let x = 3;
    x + 1
}

...is a block that, in this case, evaluates to 4.
That value gets bound to y as part of the let statement.
Note that the x + 1 line doesn’t have a semicolon at the end,
unlike most of the lines you’ve seen so far.

Expressions do not include ending semicolons.
If you add a semicolon to the end of an expression, you turn it into a statement,
and it will then not return a value.
*/

/*
Functions with Return Values
Functions can return values to the code that calls them.
We don’t name return values, but we must declare their type after an arrow (->).
In Rust, the return value of the function is synonymous with the value of the final expression...
in the block of the body of a function.

You can return early from a function by using the return keyword and specifying a value,
but most functions return the last expression implicitly.

Here’s an example of a function that returns a value:
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
There are no function calls, macros, or even let statements in the "five" function...
...just the number 5 by itself. That’s a perfectly valid function in Rust.
*/
