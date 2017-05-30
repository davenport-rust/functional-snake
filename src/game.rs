extern crate rand;
use rand::*;

use player::*;
use direction::*;
use gamewindowsettings::*;
use food::*;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum GameStage {
    New,
    Active,
    GameOver,
}

pub enum GameOverState {
    AteTail,
    OutOfBounds,
    Win,
}

impl GameOverState {
    fn announce(self) -> String {
        match self {
            GameOverState::AteTail => "You Ate Your Tail!".to_string(),
            GameOverState::OutOfBounds => "You Went out of Bounds!".to_string(),
            GameOverState::Win => "You Win!".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Game {
    pub stage: GameStage,
    pub game_window_settings: GameWindowSettings,
    pub player: Player,
    pub tail: Vec<(u32, u32)>,
    pub food: Food,
}

impl Game {
    pub fn new(gws: GameWindowSettings) -> Game {
        let mut rng = rand::thread_rng();
        Game {
            stage: GameStage::New,
            game_window_settings: gws,
            player: Player::new(gws),
            tail: Vec::new(),
            food: Food {
                x: rng.gen_range(0, gws.window_width),
                y: rng.gen_range(0, gws.window_height),
            },
        }
    }

    pub fn activate(self) -> Game {
        Game {
            stage: GameStage::Active,
            game_window_settings: self.game_window_settings,
            player: self.player,
            tail: self.tail,
            food: self.food,
        }
    }

    pub fn increment(self) -> Game {
        let new_player_opt = Player::advance(self.player, self.game_window_settings);
        match new_player_opt {
            Some(new_player) => {
                Game {
                    stage: self.stage,
                    game_window_settings: self.game_window_settings,
                    player: new_player,
                    tail: self.tail,
                    food: self.food,
                }
            }
            None => {
                Game {
                    stage: GameStage::GameOver,
                    game_window_settings: self.game_window_settings,
                    player: self.player,
                    tail: self.tail,
                    food: self.food,
                }
            }
        }
    }

    pub fn change_player_direction(self, direction: Option<Direction>) -> Game {
        let new_player = Player::change_direction(self.player, direction);
        Game {
            stage: self.stage,
            game_window_settings: self.game_window_settings,
            player: new_player,
            tail: self.tail,
            food: self.food,
        }

    }
}
