use std::io; // bring io library in standard library into scope
use rand::Rng; // items from traits can only be used if the trait is in scope

fn main() {
    println!("Guess the number!");

    // range expression
    // 1<=x<=100 -> 1..=100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}",secret_number);

    println!("Please input your guess.");

    // let : a statement to create variable
    // variables are immutable by default
    // mut : mutable
    // String::new : new is an associated function of the String type
    // associated function : a function that is implemented on a type
    let mut guess = String::new();

    // & : indicates that an argument is a reference.
    // reference is immutable by default
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
