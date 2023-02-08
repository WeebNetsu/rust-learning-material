fn main(){
    // there are 2 types of strings in rust
    // primitive strings -> fixed length, immutable
    // owned strings -> heap allocated, mutable
    
    // primitive by default, cannot be changed once set
    let primitive = "Hello";
    println!("{}", primitive);
    
    // owned string, can be changed, need to use below syntax
    // you also need to mark the variable as mutable if you want to change it later
    let mut owned = String::from("World");
    println!("{}", owned);

    // get string length
    println!("{}", primitive.len());
    println!("{}", owned.len());

    owned.push('!'); // add a character to the end of the string
    println!("{}", owned);

    owned.push_str(" I am cool"); // add a string to the end of the string
    println!("{}", owned);

    let my_str = "";
    // is empty checks if a string is empty
    println!("{}", my_str.is_empty());
    println!("{}", owned.is_empty());

    println!("{}", owned.contains("cool")); // checks if a string contains a certain string
    // replace a word in a string
    println!("{}", owned.replace("cool", "awesome"));

    let str_num = "45";
    // note: 45.3 cannot be converted to int from a string, since it would be a float... parse() would usually throw an error then
    let num: i32 = str_num.parse().unwrap_or(0); // this will cast string to int, if it fails, will default to 0
    let num2: f32 = str_num.parse().unwrap(); // this will cast string to float, if it fails, it will throw an error
    println!("int {} and float {}", num, num2);

    // .chars() will return an iterator, .next() will iterate
    // therefor the below will return the first letter (character) in the string
    let str_char = str_num.chars().next().unwrap();
    println!("char {}", str_char);
}