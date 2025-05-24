use super::player::Player;

#[derive(Debug)]
pub struct PlayerController {
    pub players: Vec<Player>,
    current_player: usize,
}

impl PlayerController {
    pub fn new() -> PlayerController {
        PlayerController {
            players: vec![],
            current_player: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn switch_current_player(&mut self) {
        if self.players.is_empty() {
            return;
        }

        self.current_player = (self.current_player + 1) % self.players.len()
    }

    //TODO: Consider returning an Option here as this can be called before any players are defined.
    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    pub fn get_opponent_players(&self) -> Vec<&Player> {
        let current_player_name = self.get_current_player().get_name();

        self.players
            .iter()
            .filter(|player| player.get_name() != current_player_name)
            .collect()
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::number::Number;

    #[test]
    fn new_controller_has_correct_starting_settings() {
        let new_controller = PlayerController::new();

        assert_eq!(0, new_controller.players.iter().count());
        assert_eq!(0, new_controller.current_player);
    }

    #[test]
    fn add_player_updates_players_count() {
        let mut controller = PlayerController::new();

        assert_eq!(0, controller.players.iter().count());
        let player = Player::new(String::from("Player"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player);

        assert_eq!(1, controller.players.iter().count());
    }

    #[test]
    fn switching_current_player_points_to_different_player() {
        let mut controller = PlayerController::new();
        let player = Player::new(String::from("Player 1"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player);
        let player = Player::new(String::from("Player 2"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player);

        assert_eq!("Player 1", controller.get_current_player().get_name());
        controller.switch_current_player();
        assert_eq!("Player 2", controller.get_current_player().get_name());
        controller.switch_current_player();
        assert_eq!("Player 1", controller.get_current_player().get_name());
    }

    #[test]
    fn switching_current_player_when_no_players() {
        let mut controller = PlayerController::new();
        assert_eq!(0, controller.current_player);
        controller.switch_current_player();
        assert_eq!(0, controller.current_player);
    }

    #[test]
    fn other_player_is_correct_player() {
        let mut controller = PlayerController::new();

        let player = Player::new(String::from("Player 1"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player);
        let player = Player::new(String::from("Player 2"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player);

        let other_players = controller.get_opponent_players();

        assert_eq!(1, other_players.len());
        assert_eq!(&String::from("Player 2"), other_players[0].get_name());

        controller.switch_current_player();
        let other_players = controller.get_opponent_players();

        assert_eq!(1, other_players.len());
        assert_eq!(&String::from("Player 1"), other_players[0].get_name());
    }
}
