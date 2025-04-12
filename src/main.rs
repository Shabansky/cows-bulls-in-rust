use std::io;
fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let guess_input = set_guess(&input.trim());

    match guess_input {
        Ok(_) => {
            print_guess_result(&input, guess(guess_input.unwrap()));
        }
        Err(e) => println!("{e}"),
    }
}

fn print_guess_result(guess: &str, guess_results: (i8, i8)) {
    let bulls = guess_results.0;
    let cows = guess_results.1;

    //TODO: Use the above to modify bulls to bull if singular etc.
    println!("{} has {} bulls and {} cows", guess.trim(), bulls, cows);
}

fn set_guess(guess: &str) -> Result<[i8; 4], &str> {
    let guess = guess.trim();

    if guess.len() != 4 {
        return Err("The number is not of the correct size");
    }

    let mut guess_arr = [0; 4];
    for (index, character) in guess.chars().enumerate() {
        if !character.is_numeric() {
            return Err("The input is not numeric");
        }
        guess_arr[index] = character.to_digit(10).unwrap() as i8;
    }
    Ok(guess_arr)
}

#[test]
fn test_set_guess() {
    debug_assert_eq!(set_guess("1234"), Ok([1, 2, 3, 4]));
    debug_assert_eq!(set_guess("1234   "), Ok([1, 2, 3, 4]));
    debug_assert_eq!(set_guess("abcd"), Err("The input is not numeric"));
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

        for &num_value in number.iter() {
            if guess_value == num_value {
                cows += 1;
                continue;
            }
        }
    }

    (bulls, cows)
}

#[test]
fn test_guess() {
    let guess1: [i8; 4] = [1, 2, 3, 4];
    let guess2: [i8; 4] = [4, 3, 2, 1];
    let guess3: [i8; 4] = [1, 2, 3, 5];

    debug_assert_eq!(guess(guess1), (4, 0));
    debug_assert_eq!(guess(guess2), (0, 4));
    debug_assert_eq!(guess(guess3), (3, 0));
}
