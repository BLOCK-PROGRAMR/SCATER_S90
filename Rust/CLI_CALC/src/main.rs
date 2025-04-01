use std::io;

fn main() {
    println!("Simple CLI Calculator in Rust!");
    
    loop {
        println!("\nEnter first number (or type 'exit' to quit):");
        let num1 = read_number();
        if num1.is_none() { break; }
        let num1 = num1.unwrap();

        println!("Enter an operator (+, -, *, /):");
        let operator = read_operator();

        println!("Enter second number:");
        let num2 = read_number();
        if num2.is_none() { break; }
        let num2 = num2.unwrap();

        let result = calculate(num1, num2, operator);
        println!("Result: {}", result);
    }

    println!("Goodbye!");
}

// Function to read a number from user input
fn read_number() -> Option<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let trimmed = input.trim();
    if trimmed == "exit" {
        return None;
    }
    match trimmed.parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Invalid number! Try again.");
            read_number()
        }
    }
}

// Function to read an operator from user input
fn read_operator() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let trimmed = input.trim();
    if trimmed.len() == 1 {
        return trimmed.chars().next().unwrap();
    }
    println!("Invalid operator! Try again.");
    read_operator()
}

// Function to perform the calculation
fn calculate(num1: f64, num2: f64, operator: char) -> f64 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                println!("Error: Division by zero!");
                return 0.0;
            }
            num1 / num2
        },
        _ => {
            println!("Invalid operator! Defaulting to 0.");
            0.0
        }
    }
}
