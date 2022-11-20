use std::io;

pub struct Human {}

impl Human {
    pub fn new () -> Self {
        Self {}
    }
}
pub struct Robot {
    previous_guess: u32,
    min: u32,
    max: u32
}

impl Robot {
    pub fn new (min: u32, max: u32) -> Self {
        Self {
            previous_guess: 0,
            min,
            max
        }
    }
}

pub trait Guesser {
    fn guess(&mut self, too_high: bool) -> String;
}

impl Guesser for Human {
    fn guess(&mut self, too_high: bool) -> String {
        let mut curr_guess = String::new();
        io::stdin()
            .read_line(&mut curr_guess)
            .expect("Failed to read line");
        curr_guess
    }
}

impl Guesser for Robot {
    fn guess(&mut self, previous_guess_too_high: bool) -> String {
        if self.previous_guess == 0 { 
            self.previous_guess = (self.max - self.min) / 2 ;
            return self.previous_guess.to_string();
        }
        if previous_guess_too_high {
            self.max = self.previous_guess;
        } else {
            self.min = self.previous_guess;
        }
        self.previous_guess = (self.max + self.min) / 2;
        self.previous_guess.to_string()
    }
}