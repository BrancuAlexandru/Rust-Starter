use std::io;

fn main() {
  println!("Guess the foood!");

  println!("Please input your guess: ");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line.");

  let food = "pizza";

  if guess.contains(food) {
    println!("Your guess '{guess}' is correct!");
  } else {
    println!("Your guess was wrong!");
  }
}