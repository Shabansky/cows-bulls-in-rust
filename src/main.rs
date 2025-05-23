mod number;

mod input_helper;
use input_helper::{create_new_player_from_input, create_number_from_input};
mod game;
use game::view::{TerminalControl, ViewControl};
use game::Game;
fn main() {
    println!("Welcome to Cows and Bulls!");
    let view_control = TerminalControl {};
    let mut game = Game::new(&view_control);

    //TODO: Iterate these based on a player_count settings
    game.add_player(create_new_player_from_input());
    view_control.clear();
    game.add_player(create_new_player_from_input());
    view_control.clear();

    game.run(
        |game| {
            let current_player_name = game.get_current_player().get_name().to_string();
            println!("Player {} Guess!", current_player_name);

            create_number_from_input()
        },
        |_| {
            println!("Game over!");
        },
    );
}
