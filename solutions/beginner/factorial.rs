// Exercise: Implement a function that takes an integer and returns its factorial.

// Here is the function signature:
fn factorial(num: u32) -> u32 {
    // Your code goes here
    let mut factorial: u32 = 1; // to make sure 0! is 1 and not 0
    let mut i: u32 = num;
    
    while i > 1 {
        factorial *= i;
        i -= 1;
    }
    
    return factorial;
}

// ALTERNATE SOLUTIONS
// -------------------
// fn factorial(num: u32) -> u32 {
//     if num == 0 {
//         1
//     } else {
//         num * factorial(num - 1)
//     }
// }

// Write your code to calculate the factorial of 'num' and return the result.
// Remember, the factorial of a number is the product of all positive integers less than or equal to the number.
// For example, the factorial of 5 (5!) is 5 * 4 * 3 * 2 * 1 = 120.

// After implementing the function, you can test it with different values:
fn main() {
    println!("The factorial of 5 is {}", factorial(5));
    // Output should be: The factorial of 5 is 120
}
