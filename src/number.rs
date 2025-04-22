const NUM_SIZE: usize = 4;

type Num = [i8; NUM_SIZE];

#[derive(Debug)]
pub struct Number {
    pub number: Num,
}

impl Number {
    pub fn new(number: Num) -> Self {
        Number { number: number }
    }

    pub fn from(text: &str) -> Result<Self, &str> {
        match Self::validate(text) {
            Ok(number) => Ok(Number { number: number }),
            Err(text) => Err(text),
        }
    }

    fn validate(number: &str) -> Result<Num, &str> {
        if number.len() != NUM_SIZE {
            return Err("The number is not of the correct size");
        }

        //TODO: Ensure non-repeating numbers

        let mut guess_arr: Num = [0; NUM_SIZE];
        for (index, character) in number.chars().enumerate() {
            if !character.is_numeric() {
                return Err("The input is not numeric");
            }
            guess_arr[index] = character.to_digit(10).unwrap() as i8;
        }
        Ok(guess_arr)
    }
}
