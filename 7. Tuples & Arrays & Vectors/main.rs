fn main(){
    // tuples holds values of different types
    // below we create a tuple with 5 items, name, age, married, weight, faviroute language
    let mike: (&str, i8, bool, f32, &str) = ("Mike", 23, true, 89.67, "Rust");

    println!("{} is {} years old. They weigh {}kg and loves to code in {}. Married: {}", mike.0, mike.1, mike.3, mike.4, mike.2);

    // an array allows you to store multiple values of the same type
    // you specify the type and the size of the array
    let friends: [&str; 3] = ["John", "Jane", "Jim"];

    // you can access an item in the array using the index
    // notes, index starts at 0, like in most languages
    println!("{}", friends[0]);
    // display all items in array with debug trait
    println!("{:?}", friends);

    // if you want to change the values in the array, you need to specify mut
    let mut values: [i32; 5] = [1, 2, 3, 4, 5];
    values[0] = 10; // you can modify an item in the array
    values[4] = 20;
    println!("{:?}", values);

    // get the length of the array
    println!("{}", values.len());

    // get specifc parts of array, includes index 1 and up to (not including) 3
    let slice: &[i32] = &values[1..3];
    println!("{:?}", slice);

    // vector is an array, but it does not have a fixed size
    let mut enemies: Vec<&str> = vec!["Mike", "Luke", "Nick"];
    // all array methods are available for vectors, so all above is still valid with vectors
    enemies[1] = "Mathias";
    println!("{:?}", enemies);

    // you can add items to the end of the vecor with push
    enemies.push("Veronica");
    println!("{:?}", enemies);

    enemies.pop(); // remove last item in vector
    println!("{:?}", enemies);
}