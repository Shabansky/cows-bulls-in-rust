use cows_bulls::game::guess::Guess;
use cows_bulls::number::Number;

#[test]
fn test_guess() {
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
