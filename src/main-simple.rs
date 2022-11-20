use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("Ser, guess the number");
    println!("Enter your guess");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("secret number: {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(r"/!\ Enter a correct number");
                continue;
            }
        };
        println!("You guessed {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Greater => { println!("{}", "Too high".red()); },
            Ordering::Less => { println!("{}", "Too low".red()); },
            Ordering::Equal => { 
                println!("{}", "Correct!".green());
                break;
            }
        }
    }
}