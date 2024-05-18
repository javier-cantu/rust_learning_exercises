// Exercise 002 - Variables and Data Types

fn main() {
    // Variables in Rust are immutable by default. To make them mutable (i.e., their values can change), we use the `mut` keyword.
    
    // Declaring an immutable variable `x` of type integer (i32)
    let x: i32 = 10;
    
    // Declaring a mutable variable `y` of type float (f64)
    let mut y: f64 = 20.5;
    
    // Declaring a variable `name` of type string slice (&str)
    let name: &str = "Alice";
    
    // Printing the values of the variables to the console
    println!("Exercise 002");
    println!("The value of x is: {}", x); // {} is a placeholder for the variable `x`
    println!("The value of y is: {}", y);
    println!("Hello, {}!", name);
    
    // Modifying the value of the mutable variable `y`
    y = 25.75;
    println!("The new value of y is: {}", y);
}
