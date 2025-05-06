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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn guess_returns_correct_number_of_cows_and_bulls() {
        let player_number = Number::new([1, 2, 3, 4]);

        let number1: Number = Number::new([1, 2, 3, 4]);
        let number2: Number = Number::new([4, 3, 2, 1]);
        let number3: Number = Number::new([1, 2, 3, 5]);

        let mut guess1 = Guess::new(number1);
        guess1.process_against(&player_number);
        debug_assert_eq!((guess1.get_bulls(), guess1.get_cows()), (4, 0));

        let mut guess2: Guess = Guess::new(number2);
        guess2.process_against(&player_number);
        debug_assert_eq!((guess2.get_bulls(), guess2.get_cows()), (0, 4));

        let mut guess3 = Guess::new(number3);
        guess3.process_against(&player_number);
        debug_assert_eq!((guess3.get_bulls(), guess3.get_cows()), (3, 0));
    }
}
