fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /*
    CONSTANTS:
    Constants aren’t just immutable by default—they’re always immutable.
    You declare constants using the const keyword instead of the let keyword,
    and the type of the value MUST be annotated.

    Constants can be declared in any scope, including the global scope,
    which makes them useful for values that many parts of code need to know about.

    The last difference is that constants may be set only to a constant expression,
    not the result of a value that could only be computed at runtime.

    Rust’s naming convention for constants is to use all uppercase with underscores between words.
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /*
    Shadowing:
    As you saw in the guessing game tutorial in Chapter 2,
    you can declare a new variable with the same name as a previous variable.

    Rustaceans say that the first variable is shadowed by the second,
    which means that the second variable’s value is what the program sees when the variable is used.
    */
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);
    /*
    Shadowing is different from marking a variable as mut, because we’ll get a compile-time error...
    if we accidentally try to reassign to this variable without using the let keyword.
    By using let, we can perform a few transformations on a value...
    ...but have the variable be immutable after those transformations have been completed.

    The other difference between mut and shadowing is that because we’re effectively...
    ...creating a new variable when we use the let keyword again,
    we can change the type of the value but reuse the same name.
    */
}
