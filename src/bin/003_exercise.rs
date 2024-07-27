// Exercise 003 - User Input

use std::io; // Importing the `io` module from the standard library to handle input and output

fn main() {
    // Printing a greeting message to the console
    println!("Exercise 003 - User Input Part 1.");

    // Prompting the user to input something
    println!("Please input something.");

    // Declaring a mutable variable `input` of type `String` to store the user's input
    // `String::new()` creates a new empty string
    let mut input = String::new();

    // Using `io::stdin()` to get the standard input handle
    // Calling the `read_line` method on the standard input handle to read a line of input from the user
    // The `&mut input` argument tells `read_line` to store the input in the `input` variable
    // The `expect` method is called on the `Result` returned by `read_line`
    // If `read_line` fails, the program will panic and print the provided error message
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Printing the user's input to the console
    // Using `{}` to include the value of the `input` variable in the output string
    println!("Your input was: {input}");
}
