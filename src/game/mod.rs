use std::io::stdin;

pub mod player;
use crate::number::Number;
use guess::Guess;
use player::Player;

pub mod guess;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    current_player: Option<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: vec![],
            current_player: None,
        }
    }

    pub fn add_player(&mut self) {
        let mut validated = false;
        while validated == false {
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => match Number::from(&input.trim()) {
                    Ok(input) => {
                        let player = Player::new(input);
                        self.players.push(player);
                        validated = true;
                    }
                    Err(e) => println!("Error: {e}"),
                },
                Err(e) => println!("Failed to read line: {}", e),
            }
        }
    }

    pub fn guess(&self, guess: Number, player: &Player) {
        let number: &Number = player.get_number();

        let mut guess = Guess::new(guess);
        guess.process_against(number);

        //TODO: What should I do with this guess?
    }
}
