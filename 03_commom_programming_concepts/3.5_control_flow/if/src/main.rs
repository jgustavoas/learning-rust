fn main() {
    let number = 11;

    // The condition in this code must be a bool.
    // If the condition isn’t a bool, we’ll get an error.
    // For example, try running a condition "if number".
    // You must be explicit and always provide if with a Boolean as its condition.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // You can use multiple conditions by combining if and else in an else if expression:
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    /// Because if is an expression, we can use it on the right side of a let statement...
    ///...to assign the outcome to a variable, as in Listing 3-2.

    println!("The value of number is: {number}");

    /*
    Remember that blocks of code evaluate to the last expression in them,
    and numbers by themselves are also expressions. In this case,
    the value of the whole if expression depends on which block of code executes.

    This means the values that have the potential to be results from each arm of the if...
    ...must be the same type. If the types are mismatched, as in the following example,
    we’ll get an error When we try to compile this code, we’ll get an error:

    let number = if condition { 5 } else { "six" };

    The if and else arms have value types that are incompatible

    The expression in the if block evaluates to an integer,
    and the expression in the else block evaluates to a string.

    This won’t work because variables must have a single type,
    and Rust needs to know at compile time what type the number variable is, definitively.
    */
}
