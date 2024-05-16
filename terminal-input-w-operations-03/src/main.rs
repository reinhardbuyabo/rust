use std::io::{self};

fn main() {
    println!("Enter first number: ");
    let mut first_number: String = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read first number.");
    println!("Enter operations: ");
    let mut operation: String = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read operation.");
    println!("Enter second number: ");
    let mut second_number: String = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read second number");

    let first_no: i32 = first_number.trim().parse().expect("Invalid First Number");
    let second_no: i32 = second_number.trim().parse().expect("Invalid Second Number");

    let op: &str = operation.as_str().trim();
    // println!("What is op: {}.", op);

    match op {
        "+" => println!("Result: {}", first_no + second_no),
        "-" => println!("Result: {}", first_no - second_no),
        "*" => println!("Result: {}", first_no * second_no),
        "/" => println!("Result: {}", first_no / second_no),
        &_ => println!("{}", op),
    }
}
