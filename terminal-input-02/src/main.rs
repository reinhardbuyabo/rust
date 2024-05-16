use std::io;

fn main() {
    // 1. Prompt for input
    println!("Enter a number!");
    let mut input: String = String::new(); // assuming that input from command line is always a string
    io::stdin().read_line(&mut input).expect("Failed to read line. Process exited with status code 1");
    let input: u32 = input.trim().parse().expect("Please enter a valid number: ");

    for i in 0..=input {
        println!("{}", i);
    }
}
