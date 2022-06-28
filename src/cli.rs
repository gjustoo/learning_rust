use std::env;
pub fn run() {
    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);
    // First arg always exec file path
    // Args: ["target/debug/tutorial", "hello"]

    let command = args[1].clone();

    println!("Command: {}", command);

    if command.eq_ignore_ascii_case("hello") {
        println!("Hi Gabriel, how are you");
    } else if command.eq_ignore_ascii_case("status") {
        println!("Status: Active and running 100% memory usage");
    } else {
        println!("{} is not a valid command", command);
    }
}
