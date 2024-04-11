fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    expression_example();
    let five = five();
    let six = plus_one(five);
    println!("The result of six is {six}");
}

// fn : declare function
// naming convention : snake case
fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statement : perform some action and do not return value
// ex. declare function, assign value to variable
// in C and Ruby, the assignment returns the value of the assignment
fn statement_example() {
    let x = 6;
    // let y = (let z = 6); //compile error
}

// Expression : evaluate to a value
// ex. calling a function, calling a macro, a new scope block created with curly brackets
// expression do not include semicolon
fn expression_example() {
    let y = {
        let x = 3;
        // the blocks of the code {...}  evaluate to the last expression in them
        x + 1 // no semicolon. If you add semicolon, it will be treated as Statement, and does not evaluate to value, but unit type ()
    };
    println!("The value of y is {y}");
}

// declare return type with an arror '->'
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
