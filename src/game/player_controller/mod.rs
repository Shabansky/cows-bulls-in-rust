use super::player::Player;

#[derive(PartialEq, Debug)]
pub enum PlayerControllerError {
    PlayerWithNameAlreadyExists,
}

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

    pub fn add_player(&mut self, player: Player) -> Result<(), PlayerControllerError> {
        let new_player_name = player.get_name();

        for player in self.players.iter() {
            if player.get_name() == new_player_name {
                return Err(PlayerControllerError::PlayerWithNameAlreadyExists);
            }
        }

        self.players.push(player);
        Ok(())
    }

    pub fn switch_current_player(&mut self) {
        if self.players.is_empty() {
            return;
        }

        self.current_player = (self.current_player + 1) % self.players.len()
    }

    pub fn get_current_player(&self) -> &Player {
        self.players
            .get(self.current_player)
            .expect("cannot retrieve current player as none set")
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        self.players
            .get_mut(self.current_player)
            .expect("cannot retrieve current player as none set")
    }

    pub fn get_opponent_players(&self) -> Vec<&Player> {
        //TODO: Think if this can be further made better (it can)
        self.players
            .iter()
            .enumerate()
            .filter_map(|(index, player)| {
                if index != self.current_player {
                    Some(player)
                } else {
                    None
                }
            })
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
    #[should_panic(expected = "cannot retrieve current player as none set")]
    fn no_player_in_controller_panics_when_current_player_called() {
        let controller = PlayerController::new();
        controller.get_current_player();
    }

    #[test]
    #[should_panic(expected = "cannot retrieve current player as none set")]
    fn no_player_in_controller_panics_when_current_player_mut_called() {
        let mut controller = PlayerController::new();
        controller.get_current_player_mut();
    }

    #[test]
    fn add_player_updates_players_count() {
        let mut controller = PlayerController::new();

        assert_eq!(0, controller.players.iter().count());
        let player = Player::new(String::from("Player"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player).unwrap();

        assert_eq!(1, controller.players.iter().count());
    }

    #[test]
    fn switching_current_player_points_to_different_player() {
        let mut controller = PlayerController::new();
        let player = Player::new(String::from("Player 1"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player).unwrap();
        let player = Player::new(String::from("Player 2"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player).unwrap();

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
        controller.add_player(player).unwrap();
        let player = Player::new(String::from("Player 2"), Number::new(vec![1, 2, 3, 4]));
        controller.add_player(player).unwrap();

        let other_players = controller.get_opponent_players();

        assert_eq!(1, other_players.len());
        assert_eq!(&String::from("Player 2"), other_players[0].get_name());

        controller.switch_current_player();
        let other_players = controller.get_opponent_players();

        assert_eq!(1, other_players.len());
        assert_eq!(&String::from("Player 1"), other_players[0].get_name());
    }

    #[test]
    fn player_with_duplicate_name_produces_error() {
        let mut controller = PlayerController::new();
        let player_original = Player::new(String::from("Player"), Number::new(vec![1, 2, 3, 4]));
        assert_eq!(Ok(()), controller.add_player(player_original));
        let player_duplicate = Player::new(String::from("Player"), Number::new(vec![1, 2, 3, 4]));

        assert_eq!(
            Err(PlayerControllerError::PlayerWithNameAlreadyExists),
            controller.add_player(player_duplicate)
        );
    }
}
