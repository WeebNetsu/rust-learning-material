// this part is for the pointers section of this chapter
// without box pointers
// enum List {
//     // this is a recursive list, that stores a integer and the next element in the list
//     // Rust wants to store this in the stack, but since it does not know it's size at compile time
//     // it will throw an error
//     Cons(i8, List),
//     Nil,
// };

// with box pointers
enum List {
    // the box pointer will allow us to store this data withouth having to know the size
    // it will be stored on the heap
    Cons(i8, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main(){
    // --- REFERENCES ---
    // you have 2 types, primitive and non-primitive
    // when reassigning a primitive type, the original value still exists
    // when assigning a non-primitive type, the original value is no longer available

    // primitive type
    let ages = [10, 20, 30];
    let ages_copy = ages;

    // because arrays are primitive types, they are copied, so the original variable is still available
    println!("{:?}", (ages, ages_copy));

    let nums = vec![1, 2, 3];
    let nums_copy = nums;

    // because vectors are non-primitive types, they are moved, so the original variable is no longer available
    // println!("{:?}", (nums, nums_copy));
    //* the above line will cause an error, because the original variable is no longer available

    // & = reference
    // when you use a reference, you're borrowing the value, so the original variable is still available if you reference it again
    let nums_copy_2 = &nums_copy;
    println!("{:?}", (&nums_copy, nums_copy_2));

    // --- POINTERS ---
    /* 
        "A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data. The most common kind of pointer in Rust is a reference. References are indicated by the & symbol and borrow the value they point to. They don’t have any special capabilities other than referring to data."

        "Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities."

        Rust uses the concept of ownership and borrowing, the difference between a reference and smart pointer is that references are pointers that only "borrow" data, where smart pointers instead own the data they point to.

        Box<T> - Point to data on the Heap

        Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.
    
        Some situations they're useful in:
        - When you have a type whose size can’t be known at compile time
        - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
        - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    */

    // below will store the value 100 on the heap
    let a: Box<i32> = Box::new(100);
    // you can still use it normally
    println!("{:?}", a);

    // without box pointer
    // let list = Cons(2, Cons(4, Cons(6, Nil)));

    // with box pointer, code will now compile
    let _b = Cons(2, Box::new(Cons(4, Box::new(Cons(6, Box::new(Nil))))));

    // TO BE CONTINUES
    // ! https://doc.rust-lang.org/book/ch15-02-deref.html
    let c = 10;
    let d = &c; // reference c, pointing to memory address
    println!("{:?}", c);
    // we use *d to dereference the reference, this will return the integer
    // without it we would get a reference to c, i32 != &i32
    println!("{:?}", *d);
}