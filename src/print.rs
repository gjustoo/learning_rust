pub fn run() {
    println!("hello from the print.rs file");

    // String formatting
    println!("The number is : {} and then goes {}", 1, 2);

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    // Named Arguments

    println!(
        "{name} likes to play {activity}",
        name = "Pepito",
        activity = "League Of Legends"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic math

    println!("10 + 10 = {result}", result = 10 + 10);
}
