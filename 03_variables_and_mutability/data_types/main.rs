fn main() {
    /* Every value in Rust is of a certain data type,
    which tells Rust what kind of data is being specified so it knows how to work with that data.

    We’ll look at two data type subsets: SCALAR and COMPOUND.*/

    // 1 - SCALAR TYPES
    /* 1.1 - Integer Types:
    An integer is a number without a fractional component.
    We used one integer type in Chapter 2, the u32 type.
    This type declaration indicates that the value it’s associated with...
    ...should be an unsigned integer (signed integer types start with i, instead of u)...
    ...that takes up 32 bits of space. Table 3-1 shows the built-in integer types in Rust.
    We can use any of these variants to declare the type of an integer value.

    Each variant can be either signed or unsigned and has an explicit size.
    Signed and unsigned refer to whether it’s possible for the number to be negative—in other words,
    whether the number needs to have a sign with it (signed) or whether it will...
    ...only ever be positive and can therefore be represented without a sign (unsigned). */

    /* 1.2 - Floating-Point Types */
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric Operations:
    /// addition
    let sum = 5 + 10;

    /// subtraction
    let difference = 95.5 - 4.3;

    /// multiplication
    let product = 4 * 30;

    /// division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    /// remainder
    let remainder = 43 % 5;

    /* 1.3 - The Boolean type */
    let t = true;
    let f: bool = false; // with explicit type annotation

    /* 1.4 - The Character Type */
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    /* Note that we specify char literals with single quotes,
    as opposed to string literals, which use double quotes.

    Rust’s char type is four bytes in size and represents a Unicode Scalar Value,
    which means it can represent a lot more than just ASCII.

    Accented letters; Chinese, Japanese, and Korean characters;
    emoji; and zero-width spaces are all valid char values in Rust. */

    /* 2 - COMPOUND TYPES */
    // 2.1 - The Tuple Type:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    /* A tuple is a general way of grouping together a number of values...
    with a variety of types into one compound type. Tuples have a fixed length: once declared,
    they cannot grow or shrink in size. */

    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);

    /* We can also access a tuple element directly by using a period (.) followed by...
    ...the index of the value we want to access. For example: */
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 2.2 - The Array type:
    let a = [1, 2, 3, 4, 5];
    /*
    Unlike a tuple, every element of an array must have the same type.
    Unlike arrays in some other languages, arrays in Rust have a fixed length.

    Arrays are useful when you want your data allocated on the stack rather than the heap...
    ...(we will discuss the stack and the heap more in Chapter 4) or when you want to ensure...
    ...you always have a fixed number of elements.

    An array isn’t as flexible as the vector type, though.
    A vector is a similar collection type provided by the standard library that is allowed...
    ...to grow or shrink in size. If you’re unsure whether to use an array or a vector,
    chances are you should use a vector. Chapter 8 discusses vectors in more detail.

    However, arrays are more useful when you know the number of elements will not need to change.
    For example, if you were using the names of the month in a program, you would probably...
    ...use an array rather than a vector because you know it will always contain 12 elements:
    */
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    /* You write an array’s type using square brackets with the type of each element,
    a semicolon, and then the number of elements in the array, like so: */
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    /* You can also initialize an array to contain the same value for each element...
    ...by specifying the initial value, followed by a semicolon,
    and then the length of the array in square brackets, as shown here: */
    let a = [3; 5]; // This is the same as writing let a = [3, 3, 3, 3, 3];

    /* When you attempt to access an element using indexing,
    Rust will check that the index you’ve specified is less than the array length.
    If the index is greater than or equal to the length, Rust will panic.
    This check has to happen at runtime, especially in this case,
    because the compiler can’t possibly know what value a user will enter when they run the code later. */
}
