fn main() {
    say_hello();
    say_hello_to("Mike");

    // we can store the return value in a variable
    let x = add(5, 6);
    println!("{}", x);

    println!("{}", super_add(900, 200));

    // we can print out the value directly
    println!("{:?}", add_and_subtract(5, 6));

    // closure (anonymous function)
    let get_larger = |x: i32, y: i32| if x > y { x } else { y };
    println!("{}", get_larger(5, 6));
}

// basic function
fn say_hello() {
    println!("Hello!");
}

// take in an argument
fn say_hello_to(name: &str) {
    if name.len() > 5 {
        println!("Hello {}!", name);
    } else {
        println!("Hello {}, you have a short name!", name);
    }
}

// funcName(arg: type) -> returnType
fn add(x: i32, y: i32) -> i32 {
    // if nothing is happening in the function you can just return (without a return lol)
    // this is called implicit returning, when, no semicolon and no return given
    x + y
}

fn super_add(x: i32, y: i32) -> i32 {
    // implicit returns can also be used on a larger scale
    if x == 10 {
        x * y
    }else if y > 100 {
        x - y
    }else {
        x + y
    }
}

// more complex function
fn add_and_subtract(x: i32, y: i32) -> (i32, i32) {
    if x > y {
        println!("{} is greater than {}", x, y);
    }
    
    // below could also be implicit, but I like the return keyword
    return (x + y, x - y);
}
