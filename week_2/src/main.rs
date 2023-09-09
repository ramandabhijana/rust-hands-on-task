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
    println!("Hello, world!");
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
