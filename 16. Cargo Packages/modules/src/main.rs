// find rust cargo crates https://crates.io/crates/colorful
// a crate is a module
// to install a crate, go to the Cargo.toml file and add
// crate = "version" (ie. colorful = "0.2.1") to the dependencies

// this will allow you to use the installed crate
extern crate colorful;

use colorful::Color;
use colorful::Colorful;

fn main() {
    // we can now use the crate as we need it
    println!("{}", "Red Text!".gradient(Color::Red));
    println!("{}", "Green Text!".gradient(Color::Green));
    println!("{}", "Yellow Text!".gradient(Color::Yellow));
    println!("{}", "Blue Text!".gradient(Color::Blue));
}
