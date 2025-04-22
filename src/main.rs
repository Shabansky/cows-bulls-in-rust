use std::io;

const NUM_SIZE: usize = 4;

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
}

#[derive(Debug)]
struct Player {
    number: [i8; NUM_SIZE],
    guesses: Vec<i32>,
}

impl Player {
    fn new(number: [i8; NUM_SIZE]) -> Self {
        Self {
            number: number,
            guesses: vec![],
        }
    }
}

fn validate_number(number: &str) -> Result<[i8; NUM_SIZE], &str> {
    dbg!(number);
    if number.len() != NUM_SIZE {
        return Err("The number is not of the correct size");
    }

    //TODO: Ensure non-repeating numbers

    let mut guess_arr = [0; NUM_SIZE];
    for (index, character) in number.chars().enumerate() {
        if !character.is_numeric() {
            return Err("The input is not numeric");
        }
        guess_arr[index] = character.to_digit(10).unwrap() as i8;
    }
    Ok(guess_arr)
}

fn main() {
    println!("Welcome to Cows and Bulls!");
    let game = init_game();
    println!("{game:#?}");
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let guess_input = set_guess(&input.trim());

    match guess_input {
        Ok(_) => {
            print_guess_result(&input, guess(guess_input.unwrap()));
        }
        Err(e) => println!("{e}"),
    }
}

fn init_game() -> Game {
    let mut game = Game { players: vec![] };
    let mut init_players = 1;

    while init_players <= 2 {
        println!("Please enter Player {} number", init_players);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match validate_number(&input.trim()) {
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

fn set_guess(guess: &str) -> Result<[i8; NUM_SIZE as usize], &str> {
    let guess = guess.trim();

    validate_number(&guess)
}

#[test]
fn test_set_guess() {
    debug_assert_eq!(set_guess("1234"), Ok([1, 2, 3, 4]));
    debug_assert_eq!(set_guess("1234   "), Ok([1, 2, 3, 4]));
    debug_assert_eq!(set_guess("abcd"), Err("The input is not numeric"));
}

fn guess(guess: [i8; NUM_SIZE]) -> (i8, i8) {
    //TODO: This needs to go
    let number: [i8; NUM_SIZE] = [1, 2, 3, 4];

    let mut cows: i8 = 0;
    let mut bulls: i8 = 0;

    for (guess_column, &guess_value) in guess.iter().enumerate() {
        if guess_value == number[guess_column] {
            bulls += 1;
            continue;
        }

        for &num_value in number.iter() {
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
    let guess1: [i8; NUM_SIZE] = [1, 2, 3, 4];
    let guess2: [i8; NUM_SIZE] = [4, 3, 2, 1];
    let guess3: [i8; NUM_SIZE] = [1, 2, 3, 5];

    debug_assert_eq!(guess(guess1), (4, 0));
    debug_assert_eq!(guess(guess2), (0, 4));
    debug_assert_eq!(guess(guess3), (3, 0));
}
