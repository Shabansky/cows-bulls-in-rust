use super::{guess::Guess, Number};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Player {
    name: String,
    number: Number,
    guesses: HashMap<String, Guess>,
}

impl Player {
    pub fn new(name: String, number: Number) -> Self {
        Self {
            name,
            number,
            guesses: HashMap::new(),
        }
    }

    pub fn get_number(&self) -> &Number {
        &self.number
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_guess(&mut self, guess: Guess) {
        self.guesses.insert(guess.to_string(), guess);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::Number;

    #[test]
    fn new_guess_updates_guesses_counter() {
        let number = Number::from("1234", 4).unwrap();
        let mut player = Player::new("Name".to_string(), number);
        let guess = Number::from("1234", 4).unwrap();

        assert_eq!(0, player.guesses.len(), "player guesses not right number");
        player.add_guess(Guess::new(guess));

        assert_eq!(1, player.guesses.len(), "player guesses not right number");
    }

    #[test]
    fn old_guess_does_not_update_guesses_counter() {
        let number = Number::from("1234", 4).unwrap();
        let mut player = Player::new("Name".to_string(), number);
        let guess = Number::from("1234", 4).unwrap();
        let guess_2 = Number::from("1234", 4).unwrap();
        assert_eq!(0, player.guesses.len(), "player guesses not right number");
        player.add_guess(Guess::new(guess));
        assert_eq!(1, player.guesses.len(), "player guesses not right number");
        player.add_guess(Guess::new(guess_2));

        assert_eq!(1, player.guesses.len(), "player guesses not right number");
    }
}
