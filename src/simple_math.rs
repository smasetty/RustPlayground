/// Determines if a number is even
///
/// # Arguments
/// * `n` - The integer to check
///
/// # Returns
/// `true` if the number is even, `false` if odd
pub fn is_even(n: i32) -> bool {
    n % 2 == 0  
}

/// Adds two 32-bit integers and returns their sum plus an extra value
///
/// # Arguments
/// * `a` - The first integer operand
/// * `b` - The second integer operand
///
/// # Returns
/// The sum of `a` and `b` plus 10 as an i32
///
/// # Note
/// This function internally calls `add_with_extra` which adds 10 to the sum
pub fn add(a: i32, b: i32) -> i32 {
    add_with_extra(a, b)
}

/// Demonstrates checking if a number is even or odd
///
/// This function creates a number, checks if it's even using the `is_even` function,
/// and prints a description of the number to standard output.
///
/// # Example Output
/// ```text
/// 2 is even
/// ```
pub fn try_is_even(a: i32) {
    let description = match is_even(a) {
        true => "even",
        false => "odd",
    };

    println!("{} is {}", a, description);
}

/// Demonstrates basic arithmetic operations using the add function
///
/// This function creates two integer variables, adds them using the `add` function,
/// and prints the result to standard output.
pub fn try_simple_math() {
    println!("\n{}", "try_simple_math");
    let a = 1_i32;
    let b: i32 = 2;
    let c = 30_i32;
    let d = 30_i32;

    let e:i32 = add(add(a, b), add(c, d));
    println!("sum: {}", e);
}

fn add_with_extra(a: i32, b: i32) -> i32 {
    let extra = 10;
    a + b + extra
}