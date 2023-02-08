// by default everything is private and cannot be accessed by outside files
// pub -> make function public and available to other modules
pub fn say_hello(){
    println!("Hello Jack!");
}

// you can also export other things like struct, if you do
// a struct has to have a pub keyword so it is public to outside files
// and all it's content that you want public should too
pub struct Animal {
    pub name: String,
    pub age: u8,
    pub dog: bool,
}