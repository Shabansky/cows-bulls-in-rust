use std::collections::HashSet;
use std::fmt::{Display, Formatter};

//At most, you can have a 10-digit non-repeating number.
const NUM_CAP: usize = 10;

type Num = Vec<u32>;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    NotCorrectSize,
    NotNumeric,
    RepeatingDigits,
    FirstDigitZero,
    SizeZero,
    SizeBeyondLimit,
}

#[derive(Debug)]
pub struct Number {
    number: Num,
}

impl Number {
    pub fn new(number: Num) -> Self {
        Number { number }
    }

    pub fn from(text: &str, num_size: usize) -> Result<Self, ValidationError> {
        Self::validate(text, num_size)?;

        Ok(Self::new(
            text.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect(),
        ))
    }

    pub fn get(&self) -> &Num {
        &self.number
    }

    //TODO: Not really enjoying that second parameter there
    //TODO: Think about the return type as well. Maybe Result is not the proper one. Option?
    fn validate(number: &str, num_size: usize) -> Result<bool, ValidationError> {
        if num_size == 0 {
            return Err(ValidationError::SizeZero);
        }

        if num_size > NUM_CAP {
            return Err(ValidationError::SizeBeyondLimit);
        }

        let number_chars: Vec<char> = number.chars().collect();

        if number_chars.len() != num_size {
            return Err(ValidationError::NotCorrectSize);
        }

        for character in number_chars.iter() {
            if !character.is_numeric() {
                return Err(ValidationError::NotNumeric);
            }
        }

        let mut unique_checker: HashSet<u8> = HashSet::new();
        for value in number.as_bytes().iter() {
            unique_checker.insert(*value);
        }

        //Checking simply against length as HashSet overwrites duplicating values
        if unique_checker.len() != num_size {
            return Err(ValidationError::RepeatingDigits);
        }

        if number_chars[0] == '0' {
            return Err(ValidationError::FirstDigitZero);
        }

        Ok(true)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.number
                .iter()
                .map(|c| c.to_string())
                .collect::<String>()
        )
    }
}

#[test]
fn validation_catches_cases() {
    assert_eq!(
        Number::validate("123", 4),
        Err(ValidationError::NotCorrectSize)
    );
    assert_eq!(
        Number::validate("", 4),
        Err(ValidationError::NotCorrectSize)
    );
    assert_eq!(
        Number::validate("12345678901234567890", 10),
        Err(ValidationError::NotCorrectSize)
    );
    assert_eq!(
        Number::validate("nota", 4),
        Err(ValidationError::NotNumeric)
    );
    assert_eq!(
        Number::validate("1111", 4),
        Err(ValidationError::RepeatingDigits)
    );
    assert_eq!(
        Number::validate("0123", 4),
        Err(ValidationError::FirstDigitZero)
    );
}
