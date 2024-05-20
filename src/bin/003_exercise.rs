// Exercise 003 - user input

// Import the 'io' module from the standard library to handle input and output
use std::io;


fn main() {
    println!("Guess the number!");

    // Prompting the user to input a number
    println!("Please input a number.");

    // Declaring a mutable variable of type String
    // 'String::new()' creates a new empty string
    let mut guess = String::new();

    // Using 'io::stdin()' to get the standard input handle
    // '.read_line(&mut guess)' to store the input into the guess variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    // Print the user's guess to the console
    println!("You guessed: {guess}");
}
