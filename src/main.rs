use rand::Rng;
use std::io;
use std::cmp::Ordering;
use guessing_game::{
    player::{Human, Robot, Guesser},
    {MIN, MAX}
};

enum PlayerType {
    Human,
    Robot
}

fn main() {
    
    let player_type = get_player_type().unwrap();

    let mut player: Box<dyn Guesser> = match player_type {
        PlayerType::Human => Box::new(Human::new()),
        PlayerType::Robot => Box::new(Robot::new(MIN, MAX))
    };
    
    let secret_number = rand::thread_rng().gen_range(MIN, MAX);

    let mut attempts = 0;
    let mut previous_guess_too_high = false;
    
    println!("Guess a number between {} and {}", MIN, MAX - 1);
    loop {
        attempts += 1;
        if attempts > 30 { break; }
        
        let curr_guess = player.guess(previous_guess_too_high);
        let guess: u32 = curr_guess.trim().parse().unwrap();

        match guess.cmp(&secret_number) {
            Ordering::Less => { 
                println!("Too low, try again:"); 
                previous_guess_too_high = false;
            },
            Ordering::Greater => { 
                println!("Too high, try again:"); 
                previous_guess_too_high = true;
            },
            Ordering::Equal => {
                println!("Correct! Won in {} attempts", attempts);
                if matches!(player_type, PlayerType::Robot) {
                    println!("Answer was {}", secret_number);
                }
                break;
            }
        }
    }
}

fn get_player_type() -> Result<PlayerType, String> {
    let mut is_human = String::new();
    println!("Who would you like to see playing ? (me / robot):");
    io::stdin()
        .read_line(&mut is_human)
        .expect("Failed to read line");

    match is_human.trim().to_lowercase().as_ref() {
        "me" => { Ok(PlayerType::Human) },
        "robot" => { Ok(PlayerType::Robot) },
        _ => { Err("Please type 'me' or 'robot'".to_string()) }
    }
}