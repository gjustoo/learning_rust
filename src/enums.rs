// Enums de toda la vida

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Moving up"),
        Movement::Down => println!("Moving down"),
        Movement::Right => println!("Moving right"),
        Movement::Left => println!("Moving left"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Left;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
