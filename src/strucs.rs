// Used to create custom data types.

// Traditional Struct ( Classes &)
// struct Color {
//     red: u8,
//     green: u8,
//   &  blue: u8,
// }

// Tuple Struct

// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor person

    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Creating fn

    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }


    fn set_name(&mut self, name:&str){
        self.first_name = name.to_string();
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.blue = 255;

    // println!("Color: {} {} {}", c.red, c.blue, c.green);

    // let mut c = Color(255, 0, 0);

    // c.1 = 44;

    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Gabriel", "Justo");

    println!("Person : {}", p.full_name());


    p.set_name("Beatriz");

    println!("Person: {}",p.full_name());
}
