use std::io::stdin;

pub mod player;
use crate::number::Number;
use player::Player;

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

    pub fn guess(&self, guess: Number, player: &Player) -> (i8, i8) {
        let number: &Number = player.get_number();

        let mut cows: i8 = 0;
        let mut bulls: i8 = 0;

        for (guess_column, &guess_value) in guess.number.iter().enumerate() {
            if guess_value == number.number[guess_column] {
                bulls += 1;
                continue;
            }

            for &num_value in number.number.iter() {
                if guess_value == num_value {
                    cows += 1;
                    continue;
                }
            }
        }

        (bulls, cows)
    }
}
