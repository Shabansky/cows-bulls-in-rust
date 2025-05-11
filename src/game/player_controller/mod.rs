// pub mod player;
use super::player::Player;

struct PlayerController {
    pub players: Vec<Player>,
    current_player: usize,
    target_player: usize,
    winning_player: usize,
}

impl PlayerController {
    pub fn new() -> PlayerController {
        PlayerController {
            players: vec![],
            current_player: 0,
            target_player: 0,
            winning_player: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn switch_current_player(&mut self) {
        if self.players.is_empty() {
            return;
        }

        self.current_player = (self.current_player + 1) % self.players.len()
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    fn get_opponent_players(&self) -> Vec<&Player> {
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
