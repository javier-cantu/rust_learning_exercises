// Exercise 006 - List Operations
// Objectives:
//1. Add numbers to a list: Allow the user to add multiple numbers to a list.
//2. Calculate the mean: Compute the average of the numbers in the list.
//3. Find the maximum and minimum numbers: Identify the largest and smallest numbers in the list.
//4. Show the sorted list: Sort and display the list of numbers.

use std::io;

// Function to Read User Input
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


fn main() {
    println!("Exercise 006 - List Operations");

    // Vector to store numbers
    let mut numbers: Vec<f64> = Vec::new();

    // Main loop to handle user choices
    loop {
        let choice = read_input("Choose an option: \n1. Add a number \n2. Calculate mean \n3. Find max and min \n4.Show sorted list \n5. Exit \n(Type 1, 2, 3, 4, or 5):");

        match choice.trim() {
            // Option 1: Add a number
            "1" => {
                let num_str = read_input("Enter a number:");
                match num_str.trim().parse::<f64>() {
                    Ok(num) => {
                        numbers.push(num);
                        println!("Number added.");
                    }
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                }
            }
            // Option 2: Calculate mean
            "2" => {
                if numbers.is_empty() {
                    println!("The list is empty.");
                } else {
                    let sum: f64 = numbers.iter().sum();
                    let mean = sum / numbers.len() as f64;
                    println!("The mean is {:.2}.", mean);
                }
            }
            // Option 3: Find max and min
            "3" => {
                if numbers.is_empty() {
                    println!("The list is empty.");
                } else {
                    let max = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let min = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
                    println!("The maximum number is {:.2} and the minimum number is {:.2}.", max, min);
                }
            }
            // Option 4: Show sorted list
            "4" => {
                if numbers.is_empty() {
                    println!("The list is empty.");
                } else {
                    let mut sorted_numbers = numbers.clone();
                    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    println!("The sorted list is: {:?}", sorted_numbers);
                }
            }
            // Option 5: Exit
            "5" => {
                println!("Exiting the program.");
                break;
            }
            // Invalid choice
            _ => println!("Invalid choice. Please enter 1, 2, 3, 4, or 5."),
        }
    }
}
