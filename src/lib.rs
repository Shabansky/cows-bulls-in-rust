pub mod game;
pub mod input_helper;
pub mod number;
use game::player_controller::PlayerControllerError;
use game::view::{TerminalControl, ViewControl};
use game::Game;
use input_helper::{create_new_player_from_input, create_number_from_input};
use number::Number;

mod settings;
use settings::settings::DEFAULT_NUM_PLAYERS;

pub fn init_game() -> Game<TerminalControl> {
    let mut game = Game::new(TerminalControl {});

    for _ in 0..DEFAULT_NUM_PLAYERS {
        add_player(&mut game);
    }

    game
}

pub fn input_closure(game: &Game<TerminalControl>) -> Number {
    let current_player_name = game.get_current_player().get_name().to_string();
    println!("Player {} Guess!", current_player_name);

    create_number_from_input()
}

pub fn game_over_closure(_game: &Game<TerminalControl>) {
    println!("Game over!");
}

fn add_player(game: &mut Game<TerminalControl>) {
    let view_control = TerminalControl {};
    loop {
        let created_player = create_new_player_from_input();
        let player_name = created_player.get_name().clone();
        let player = game.player_controller.add_player(created_player);
        match player {
            Ok(_) => {
                view_control.clear();
                break;
            }
            Err(PlayerControllerError::PlayerWithNameAlreadyExists) => {
                println!("Player with name {} already exists", player_name);
            }
        }
    }
}
