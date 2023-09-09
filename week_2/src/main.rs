use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: &Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    let mut first_number = String::new();
    let mut operation = String::new();
    let mut second_number = String::new();

    println!("Enter first number: ");

    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    let first_number: f64 = first_number.trim().parse().expect("Please type a number");

    println!("Operation [CHOOSE ONE: +, -, *, /]: ");

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let operation = operation.trim();

    if !matches!(operation, "+" | "-" | "*" | "/") {
        panic!("Invalid operator");
    }

    println!("Enter second number: ");

    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    let second_number: f64 = second_number.trim().parse().expect("Please type a number");

    let operation = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => panic!("Unknown operation"),
    };

    let result = calculate(&operation);

    println!("Calculation result: {result}");
}

#[test]
fn calculate_operation_should_yield_correct_result() {
    // Addition
    assert_eq!(calculate(&Operation::Add(2.0, 3.0)), 5.0);

    // Subtraction
    assert_eq!(calculate(&Operation::Subtract(7.0, 6.0)), 1.0);

    // Multiplication
    assert_eq!(calculate(&Operation::Multiply(12.0, 12.0)), 144.0);

    // Division
    assert_eq!(calculate(&Operation::Divide(12.0, 6.0)), 2.0);
}
