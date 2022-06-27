// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Gabriel";
    let mut age = 22;
    println!("My name is {name} and I am {age}", name = name, age = age);

    age = 21;

    println!("My name is {name} and I am {age}", name = name, age = age);

    // Define constants

    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple vars

    let (my_name, my_age) = ("Gabriel", 21);

    println!("{} is {}", my_name, my_age);
}
