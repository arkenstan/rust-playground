// Vectos - Resizeable Vectors

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Re-assign Value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // Add on to Vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  //Get single value
  println!("Single Value: {}", numbers[0]);

  // Get Vector length
  println!("Vector length: {}", numbers.len());

  // Vectors are stack allocated: Size of Vector on memory
  println!("Vector Occupies {} bytes", std::mem::size_of_val(&numbers));
  println!("Vector Occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate value
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}
