extern crate gurnal;

use std::io;

pub fn main() {
  let mut email  = String::new();

  println!("Enter an email address you'd like to parse: ");
  io::stdin().read_line(&mut email).expect("Failed to read email");

  // FIXME:  How do you check for nil input from STDIN!?
  if email.is_empty() {
    println!("The email was empty!");
  } else {
    println!("The email was NOT empty!");
    gurnal::parse(&email);
  }
}