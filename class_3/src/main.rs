mod animals;
mod counties;

// Book Struct
mod book;

// Rectangle Struct
mod rectangle;

use animals::reptiles;
use counties::print_counties;

use rust_math::num::sqrt;

// Using/importing Book Struct
use book::Book;

// Using/import Rectangle Struct
use rectangle::Rectangle;

struct User {
    name: String,
    age: i32,
    id_number: i32,
}

struct Car {
    name: String,
    number: i32,
    model: String,
}

impl User {
    pub fn walk(&self) {
        println!("User {} is walking ", &self.name);
    }
}

fn main() {
    println!("Hello, world!");

    reptiles::print_reptiles(); // Printing from 
    print_counties();

    println!("{}", sqrt(4.0));

    let kenn: User = User {
        name: String::from("Kenn"),
        age: 30,
        id_number: 40902024,
    };

    println!("Hello {}", kenn.name);
    // walk(kenn.name);
    kenn.walk();

    // 3. Book Struct
    let new_book: Book = Book{
        name: String::from("The Alchemist"),
        status: true,
        category: String::from("Fiction"),
    };

    println!("The book is available: {}", new_book.is_available());

    // Rectangle Area:

    // Instantiating Rectangle
    let rect1: Rectangle = Rectangle::new(10.0, 5.0);
    println!("Area: {}", rect1.area()); // Area of rectangle
    println!("Perimeter: {}", rect1.perimeter()); // Perimeter of rectangle

}

// fn walk(name: String) {
//     println!("Hello from walk: {}", name);
// }
