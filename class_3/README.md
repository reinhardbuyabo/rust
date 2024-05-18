# Modules

- Whenever you see the 2 colons, it means we're accessing sth deeply nested inside that folder.
- `mod <file>`
- `use <function>`
- Group your files into folders.
- Introduce a `mod.rs` file in the folder that hosts the modules.
- A file may be inaccessible because it was not specified in the `mod.rs`.

- In mod.rs, add function signatures. (public)

# Structs
- Pass in parameters as types in function signatures to restrict functionality to certain.
- We want to create functionality that is specific to User struct.
- Probably to walk, eat, and has a birthday.

# Exercises
- [ ] Create a function that returns a slice
```rs
fn return_slice() -> [i32] {
    [1, 2, 3]
}
```

- An `str` can be a primitive data types.
- A `String` can be a vector of ...
- Strings can be formed by characters that are not exems

- [ ] **Fibonacci** Program
- What is a fibonacci?
- Create a program where the third number is a result of the preceding two numbers. 
```rs
fn main() {
// Define the number of terms in the Fibonacci sequence you want to generate
let n = 10; // You can change this to any positive integer

// Call the function to print the Fibonacci sequence
print_fibonacci(n);
}

fn print_fibonacci(n: usize) {
// Initialize the first two Fibonacci numbers
let mut a = 0;
let mut b = 1;

// Print the first Fibonacci number
if n >= 1 {
println!("{}", a);
}

// Print the second Fibonacci number
if n >= 2 {
println!("{}", b);
}

// Generate and print the rest of the Fibonacci sequence
for _ in 2..n {
let next = a + b;
println!("{}", next);
a = b;
b = next;
}
}

```

- [ ] **Recursion**
- Recursion runs forever.
```rs
fn recursion(intgr: i32) {
    println!("number: {intgr}");
    recursion(10);
}
```
- If you don't want it to run forever, you give it a condition, embed the function call within the condition:
```rs
fn recursion(intgr: i32) {
    println!("number: {intgr}");
    
    if intgr <= 10 {
        println!("{}", intgr);
        intgr += 1; // increment
        recursion(10);
    }
}
```

- [ ] Using your knowledge of **structs** create characters in a game. Characters should have a name, score, and a level.

- If there is any function that takes a mutable reference to self, consider instatiating a variable as mutable.

- [ ] Create a program that takes in 2 inputs in the terminal, divides the second input by the first. Divide by zero, integer overflow

- [ ] Create a program taking in an input that. 
    - [ ] Check for Divide By Zero
    - [ ] Check for Overflow
    - [ ] Check for Letters

- Real life solution: Text sth to a certain code, and then you're told if you've won or not.
- Catch: Never accounted for emojis.
- For people who input emojis, the service crashes.
- Capturing None ASCII characters. (Characters)