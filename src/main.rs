use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The Secret Number: {}", secret_number);

    loop {
        println!("insert your number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your Guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too Big".purple()),
            Ordering::Equal => {
                println!("{}", "You Win".green());
                break;
            }
        }
    }
  
}
