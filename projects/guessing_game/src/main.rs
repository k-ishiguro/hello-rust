use std::io;  // import io libary from the standard library

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();  // declaring a MUTABLE variable (by default, immurable. String class object, new function) here "::" means a (class-)static method
    io::stdin().read_line(&mut guess)
        .expect("Failed to reald line"); // stdin() is a static method. read_line is a method. & is a reference pointer, $mut is a mutable pointer
    println!("You guessed: {}", guess);
}
