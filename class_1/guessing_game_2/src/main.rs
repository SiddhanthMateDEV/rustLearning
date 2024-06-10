use rand::Rng;
use std::io;

#[allow(dead_code)]
#[warn(unused_variables)]
fn main() {
    println!("Guess a number \n");
    
    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess");
    
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guess: {guess}");

    println!("The secret number: {secret_number}");
}
