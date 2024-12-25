use std::io;

fn main() {

  println!("Guess the number!");
  println!("Please input the number!");

  let mut guess_number = String::new();

  io::stdin()
    .read_line(&mut guess_number)
    .expect("Failed to read the lined");

  println!("Your guess number is {}", guess_number)

}