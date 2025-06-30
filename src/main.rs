mod number;

mod game;
mod input_helper;

mod settings;

use cows_bulls::{game_over_closure, init_game, input_closure};

fn main() {
    println!("Welcome to Cows and Bulls!");
    let mut game = init_game();

    game.run(input_closure, game_over_closure);
}
