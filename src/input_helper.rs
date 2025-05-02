use std::io::stdin;

use crate::game::player::Player;
use crate::number::Number;

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

fn create_new_value_from_input<T, F>(mut procedure: F) -> T
where
    F: FnMut() -> Result<T, String>,
{
    loop {
        let input = procedure();

        match input {
            Ok(input) => {
                break input;
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}

pub fn create_new_player_from_input() -> Player {
    let player_name = create_new_value_from_input(get_name_from_input);
    let player_number = create_new_value_from_input(get_number_from_input);

    Player::new(player_name, player_number)
}
