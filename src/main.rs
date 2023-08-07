use std::io;

// Step 1: Create the enum with variants and values
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Step 2: Create the calculate function
fn calculate(op: Operation) -> f64 {
    // Step 3: Implement the function using pattern matching
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    // Step 4: Prompt the user for input
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: f64 = input1.trim().parse().expect("Invalid input");

    println!("Enter the operation (+, -, *, /):");
    let mut input_op = String::new();
    io::stdin().read_line(&mut input_op).expect("Failed to read line");
    let operator: char = input_op.trim().chars().next().expect("Invalid input");

    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: f64 = input2.trim().parse().expect("Invalid input");

    // Step 5: Create an Operation enum instance with the parsed input values
    let operation = match operator {
        '+' => Operation::Add(num1, num2),
        '-' => Operation::Subtract(num1, num2),
        '*' => Operation::Multiply(num1, num2),
        '/' => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    // Step 6: Call the calculate function with the created Operation enum instance
    let result = calculate(operation);

    // Step 7: Print the result to the console
    println!("Result: {}", result);
}

