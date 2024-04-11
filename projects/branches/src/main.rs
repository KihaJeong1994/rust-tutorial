fn main() {
    // 1. if expression
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;

    // since if is an expression, it evaluates to value, so it can be assigned to the variable.
    let number2 = if condition { 5 } else { 6 };
    println!("The value of number2 is {number2}");

    // 2. loop expression
    // break [value] : you can return value after loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Loop label : disambiguate between multiple loops
    // begin with a single quote
    // ex. 'counting_up: loop{}
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // 3. while
    let mut counter = 3;

    while counter > 0 {
        println!("counter : {counter}");
        counter -= 1;
    }

    // 4. for
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("{element}");
    }

    // use Range to countdown
    // 1..4 : 1<=x<4
    // rev() : reverse the range
    for i in (1..4).rev() {
        println!("the number is {i}");
    }

    // practice
    // Convert temperatures between Fahrenheit and Celsius.
    let fahrenheit = 32.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit} fahrenheit is {celsius} celsius");

    let fahrenheit = 212.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit} fahrenheit is {celsius} celsius");

    let celsius = 0.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius} celsius is {fahrenheit} fahrenheit");

    let celsius = 100.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius} celsius is {fahrenheit} fahrenheit");

    // Generate the nth Fibonacci number.
    for i in 1..=10 {
        let ith_fibonacci = fibonacci(i);
        println!("the {i}th fibonacci number is {ith_fibonacci} ")
    }
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (9.0 * celsius / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
