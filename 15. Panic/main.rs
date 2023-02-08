fn main() {
    // similar to rust, panicing allows us to "throw an error"
    // and stop the program execution
    let pan = false;

    if pan {
        panic_me();
    } else {
        println!("Did not panic ;)");
    }

    println!("Program finished");

    // sometimes you want to be able to recover from a panic
    // aka you don't want to stop the program execution
    let res = match safe_panic(true) {
        Ok(resp) => resp,
        // I could panic here if the error is severe enough
        // but in this case I just return the result and store
        // it in the variable
        Err(error) => error
    };

    println!("{}", res);

    println!("Program still safe");
}

fn panic_me(){
    println!("Think I'm going to panic!");

    // will throw a custom error message
    panic!("SOMETHING HAPPENED, CUSTOM PANIC ERROR MESSAGE");
}

// Result has 2 values in the enum, Ok and Err
// Err is used when something goes wrong
// you can specify the type of Ok and Err, in this case
// the type of both is String
fn safe_panic(do_err: bool) -> Result<String, String>{
    // returning a result instead of a panic is a good practice
    // because then you can decide when to panic when an error
    // occurs and when not to when calling the function
    if do_err {
        // return an error without crashing the program
        Err(String::from("Error, panic thrown"))
    } else {
        Ok(String::from("Success, no panic!"))
    }
}