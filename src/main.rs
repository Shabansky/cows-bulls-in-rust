use std::io;
use std::io::stdin;

mod number;

use number::Number;

mod game;
use game::player::Player;
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

fn get_name_from_input() -> Result<String, String> {
    println!("Please enter player name");
    let mut name = String::new();
    match stdin().read_line(&mut name) {
        Ok(_) => Ok(name),
        Err(e) => Err(format!("Error reading input: {}", e)),
    }
}

fn get_number_from_input() -> Result<Number, String> {
    println!("Please enter player number");
    let mut number = String::new();
    match stdin().read_line(&mut number) {
        Ok(_) => Number::from(&number.trim()),
        Err(e) => Err(format!("Error reading input: {}", e)),
    }
}

fn create_new_player_from_input() -> Player {
    let player_name: String = loop {
        let name = get_name_from_input();

        if name.is_err() {
            continue;
        }

        break name.unwrap();
    };

    let player_number: Number = loop {
        let number = get_number_from_input();

        if number.is_err() {
            continue;
        }

        break number.unwrap();
    };

    Player::new(player_name, player_number)
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
