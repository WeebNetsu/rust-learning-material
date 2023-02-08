enum Pet {
    DOG,
    CAT,
    SNAKE,
    BEARDED_DRAGON,
}

fn buy_pet(p: Pet){
    /* if (Pet::DOG == p) {
        println!("You bought a dog!");
    }else if (Pet::CAT == p){
        println!("You bought a cat!");
    }else if (Pet::SNAKE == p){
        println!("You bought a snake!");
    }else if (Pet::BEARDED_DRAGON == p){
        println!("You bought a bearded dragon!");
    }else{
        println!("You bought something else!");
    } */

    // below is the same as above, but you can think of it as a switch statement
    match p {
        Pet::DOG => println!("You bought a dog!"),
        Pet::CAT => println!("You bought a cat!"),
        Pet::SNAKE => println!("You bought a snake!"),
        Pet::BEARDED_DRAGON => println!("You bought a bearded dragon!"),
    }

    let x = 10;
    // this is a basic example of match
    match x {
        2 => println!("You entered 2!"),
        1 | 3 | 5 => println!("You entered 1 or 3 or 5!"),
        _ => println!("You didn't enter 1, 2, 3 or 5!"),
    }
}

fn main() {
    let pet = Pet::SNAKE;

    buy_pet(pet);
}