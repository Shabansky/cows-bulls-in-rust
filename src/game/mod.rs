use std::default::Default;

pub mod player;

use crate::number::Number;
use guess::Guess;
use player::Player;
pub mod guess;

mod player_controller;
use player_controller::PlayerController;

#[derive(Debug)]
pub struct Game {
    player_controller: PlayerController,
    is_over: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player_controller: PlayerController::new(),
            is_over: false,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.player_controller.add_player(player);
    }

    pub fn get_current_player(&self) -> &Player {
        self.player_controller.get_current_player()
    }

    // fn boot(&mut self) -> Result<(), String> {
    //     /**Responsibilities
    //      * Responsibilities:
    //      * - Define the current player (first one by default)
    //      * - Define the target player
    //      *
    //      *
    //      * */
    //     //Invariance 1 - at least two players
    //     //Invariance 2 - current player always different from target player
    //     Ok(())
    // }

    pub fn run<F, G>(&mut self, input_closure: F, game_over_closure: G)
    where
        F: Fn(&Self) -> Number,
        G: Fn(&Self),
    {
        loop {
            let guess_number = input_closure(self);

            self.guess(guess_number);

            if self.is_over {
                game_over_closure(self);
                return;
            }

            self.player_controller.switch_current_player();
        }
    }

    pub fn guess(&mut self, guess: Number) {
        let mut guess = Guess::new(guess);
        //TODO: Naive assumption that there is an opponent. TEMP!
        let opponent_number = self.player_controller.get_opponent_players()[0].get_number();
        guess.process_against(opponent_number);

        println!(
            "Guess with number {} has {} bulls and {} cows",
            guess.get_number(),
            guess.get_bulls(),
            guess.get_cows()
        );

        if guess.is_match() {
            self.is_over = true;
        }

        self.player_controller
            .get_current_player_mut()
            .add_guess(guess);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn new_game_has_correct_starting_settings() {
        let new_game = Game::new();

        assert_eq!(false, new_game.is_over);
    }

    #[test]
    fn number_match_leads_to_game_end() {
        let mut new_game = Game::new();

        let player = Player::new(String::from("Player 1"), Number::new([1, 2, 3, 4]));
        new_game.add_player(player);
        let target_player = Player::new(String::from("Player 2"), Number::new([4, 3, 2, 1]));
        new_game.add_player(target_player);

        assert!(!new_game.is_over);
        new_game.guess(Number::new([4, 3, 2, 1]));

        assert!(new_game.is_over);
    }
}
