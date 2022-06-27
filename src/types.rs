/*

   Primitive Types :
       Integers: [u8, i8,u16, i16,u32, i32,u64, i64, u128, i128] ( Number of bits they take in memory)

       Floats : [f32, f64].

       Boolean: bool

       Characters: [char]

       Tuples

       Arrays

       Rust is a statically type language, wich means that it mus know the toyes of all variables at compile time,
       however, the compiler can usuarlly infer what type we want to use based on the value and how we use it.
*/

pub fn run() {
    // Default is "i32"

    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45454545241342545;

    // Find max size

    println!("Max i32:{}", std::i32::MAX);
    println!("Max i64:{}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // From expression

    let x_is_greater_than = z > 50;

    let a1 = '🫔'; 
    

    println!("{:?}", (x, y, z, is_active,x_is_greater_than,a1));
}
