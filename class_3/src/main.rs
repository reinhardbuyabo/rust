use std::collections::HashMap;

// CHARACTER GAME
mod character; // importing

// DIVISION
mod math_operations;

// use character::Character;
// use math_operations::division::divide; // FOLDER/FILE/FUNCTION

fn main() {
    #[derive(Debug)]
    struct Institution {
        offices: Vec<String>,
        rooms: HashMap<String, i32>
    }

    impl Institution {
        fn new() -> Self {
            Self {
                offices: Vec::new(),
                rooms: HashMap::new(),
            }
        }

        fn add_offices(&mut self, office: String){
            self.offices.push(office);
        }

        fn add_room(&mut self, room_name: String, room_number: i32) {
            self.rooms.insert(room_name.clone(), room_number);
            self.rooms.entry(room_name.clone()).or_insert(room_number);
        }
    }

    let mut kencom: Institution = Institution::new();
    kencom.add_offices(String::from("Manager's")); // this function mutates the instance, that's why its first parameter is a mutable reference to self

    kencom.add_room("Room 1".to_string(), 1);

    println!("{:?}", kencom);
}


// mod animals;
// mod counties;

// // Book Struct
// mod book;

// // Rectangle Struct
// mod rectangle;

// // Recursion
// mod one_hundred;

// use animals::reptiles;
// use counties::print_counties;

// use rust_math::num::sqrt;

// // Using/importing Book Struct
// use book::Book;

// // Using/import Rectangle Struct
// use rectangle::Rectangle;

// use one_hundred::recursive::recursion; // FOLDER::FILE::FUNCTION;

// struct User {
//     name: String,
//     age: i32,
//     id_number: i32,
// }

// struct Car {
//     name: String,
//     number: i32,
//     model: String,
// }

// impl User {
//     pub fn walk(&self) {
//         println!("User {} is walking ", &self.name);
//     }
// }

// fn main() {
//     // Define the number of terms in the Fibonacci sequence you want to generate
//     let n = 10; // You can change this to any positive integer
    
//     // Call the function to print the Fibonacci sequence
//     print_fibonacci(n);

//     recursion(35);
// }
    
// fn print_fibonacci(n: usize) {
//     // Initialize the first two Fibonacci numbers
//     let mut a = 0;
//     let mut b = 1;
    
//     // Print the first Fibonacci number
//     if n >= 1 {
//     println!("{}", a);
//     }
    
//     // Print the second Fibonacci number
//     if n >= 2 {
//     println!("{}", b);
//     }
    
//     // Generate and print the rest of the Fibonacci sequence
//     for _ in 2..n {
//     let next = a + b;
//     println!("{}", next);
//     a = b;
//     b = next;
//     }


// }
    

// fn main() {
//     println!("Hello, world!");

//     reptiles::print_reptiles(); // Printing from 
//     print_counties();

//     println!("{}", sqrt(4.0));

//     let kenn: User = User {
//         name: String::from("Kenn"),
//         age: 30,
//         id_number: 40902024,
//     };

//     println!("Hello {}", kenn.name);
//     // walk(kenn.name);
//     kenn.walk();

//     // 3. Book Struct
//     let new_book: Book = Book{
//         name: String::from("The Alchemist"),
//         status: true,
//         category: String::from("Fiction"),
//     };

//     println!("The book is available: {}", new_book.is_available());

//     // Rectangle Area:

//     // Instantiating Rectangle
//     let rect1: Rectangle = Rectangle::new(10.0, 5.0);
//     println!("Area: {}", rect1.area()); // Area of rectangle
//     println!("Perimeter: {}", rect1.perimeter()); // Perimeter of rectangle

//     for i in return_slice() {
//         println!("{}", i);
//     }
// }

// fn return_slice() -> [i32; 3] {
//     [1, 2, 3]
// }

// fn walk(name: String) {
//     println!("Hello from walk: {}", name);
// }
