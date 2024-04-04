fn main() {
    // There are scalar and compound type in Rust
    // scalar is almost same as other language, so let's take a look at compound type

    // 1. Tuple : group a number of values with a variety of types into one compound type
    // fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1); // type optional

    // destructuring : use pattern matching to get values out of a tuple
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // direct aceess tuple element
    let five_hundred = tup.0;

    println!("The value of five_hundred is: {five_hundred}");

    // unit : tuple without any values
    let unit = ();

    // 2. Array : group of values with the same type
    // fixed length
    // useful when you want to allocate your data on the stack, rather than heap -> why?
    let five_num_arr: [i32; 5] = [1, 2, 3, 4, 5]; // type optional : but should add size when annotated
    let third = five_num_arr[2];
    // let sixth = five_num_arr[5];
    println!("The value of third element of five_num_arr is: {third}");

    // array with same value. same as [3,3,3,3,3]
    let five_threes = [3; 5];
    let mut i = 0;
    loop {
        let three = five_threes[i];
        println!("The value of {i} index of [3;5] is {three}");
        i = i + 1;
        if (i == 5) {
            break;
        }
    }
}
