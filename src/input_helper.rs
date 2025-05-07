use std::io::stdin;

use crate::game::player::Player;
use crate::number::Number;

fn get_input_as_string() -> Result<String, String> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(format!("Error reading input: {}", e)),
    }
}

fn get_number_from_input(number: &str) -> Result<Number, String> {
    Number::from(&number)
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

pub fn create_number_from_input() -> Number {
    create_new_value_from_input(|| {
        let number = get_input_as_string()?;
        get_number_from_input(&number)
    })
}

pub fn create_new_player_from_input() -> Player {
    let player_name = create_new_value_from_input(|| {
        println!("Please enter player name");
        get_input_as_string()
    });
    let player_number = create_new_value_from_input(|| {
        println!("Please enter player number");
        let number = get_input_as_string()?;
        get_number_from_input(&number)
    });

    Player::new(player_name, player_number)
}
