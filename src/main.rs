use std::io;

mod number;

use number::Number;

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
    current_player: Option<Player>,
}

#[derive(Debug)]
struct Player {
    number: Number,
    guesses: Vec<i32>,
}

impl Player {
    fn new(number: Number) -> Self {
        Self {
            number: number,
            guesses: vec![],
        }
    }
}

fn main() {
    println!("Welcome to Cows and Bulls!");
    let game = init_game();
    println!("{game:#?}");
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let guess_input = set_guess(&input.trim());

    //TODO: This is only for a little while
    let temp_player = &game.players[0];

    match guess_input {
        Ok(_) => {
            print_guess_result(&input, guess(guess_input.unwrap(), temp_player));
        }
        Err(e) => println!("{e}"),
    }
}

fn init_game() -> Game {
    let mut game = Game {
        players: vec![],
        current_player: None,
    };

    let mut init_players = 1;

    while init_players <= 2 {
        println!("Please enter Player {} number", init_players);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match Number::from(&input.trim()) {
                Ok(input) => {
                    let player = Player::new(input);
                    game.players.push(player);
                    init_players += 1;
                }
                Err(e) => println!("Error: {e}"),
            },
            Err(e) => println!("Failed to read line: {}", e),
        }
    }

    game
}

fn print_guess_result(guess: &str, guess_results: (i8, i8)) {
    let bulls = guess_results.0;
    let cows = guess_results.1;

    //TODO: Use the above to modify bulls to bull if singular etc.
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
}

fn guess(guess: Number, player: &Player) -> (i8, i8) {
    let number: &Number = &player.number;

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

#[test]
fn test_guess() {
    let player_number = Number::new([1, 2, 3, 4]);
    let player = Player::new(player_number);

    let guess1: Number = Number::new([1, 2, 3, 4]);
    let guess2: Number = Number::new([4, 3, 2, 1]);
    let guess3: Number = Number::new([1, 2, 3, 5]);

    debug_assert_eq!(guess(guess1, &player), (4, 0));
    debug_assert_eq!(guess(guess2, &player), (0, 4));
    debug_assert_eq!(guess(guess3, &player), (3, 0));
}
