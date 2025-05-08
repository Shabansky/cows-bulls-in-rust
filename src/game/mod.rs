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

    fn switch_current_player(&mut self) {
        if self.players.is_empty() {
            return;
        }

        self.current_player = (self.current_player + 1) % self.players.len()
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    //TODO: Consider whether cloning is the best approach here
    fn get_opponent_players(&self) -> Vec<&Player> {
        let current_player_name = self.get_current_player().get_name().clone();

        self.players
            .iter()
            .filter(|player| *player.get_name() != current_player_name)
            .collect()
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

    //TODO: Maybe consider an output_closure for whenever the game is over. Instead of a simple return.
    pub fn run<F>(&mut self, input_closure: F)
    where
        F: Fn(&Self) -> Number,
    {
        loop {
            let guess_number = input_closure(&self);

            self.guess(guess_number);

            if self.is_over == true {
                return;
            }

            self.switch_current_player();
        }
    }

    pub fn guess(&mut self, guess: Number) {
        let mut guess = Guess::new(guess);
        //TODO: Naive assumption that there is an opponent. TEMP!
        let opponent_number = self.get_opponent_players()[0].get_number();
        guess.process_against(opponent_number);

        println!("{}", guess.print());

        if guess.is_match() {
            self.is_over = true;
        }

        self.get_current_player_mut().add_guess(guess);
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

    #[test]
    fn switching_current_player_points_to_different_player() {
        let mut new_game = Game::new();
        let player = Player::new(String::from("Player 1"), Number::new([1, 2, 3, 4]));
        new_game.add_player(player);
        let player = Player::new(String::from("Player 2"), Number::new([1, 2, 3, 4]));
        new_game.add_player(player);

        assert_eq!("Player 1", new_game.get_current_player().get_name());
        new_game.switch_current_player();
        assert_eq!("Player 2", new_game.get_current_player().get_name());
        new_game.switch_current_player();
        assert_eq!("Player 1", new_game.get_current_player().get_name());
    }

    #[test]
    fn switching_current_player_when_no_players() {
        let mut new_game = Game::new();
        assert_eq!(0, new_game.current_player);
        new_game.switch_current_player();
        assert_eq!(0, new_game.current_player);
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

    #[test]
    fn other_player_is_correct_player() {
        let mut new_game = Game::new();

        let player = Player::new(String::from("Player 1"), Number::new([1, 2, 3, 4]));
        new_game.add_player(player);
        let player = Player::new(String::from("Player 2"), Number::new([1, 2, 3, 4]));
        new_game.add_player(player);

        let other_players = new_game.get_opponent_players();

        assert_eq!(1, other_players.len());
        assert_eq!(&String::from("Player 2"), other_players[0].get_name());

        new_game.switch_current_player();
        let other_players = new_game.get_opponent_players();

        assert_eq!(1, other_players.len());
        assert_eq!(&String::from("Player 1"), other_players[0].get_name());
    }
}
