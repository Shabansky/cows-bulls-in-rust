fn main() {
    let guess1: [i8; 4] = [1, 2, 3, 4];
    let guess2: [i8; 4] = [4, 3, 2, 1];
    let guess3: [i8; 4] = [1, 2, 3, 5];
    guess(guess1);
    guess(guess2);
    guess(guess3);
}

fn guess(guess: [i8; 4]) {
    let number: [i8; 4] = [1, 2, 3, 4];
    
    let mut cows:i8 = 0;
    let mut bulls:i8 = 0;

    let mut i:usize = 0;
    loop {
        if i == 4 {
            break;
        }

        if guess[i] == number[i] {
            bulls += 1
        } else {
            let mut j:usize = 0;
            loop {
                if j == 4 {
                    break;
                }

                if guess[i] == number[j] {
                    cows += 1;
                }

                j += 1;
            }
        }
        i += 1;
    }
    println!("Cows: {}, Bulls: {}", cows, bulls);
}
