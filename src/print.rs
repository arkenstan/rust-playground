pub fn run(){
  // Print to console
  println!("Hello from print.rs file");

  // Basic Formatting
  println!("{} is from {}", "Brad", "Mass");

  // Positional arguments
  println!("{0} is from {1} and {0} like to {2}", "Brad", "Mass", "Code" );

  // Named arguments
  println!("{name} like to play {activities}", name="John", activities="Baseball");

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x}, Octal: {:o}", 10,10,10);

  // Placeholder for Debug trait
  println!("{:?}", (12, true, "Hello"));

  // Basic Math
  println!("10+10 = {}", 10+10)

}