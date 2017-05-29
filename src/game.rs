extern crate rand;
extern crate piston_window;
extern crate sdl2_window;
use rand::*;
use piston_window::*;
use sdl2_window::Sdl2Window;

use player::*;
use direction::*;
use gamewindowsettings::*;
use food::*;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Game {
    pub active: bool,
    pub game_window_settings: GameWindowSettings,
    pub player: Player,
    pub tail: Vec<(u32, u32)>,
    pub food: Food,
}

impl Game {
    pub fn new(gws: GameWindowSettings) -> Game {
        let mut rng = rand::thread_rng();
        Game {
            active: true,
            game_window_settings: gws,
            player: Player {
                x: gws.window_width / gws.block_size / 2 * gws.block_size,
                y: gws.window_height / gws.block_size / 2 * gws.block_size,
                direction: Direction::Right,
            },
            tail: Vec::new(),
            food: Food {
                x: rng.gen_range(0, gws.window_width),
                y: rng.gen_range(0, gws.window_height),
            },
        }
    }
}
