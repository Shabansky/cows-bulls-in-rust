use std::collections::HashSet;
use std::fmt::{Display, Formatter};

const NUM_SIZE: usize = 4;

type Num = [i8; NUM_SIZE];

#[derive(Debug, PartialEq)]
pub enum NumberError {
    NotCorrectSize,
    NotNumeric,
    RepeatingNumbers,
}

#[derive(Debug)]
pub struct Number {
    number: Num,
}

impl Number {
    pub fn new(number: Num) -> Self {
        Number { number }
    }

    pub fn from(text: &str) -> Result<Self, NumberError> {
        match Self::validate(text) {
            Ok(number) => Ok(Self::new(number)),
            Err(text) => Err(text),
        }
    }

    pub fn get(&self) -> Num {
        self.number
    }

    fn validate(number: &str) -> Result<Num, NumberError> {
        if number.len() != NUM_SIZE {
            return Err(NumberError::NotCorrectSize);
        }

        let mut guess_arr: Num = [0; NUM_SIZE];
        for (index, character) in number.chars().enumerate() {
            if !character.is_numeric() {
                return Err(NumberError::NotNumeric);
            }
            guess_arr[index] = character.to_digit(10).unwrap() as i8;
        }

        let mut unique_checker: HashSet<i8> = HashSet::new();
        for value in guess_arr.iter() {
            unique_checker.insert(*value);
        }

        //Checking simply against length as HashSet overwrites duplicating values
        if unique_checker.len() != NUM_SIZE {
            return Err(NumberError::RepeatingNumbers);
        }

        Ok(guess_arr)
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
fn test() {
    assert_eq!(Number::validate("1111"), Err(NumberError::RepeatingNumbers));
}
