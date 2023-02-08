/* 
    Usually you would initialise a rust project using cargo (it will also intialise a Git repo)
    Cargo is a build system for Rust, to create a new project, run:
    cargo new project_name

    To create a new project in the current folder (not creating a new folder):
    cargo init

    To compile the project, run:
    cargo build
    it will compile to target/debug/cargo

    To run the project, run:
    cargo run

    Cargo.toml is like the package.json in JS or the go.mod in Go
    Cargo.lock is like the package-lock.json in JS or the go.sum in Go
*/

fn main() {
    println!("Hello, world!");
}
