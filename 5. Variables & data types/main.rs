fn main(){
    // NOTE: in Rust, variables are constant by default
    let name = "Steve"; // create a variable

    // remember! you have to use {} to print the variable
    println!("My name is {}", name);

    // Below will throw an error, because the variable cannot be changed!
    // name = "Max";

    let mut age = 19; // create a mutable variable
    println!("My name is {name} and I am {age} years old", name=name, age=age);

    // we can change age, because it is mutable
    age = 20;
    println!("Next month is my birthday! I will then be {age} years old", age=age);

    // assign multiple variables at once
    let (var_one, var_two) = (90, "Kaboom");

    /* 
        Rust data types:
        (u = unsigned)
        - integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
        - floats: f32, f64
        - bool: true, false
        - char
        - strings: &str, String
        - tuples
        - arrays
    */

    // you can also specify types
    // we use _ to say that we don't care about the value, so the compiler doesn't complain
    let _x: bool = false;
    let _y: i32 = 10;

    // this is a character, it is inferred to be a char, we don't have to specify the type
    // char is only ONE character, unlike a string that can be multiple characters
    let _z = 'c'; // note that chars use single quotes

    // const is another way to declare constant variables, the difference is
    // that const can't use "mut" to make it mutable
    const MAX_POINTS: u32 = 100000;

    // Math Basics: +, -, *, /, %
    let a = 5 + 9;
    let b = 10 * a;
    let c = 15 % 4;
    // to overwrite when equations should be done (so plus before times)
    // we can use () 
    let d = (10 + 9) * 4 - (a / c);
    println!("a: {}\nb: {}\nc: {}\nd: {}", a, b, c, d);
}