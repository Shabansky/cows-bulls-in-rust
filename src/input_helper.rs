use std::io::stdin;

use crate::game::player::Player;
use crate::number::Number;
use crate::number::ValidationError;

fn get_input_as_string() -> Result<String, String> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(format!("Error reading input: {}", e)),
    }
}

fn get_number_from_input(number: &str) -> Result<Number, String> {
    match Number::from(number, 4) {
        Err(ValidationError::NotCorrectSize) => {
            Err(String::from("The number is not of the correct size"))
        }
        Err(ValidationError::NotNumeric) => Err(String::from("The input is not numeric")),
        Err(ValidationError::RepeatingDigits) => Err(String::from(
            "The number must contain non-repeating digits only",
        )),
        Err(ValidationError::FirstDigitZero) => {
            Err(String::from("The number cannot start with a zero"))
        }
        Err(ValidationError::SizeBeyondLimit) => {
            Err(String::from("The number size provided is too large"))
        }
        Err(ValidationError::SizeZero) => Err(String::from("The number size cannot be 0")),
        Ok(num) => Ok(num),
    }
}

fn get_name_from_input(name: &str) -> Result<String, String> {
    if name.len() == 0 {
        return Err(String::from("Please enter a name for the player"));
    }

    Ok(name.to_string())
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
        let name = get_input_as_string()?;
        get_name_from_input(&name)
    });
    let player_number = create_new_value_from_input(|| {
        println!("Please enter player number");
        let number = get_input_as_string()?;
        get_number_from_input(&number)
    });

    Player::new(player_name, player_number)
}
