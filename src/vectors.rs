// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Reassign values
    numbers[2] = 21;

    println!("{:?}", numbers);

    // Add to vector

    numbers.push(45);
    numbers.push(6);

    // Pop last value

    numbers.pop();

    // Get single value

    println!("Single value : {}", numbers[0]);

    // Get Length
    println!("{}", numbers.len());

    // Vector are stack allocated
    println!("This vector uses {} bytes", mem::size_of_val(&numbers));

    // Get Slice

    let slice: &[i32] = &numbers[0..2];

    println!("Slice : {:?}", slice);

    // Loop throught vector values

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate vector values

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Number: {:?}", numbers);
}
