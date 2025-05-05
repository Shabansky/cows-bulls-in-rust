use super::{guess::Guess, Number};

#[derive(Debug)]
pub struct Player {
    name: String,
    number: Number,
    guesses: Vec<Guess>,
}

impl Player {
    pub fn new(name: String, number: Number) -> Self {
        Self {
            name: name,
            number: number,
            guesses: vec![],
        }
    }

    pub fn get_number(&self) -> &Number {
        &self.number
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_guess(&mut self, guess: Guess) {
        self.guesses.push(guess);
    }
}
