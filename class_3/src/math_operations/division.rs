use std::io;

pub fn divide() {
    // Reading first no.
    println!("Enter first no.: ");
    let mut first_no: String = String::new();
    io::stdin().read_line(&mut first_no).expect("Read failed!");
    // println!("{}", first_no);

    // Second Number:
    println!("Enter second no.: ");
    let mut second_no: String = String::new();
    io::stdin().read_line(&mut second_no).expect("Read failed!");
    // println!("{}", second_no);

    // Type Conversions:
    let first_no: i32 = first_no.trim().parse().expect("Invalid First Number");
    let second_no: i32 = second_no.trim().parse().expect("Invalid Second Number");

    // Dividing
    if second_no == 0 {
        panic!("Zero Division Error!"); // crashes
    } else {
        println!("Result: {}", first_no / second_no);
    }

    // Integer Overflow? Trillion (Simulate by running with very large numbers, or very small numbers) -> Smallest i32: -2,147,483,648 | Largest i32: 2,147,483,647.
    
}