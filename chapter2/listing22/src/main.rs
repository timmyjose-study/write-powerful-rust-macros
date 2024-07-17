use crate::greeting::base_greeting_fn;

#[macro_use]
mod greeting;

fn main() {
    let greet = greeting!("Bob", "Hola");
    println!("{greet}");

    let greet_with_default = greeting!("Dave");
    println!("{greet_with_default}");
}
