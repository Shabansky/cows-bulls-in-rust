use cows_bulls::game::player::Player;
use cows_bulls::game::Game;
use cows_bulls::number::Number;

#[test]
fn test_guess() {
    let player_number = Number::new([1, 2, 3, 4]);
    let game = Game::new();
    let player = Player::new(player_number);

    let guess1: Number = Number::new([1, 2, 3, 4]);
    let guess2: Number = Number::new([4, 3, 2, 1]);
    let guess3: Number = Number::new([1, 2, 3, 5]);

    debug_assert_eq!(game.guess(guess1, &player), (4, 0));
    debug_assert_eq!(game.guess(guess2, &player), (0, 4));
    debug_assert_eq!(game.guess(guess3, &player), (3, 0));
}
