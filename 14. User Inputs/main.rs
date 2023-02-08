use std::env; // this allows us to remove some namespaces from the code

fn main(){
    // --- getting input from app execution
    // get user input on running executable
    // ./main hello world
    // since we used use std::env, the below has been shortened from std::env::args().collect()
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    // get the first argument
    let command = args[1].clone();
    println!("{}", command);

    if command == "power" {
        println!("Your power level is over 9000!!");
    }

    // --- getting input from user
    // create an empty variable to keep the input
    let mut input = String::new();

    println!("Enter your name: ");
    
    // this will get the user input, it might return an error
    // if something goes wrong on getting input
    // this will store the value inside the input variable
    match std::io::stdin().read_line(&mut input) {
        // if no error occurs
        Ok(_) => {
            println!("Your name is: {}", input);
        },
        // if an error occurs
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}