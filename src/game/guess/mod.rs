type Cows = usize;
type Bulls = usize;

use crate::number::Number;

#[derive(Debug)]
pub struct Guess {
    number: Number,
    cows: Cows,
    bulls: Bulls,
}

impl Guess {
    pub fn new(number: Number) -> Self {
        Self {
            number,
            cows: 0,
            bulls: 0,
        }
    }

    pub fn process_against(&mut self, number: &Number) {
        for (guess_column, &guess_value) in self.number.get().iter().enumerate() {
            if guess_value == number.get()[guess_column] {
                self.bulls += 1;
                continue;
            }

            for &num_value in number.get().iter() {
                if guess_value == num_value {
                    self.cows += 1;
                    continue;
                }
            }
        }
    }

    pub fn get_cows(&self) -> Cows {
        self.cows
    }

    pub fn get_bulls(&self) -> Bulls {
        self.bulls
    }

    pub fn is_match(&self) -> bool {
        self.bulls == self.number.get().len()
    }
}
