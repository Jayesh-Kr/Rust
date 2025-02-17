use std::io;

fn main() {
    println!("Welcome to the guessing game");
    println!("Guess the number");
    println!("Please input a number");

    let mut number = String::new();
    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");

    // println!("Your entered number is : {}", number);
    println!("Your entered number is : {number}");
}