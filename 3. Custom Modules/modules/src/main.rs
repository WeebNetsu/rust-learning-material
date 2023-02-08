// mod filename -> import module
mod custom;

fn main() {
    println!("Hello, world!");

    // use a function from the custom module
    custom::say_hello();

    let dog = custom::Animal {
        name: String::from("joe"),
        dog: true,
        age: 2,
    };

    println!("{}", dog.name);
}
