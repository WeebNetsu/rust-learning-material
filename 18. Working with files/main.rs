// use std::io::Write;

fn main() {
    // --- Read file
    // we use .expect to say what should be said when an error occurs
    let file_content = std::fs::read_to_string("readme.txt").expect("Could not read the file");

    println!("{}", file_content);

    // -- write to file
    std::fs::write("myfile.txt", "Hello world!\nSee you soon!").expect("Could not write to file");

    // -- append to file
    // with openoptions we can specify what should be done when working with the file
    // so we say we should be able to append and create a file if it doesn't exist
    let mut file = std::fs::OpenOptions::new().append(true).create(true).open("myfile.txt").expect("Could not open file");

    std::io::Write::write_all(&mut file, b"\nThis has been appended").expect("Could not write to file");

    // note: if you did: "use std::io::Write;"
    // then you can do below instead of the above
    // file.write_all(b"\nThis has been appended").expect("Could not append to file");
}