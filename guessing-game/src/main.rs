use std::io; // Library for input-output
use rand::RngExt; // From where I've been studying, I should've used Rng, but I failed to do so, reading further, found out RngExt
use std::cmp::Ordering;
use colored::*;

fn main() {
    
    println!("Guess your number!");
    
    let secret_number = rand::rng().random_range(1..=100);
    
    // println!("Debug for rngExt: secret number is {}", secret_number);

    loop {
    
    println!("Type your guess: ");
    
    let mut guess = String::new(); // Declaration of a mutable variable (it is immutable by default)
    
    io::stdin()
        .read_line(&mut guess) // OK
        .expect("Failed to read input"); // Err
    println!("Your guess: {}", guess);

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue, // The underline "_" means any value 
        // parse guess string -> u32 (int 32bits)
    };

    match guess.cmp(&secret_number){
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too big!".red()),
        Ordering::Equal => {
            println!("{}", "You win!".green());
            break;
        },
    }
    }
}
