use std::io;

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

    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let guess_input = set_guess(&input.trim());

    //TODO: This is only for a little while
    let temp_player = game.players.get(0);

    // match guess_input {
    //     Ok(_) => {
    //         print_guess_result(
    //             &input,
    //             game.guess(guess_input.unwrap(), temp_player.unwrap()),
    //         );
    //     }
    //     Err(e) => println!("{e}"),
    // }
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
