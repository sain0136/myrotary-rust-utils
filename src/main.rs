use rand::Rng;
use std::cmp::Ordering;
use std::io;
use  colored::*;

fn main() {
    println!("This is a guessing game");
    // mut allows this variable to be changed
    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 5);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("The secrert number is {}", secret_number);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","You win".green());
                break;
            }
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Greater => println!("{}","Too big".red()),
        }
    }
}
