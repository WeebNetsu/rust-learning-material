fn main(){
    // basic output
    println!("Hello, world!");
    // same as above, but no new line at the end
    print!("Hello, world!");

    // you can also use escape characters
    println!("\n\tHello World\\Earth!");

    // to display a number, you have to use {}
    println!("{}", 1);

    // to replace anything in a string, use {}
    println!("Hello {}! You are {} years old.", "Jack!", 18);

    // positional string formatting
    // 0 will be the 0th positional argument (Megan) and 1 will be the 1st positional argument (dreams)
    println!("I love {0} and {0} is the women of my {1}! {0}...", "Megan", "dreams");

    // you can also used named parameters
    println!("My name is {name} and I like {food}", name="Steve", food="pizza");

    // traits, convert value to selected type
    // b - binary, x - hex, o - octal
    println!("Binary converted: {:b}", 100);

    // debug trait, used to display debug data
    println!("INFO: {:?}", (200, "Main Database Connected"));
}