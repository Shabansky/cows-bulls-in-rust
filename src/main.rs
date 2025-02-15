fn main() {
    let guess1: [i8; 4] = [1, 2, 3, 4];
    let guess2: [i8; 4] = [4, 3, 2, 1];
    let guess3: [i8; 4] = [1, 2, 3, 5];

    debug_assert_eq!(guess(guess1), (4, 0));
    debug_assert_eq!(guess(guess2), (0, 4));
    debug_assert_eq!(guess(guess3), (3, 0));
}

fn guess(guess: [i8; 4]) -> (i8, i8) {
    let number: [i8; 4] = [1, 2, 3, 4];

    let mut cows: i8 = 0;
    let mut bulls: i8 = 0;

    for (guess_index, &guess_value) in guess.iter().enumerate() {
        if guess_value == number[guess_index] {
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
