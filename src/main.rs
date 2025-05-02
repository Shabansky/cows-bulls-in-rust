mod number;

use number::Number;
mod input_helper;
use input_helper::create_new_player_from_input;

mod game;
use game::Game;

fn main() {
    println!("Welcome to Cows and Bulls!");
    let mut game = Game::new();

    game.add_player(create_new_player_from_input());
    game.add_player(create_new_player_from_input());

    println!("{:#?}", game);
}

fn set_guess(guess: &str) -> Result<Number, String> {
    let guess = guess.trim();

    Number::from(&guess)
}

#[test]
fn test_set_guess() {
    debug_assert_eq!(set_guess("1234").is_ok(), true);
    debug_assert_eq!(set_guess("1234   ").is_ok(), true);
    debug_assert_eq!(set_guess("abcd").is_err(), true);
    debug_assert_eq!(set_guess("abcd").is_err(), true);
}
