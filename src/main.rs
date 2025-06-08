mod number;

mod input_helper;
use input_helper::{create_new_player_from_input, create_number_from_input};
mod game;
use game::player_controller::PlayerControllerError;
use game::view::{TerminalControl, ViewControl};
use game::Game;
fn main() {
    println!("Welcome to Cows and Bulls!");
    let view_control = TerminalControl {};
    let mut game = Game::new(&view_control);

    //TODO: Iterate these based on a player_count settings

    loop {
        let created_player = create_new_player_from_input();

        let player = game
            .player_controller
            .add_player(create_new_player_from_input());
        match player {
            Ok(_) => {
                view_control.clear();
                break;
            }
            Err(PlayerControllerError::PlayerWithNameAlreadyExists) => {
                println!(
                    "Player with name {} already exists",
                    created_player.get_name()
                );
            }
        }
    }

    loop {
        let created_player = create_new_player_from_input();

        let player = game
            .player_controller
            .add_player(create_new_player_from_input());
        match player {
            Ok(_) => {
                view_control.clear();
                break;
            }
            Err(PlayerControllerError::PlayerWithNameAlreadyExists) => {
                println!(
                    "Player with name {} already exists",
                    created_player.get_name()
                );
            }
        }
    }

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
