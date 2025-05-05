use std::io::stdin;

pub mod player;
use crate::number::Number;
use guess::Guess;
use player::Player;

pub mod guess;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    is_over: bool,
    current_player: usize,
    winning_player: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: vec![],
            is_over: false,
            current_player: 0,
            winning_player: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn run(&mut self) {
        loop {
            let mut input = String::new();
            let _ = stdin().read_line(&mut input);

            let number = Number::from(input.trim());

            if number.is_err() {
                println!("{}", number.unwrap_err());
                continue;
            }

            println!("");

            let guess = guess::Guess::new(number.unwrap());
        }
    }

    pub fn guess(&self, guess: Number, player: &Player) {
        let number: &Number = player.get_number();

        let mut guess = Guess::new(guess);
        guess.process_against(number);

        //TODO: What should I do with this guess?
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn new_game_has_correct_starting_settings() {
        let new_game = Game::new();

        assert_eq!(0, new_game.players.iter().count());
        assert_eq!(false, new_game.is_over);
        assert_eq!(0, new_game.current_player);
        assert_eq!(0, new_game.winning_player);
    }

    #[test]
    fn add_player_updates_game_players_count() {
        let mut new_game = Game::new();

        assert_eq!(0, new_game.players.iter().count());
        let player = Player::new(String::from("Player"), Number::new([1, 2, 3, 4]));
        new_game.add_player(player);

        assert_eq!(1, new_game.players.iter().count());
    }
}
