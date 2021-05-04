// fn main() {
    // Functions are the primary way code is executed within Rust. You've already seen on of the most important functions in the language. The main function is the entry point of many programs.

    // Function definitions in Rust start with fn and have a set of parentheses after the function name. The braces tell the compiler where the function body begins and ends.
//     println!("Hello, world!");
//     another_function();
// }

// We could have declared the function before the main function, Rust doesn't care. It just needs to be defined somewhere.
// fn another_function() {
//     println!("Hello from another function!");
// }

// PASS PARAMETERS TO FUNCTIONS

// We're going to declare a function that checks if a given number is divisible by another and returns a boolean value to confirm that

fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    // If the divisor is zero, we want to return early with a 'false' value
    if divisor == 0 {
        return false;
    }
    
    dividend % divisor == 0
}

// Breaking down the above function:
// fn: The function declaration keyword in Rust
// is_divisible_by: The function name
// (dividend: u32, divisor: u32): This function's parameter list. It states that two unsigned 32-bit integers are expected as input values.
// -> bool: The arrow points to the type of value this function will always return.

// Looking at the body:
// if divisor == 0 {
//     return false;
// }

// The above code is trying to prevent the classic programming error, the divide by zero error.
// If says, if the divisor is equal to 0, we return false so that the function will stop and not try to divide by 0. If it isn't zero, the if statement is bypassed and we run:
// dividend % divisor == 0
// And do the division

// In Rust, the last expression inside a code block is always returned, so we don't need to use the return keyword in the above example.

fn main() {
    assert_eq!(is_divisible_by(2, 3), false);
    assert_eq!(is_divisible_by(5, 1), true);
    assert_eq!(is_divisible_by(24, 6), true);
}