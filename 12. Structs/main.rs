// structs are like the Classes of Rust
// structs are used to create custom data types

// --- PART 1 ---
// create basic struct
struct User {
    // note we can't use &str here
    username: String,
    email: String,
    age: u8,
}

// --- PART 2 ---
// tuple struct
struct Person(String, String, u8);

// --- PART 3 ---
// we cab do the below to simulate OOP in Rust
struct Animal {
    name: String,
    age: u8,
    dog: bool,
}

// impl allows us to create methods for structs
impl Animal {
    fn new(name: &str, age: u8, dog: bool) -> Animal {
        Animal {
            // to_string converts a &str to a String
            name: name.to_string(),
            age, // since they have the same name, we don't have to say age: age
            dog,
        }
    }

    // &self is like "this" in other languages
    // it is a reference to the struct
    fn to_string(&self) -> String {
        // format will return a String
        format!("{} is {} years old and is a {}", self.name, self.age, if self.dog { "dog" } else { "cat" })
    }

    // since we want to change a value in the struct, we
    // use mut to say it should be mutable
    // NOTE when you create the struct, you need to use "let mut"
    // since it should be mutable!
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

fn main(){
    // --- PART 1 ---
    let mut user1 = User {
        username: String::from("joe"),
        email: String::from("joe@gmail.com"),
        age: 20,
    };

    // we can access the struct's fields using dot notation
    println!("{} has the email {}, and is {} years old", user1.username, user1.email, user1.age);

    // you can change a value in a struct like this
    user1.email = String::from("joe@yahoo.com");
    println!("{} has the email {}, and is {} years old", user1.username, user1.email, user1.age);

    // --- PART 2 ---
    // in a tuple struct, the values don't have to be named
    let mut person1 = Person(String::from("Max"), String::from("max@gmail.com"), 39);
    println!("{} has the email {}, and is {} years old", person1.0, person1.1, person1.2);

    // we can change the value of a tuple struct's fields
    person1.2 = 100;
    println!("{} has the email {}, and is {} years old", person1.0, person1.1, person1.2);

    // --- PART 3 ---
    // we can now use the struct's methods
    let mut animal1 = Animal::new("Fluffy", 10, true);
    println!("{}", animal1.to_string());

    animal1.set_name("Poofy");
    println!("{}", animal1.to_string());
}