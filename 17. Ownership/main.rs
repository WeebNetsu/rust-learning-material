fn main(){
    /* 
        Rust does not have a garbage collector, and it also doesn't force you to
        clean up your program memory yourself, like C/C++, instead we have ownership
        and borrowing.

        Ownership:
            - Ownership is the process of giving a variable to a function, and then
            giving the function ownership of the variable.
            - Ownership is a way to prevent data from being accidentally deleted, 
            overwritten, modified, accessed, copied, moved, cloned, borrowed, used.

        Borrowing:
            - Borrowing is the process of taking ownership of a variable from another
            function, and then giving the variable back to the original function.
            - When the original function is done with the variable, it gives back
            ownership.
            - Borrowing is a way to prevent data from being accidentally deleted, 
            overwritten, modified, accessed, copied, moved, cloned, borrowed, used.
        
        When you use ownership, you have to be careful about what you give to a function.
        You can't give a variable to a function that doesn't own it, already owns it, 
        already borrowed it

        --- Ownership rules ---
        1. Each value in rust has a variable that is called its owner. 
            (1 variable = 1 owner, a variable cannot have multiple owners at a time)
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value will be dropped.

        The stack:
            - The stack is a first-in, first-out structure, meaning that the first item
            - The stack is a LIFO structure, meaning the last item in the stack is the
            first item out of the stack.
            - An item in the stack will have a fixed size, cannot be resized.

        The heap:
            - it's a more dynamic way to store data than on the heap, since the data
            stored on the heap, it can be resized.
            - storing data on the heap is a lot more expensive, resource-wise
    */

    {
        let _word = String::from("hello");
        // word is now owned by this scope
        // once the scope end (}) is reached, word is dropped (deleted)
    }

    // this will be stored on the stack
    let a = 10;
    // this will create a copy of a, and store it on the stack
    let b = a;
    println!("a: {}, b: {}", a, b);

    // same will be applied here
    let c: &str = "hello";
    let d = c;
    println!("c: {}, d: {}", c, d);

    // the below however will be stored on the heap
    let name = String::from("Jack");
    // since storing the data on the heap is expensive, rust will not create
    // a new copy of the data, like in the stack, but instead it will delete
    // the old owner and create a new one.
    let name2 = name;

    // usually in C if you create an item on the heap, you have to delete it manually
    // in rust it will be done for you, when the variable goes out of scope

    // this will cause an error, because name was now deleted
    // since we are storing the name in name2
    // println!("name: {}, name2: {}", name, name2);

    // to instead create a copy of the data, we can use the clone method
    let name3 = name2.clone();
    // this will create a new copy of the data, and store it on the heap
    println!("name2: {}, name3: {}", name2, name3);

    let name_size = get_size(name3);
    // below will cause an error, because the above function got ownership
    // of name3 when we passed it in, and it then retuned the ownership
    // to the name5 variable, so name3 is now deleted
    // println!("name3: {}, name size: {}", name3, name_size);

    let name_size2 = get_size2(&name2);
    // this will create a new copy of the data, and store it on the heap,
    // since we're passing in by reference and not by value
    // so ownership is not transferred
    println!("size 1: {}, size 2: {}", name_size, name_size2);
}

// this will borrow the data, and return the size of the data
// BUT it will give the ownership of the data to whichever
// variable calls this function
fn get_size(s: String) -> usize {
    s.len()
}

// this will get a reference to the string
// unlike the above function, this will not give
// the ownership away, but instead it will borrow the data
fn get_size2(s: &String) -> usize {
    s.len()
}