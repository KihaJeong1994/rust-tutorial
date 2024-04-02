use rand::Rng; // items from traits can only be used if the trait is in scope
use std::cmp::Ordering;
use std::io; // bring io library in standard library into scope

fn main() {
    println!("Guess the number!");

    // range expression
    // 1<=x<=100 -> 1..=100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
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

        // Shadowing : reuse the variable name to convert from one type to another
        // u32 : unsigned 32 bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ : catchall value
            Err(_) => {
                println!("Type the number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // match : decide what to do next based on the return value
        // match expression is made up of arms
        // arm : pattern + code that should be run if the value given to match fits that arm's pattern
        // match expression ends after the first successful match -> different with switch case that needs break
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
