// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Reassign values

    numbers[2] = 21;

    println!("{:?}", numbers);

    // Get single value

    println!("Single value : {}", numbers[0]);

    // Get Length
    println!("{}", numbers.len());

    // Arrays are stack allocated
    println!("This array uses {} bytes", mem::size_of_val(&numbers));

    // Get Slice

    let slice: &[i32] = &numbers[0..2];

    println!("Slice : {:?}", slice);
}
