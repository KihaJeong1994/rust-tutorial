fn main() {
    // 1. Variable
    // use 'let' keyword to declare variable and assign value to it
    let x = 6;
    println!("The value of x is : {x}");

    // compile error since x is immutable by default
    // x = 5;
    // println!("The value of x is : {x}");

    // add 'mut' keyword to make variable mutable
    let mut y = 7;
    println!("The value of y is : {y}");
    y = 8;
    println!("The value of y is : {y}");

    // 2. Constant
    // use 'const' keyword
    // type of the value must be annotated
    // can be declared in any scope
    // cannot use 'mut' keyword
    // naming convention : uppercase with underscores between words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours is {THREE_HOURS_IN_SECONDS} in seconds");

    // 3. Shadowing
    // declare a new variable with the same name as a previous variable
    // the first variable is shadowed by the second
    // the second variable overshadows the first, until either it itself is shadowed or the scope ends
    // shadowing != mut , rather shadowing = declare 'new' variable with the same name -> possible to change the type of the value
    let z = 5;

    let z = z + 1;

    // inner scope created with the curly brakets
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    // shadowing ends since the scope ends
    println!("The value of z is: {z}");

    let spaces = "   "; // type : &str
    let spaces = spaces.len(); // type : usize

    println!("There are {spaces} spaces.");
}
