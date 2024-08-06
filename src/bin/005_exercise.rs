// Exercise 005 - Simple Temperature Converter

use std::io;

// main function
fn main() {
    println!("Exercise 005 - Simple Temperature Converter");

    loop {
        let choice = read_input("Choose conversion type: \n1. Celsius to Fahrenheit \n2. Fahrenheit to Celsius \n(Type 1 or 2):");

        match choice.trim() {
            "1" => {
                let celsius = read_input("Enter temperature in Celsius:");
                match celsius.trim().parse::<f64>() {
                    Ok(c) => {
                        let fahrenheit = c * 9.0 / 5.0 + 32.0;
                        println!("{:.2} Celsius is {:.2} Fahrenheit", c, fahrenheit);
                    }
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                }
            },
            "2" => {
                let fahrenheit = read_input("Enter temperature in Fahrenheit:");
                match fahrenheit.trim().parse::<f64>() {
                    Ok(f) => {
                        let celsius = (f - 32.0) * 5.0 / 9.0;
                        println!("{:.2} Fahrenheit is {:.2} Celsius", f, celsius);
                    }
                    Err(_) => println!("Invalid input. Please enter a valid number."),
            }
        },
        _ => println!("Invalid choice. Please enter 1 or 2"),
        }
    }
}

// Function to read user input and return it as a String
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
