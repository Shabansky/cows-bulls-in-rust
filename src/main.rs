use std::io;
fn main() {
    asserts();

    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let guess_input = set_guess(&input.trim());

    match guess_input {
        Some(_) => {
            print_guess_result(&input, guess(guess_input.unwrap()));
        }
        None => println!("{} is not 4 digit or a numeral", input.trim()),
    }
}

fn print_guess_result(guess: &str, guess_results: (i8, i8)) {
    let bulls = guess_results.0;
    let cows = guess_results.1;

    //TODO: Use the above to modify bulls to bull if singular etc.

    println!(
        "{} has {} bulls and {} cows",
        guess.trim(),
        guess_results.0,
        guess_results.1
    );
}

fn asserts() {
    let guess1: [i8; 4] = [1, 2, 3, 4];
    let guess2: [i8; 4] = [4, 3, 2, 1];
    let guess3: [i8; 4] = [1, 2, 3, 5];

    debug_assert_eq!(guess(guess1), (4, 0));
    debug_assert_eq!(guess(guess2), (0, 4));
    debug_assert_eq!(guess(guess3), (3, 0));

    debug_assert_eq!(set_guess("1234"), Some([1, 2, 3, 4]));
    debug_assert_eq!(set_guess("abcd"), None);
    debug_assert_eq!(set_guess("1234   "), None);
}

fn set_guess(guess: &str) -> Option<[i8; 4]> {
    if guess.len() != 4 {
        return None;
    }

    let mut guess_arr = [0; 4];
    for (index, character) in guess.chars().enumerate() {
        if !character.is_numeric() {
            return None;
        }
        guess_arr[index] = character.to_digit(10).unwrap() as i8;
    }
    Some(guess_arr)
}

fn guess(guess: [i8; 4]) -> (i8, i8) {
    let number: [i8; 4] = [1, 2, 3, 4];

    let mut cows: i8 = 0;
    let mut bulls: i8 = 0;

    for (guess_column, &guess_value) in guess.iter().enumerate() {
        if guess_value == number[guess_column] {
            bulls += 1;
            continue;
        }

        for (_, &num_value) in number.iter().enumerate() {
            if guess_value == num_value {
                cows += 1;
                continue;
            }
        }
    }

    (bulls, cows)
}
