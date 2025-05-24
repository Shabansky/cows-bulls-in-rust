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
        let has_errors = Self::validate(text, num_size);

        if let Some(error) = has_errors {
            Err(error)
        } else {
            Ok(Self::new(
                text.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect(),
            ))
        }
    }

    pub fn get(&self) -> &Num {
        &self.number
    }

    fn validate(number: &str, num_size: usize) -> Option<ValidationError> {
        if num_size == 0 {
            return Some(ValidationError::SizeZero);
        }

        if num_size > NUM_CAP {
            return Some(ValidationError::SizeBeyondLimit);
        }

        let number_chars: Vec<char> = number.chars().collect();

        if number_chars.len() != num_size {
            return Some(ValidationError::NotCorrectSize);
        }

        for character in number_chars.iter() {
            if !character.is_numeric() {
                return Some(ValidationError::NotNumeric);
            }
        }

        let mut unique_checker: HashSet<u8> = HashSet::new();
        for value in number.as_bytes().iter() {
            unique_checker.insert(*value);
        }

        //Checking simply against length as HashSet overwrites duplicating values
        if unique_checker.len() != num_size {
            return Some(ValidationError::RepeatingDigits);
        }

        if number_chars[0] == '0' {
            return Some(ValidationError::FirstDigitZero);
        }

        None
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
        Some(ValidationError::NotCorrectSize)
    );
    assert_eq!(
        Number::validate("", 4),
        Some(ValidationError::NotCorrectSize)
    );
    assert_eq!(
        Number::validate("1234", 11),
        Some(ValidationError::SizeBeyondLimit)
    );
    assert_eq!(Number::validate("1234", 0), Some(ValidationError::SizeZero));
    assert_eq!(
        Number::validate("nota", 4),
        Some(ValidationError::NotNumeric)
    );
    assert_eq!(
        Number::validate("1111", 4),
        Some(ValidationError::RepeatingDigits)
    );
    assert_eq!(
        Number::validate("0123", 4),
        Some(ValidationError::FirstDigitZero)
    );
}
