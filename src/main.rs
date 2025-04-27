use std::io;

mod number;

use number::Number;

mod game;
use game::player::Player;
use game::Game;

fn main() {
    println!("Welcome to Cows and Bulls!");
    let mut game = Game::new();
    println!("Please enter Player 1 number");
    game.add_player();
    println!("Please enter Player 2 number");
    game.add_player();
    println!("{game:#?}");
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let guess_input = set_guess(&input.trim());

    //TODO: This is only for a little while
    let temp_player = game.players.get(0);

    match guess_input {
        Ok(_) => {
            print_guess_result(
                &input,
                game.guess(guess_input.unwrap(), temp_player.unwrap()),
            );
        }
        Err(e) => println!("{e}"),
    }
}

fn print_guess_result(guess: &str, guess_results: (i8, i8)) {
    let bulls = guess_results.0;
    let cows = guess_results.1;

    println!("{} has {} bulls and {} cows", guess.trim(), bulls, cows);
}

fn set_guess(guess: &str) -> Result<Number, &str> {
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
