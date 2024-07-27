// Exercise 004 - User Input simple calculator

use std::io; // Import the 'io' module from the standard library to handle input and output
use std::num::ParseIntError; // Import 'ParseIntError' to handle parsing errors

fn main() {
    println!("Exercise 004 - Basic Calculator");
    
    // Function to read user input and return it as a String
    fn read_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }

    // Function to parse a string to an integer, with error handling
    fn parse_int(input: &str) -> Result<i32, ParseIntError> {
        input.parse::<i32>()
    }

    // Reading first number from user
    let num1 = loop {
        let input = read_input("Please enter the first number:");
        match parse_int(&input) {
            Ok(n) => break n,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    };

    // Reading second number from user
    let num2 = loop {
        let input = read_input("Please enter the second number:");
        match parse_int(&input) {
            Ok(n) => break n,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    };

    // Reading operation from user
    let operation = loop {
        let input = read_input("Please enter an operation (+, -, *, /):");
        match input.as_str() {
            "+" | "-" | "*" | "/" => break input,
            _ => println!("Invalid operation, Please enter one of +, -, *, /."),
        }
    };

    // Perform the chosen operation
    let result = match operation.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0 {
                num1 / num2
            } else {
                println!("Error: Division by zero is not allowed.");
                return;
            }
        }
        _ => unreachable!(),
    };

    // Printing the result
    println!("The result of {} {} {} is: {}", num1, operation, num2, result);
}







