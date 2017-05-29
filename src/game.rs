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

//    pub fn run(&self, mut window: PistonWindow<Sdl2Window>) -> () {
//
//        while let Some(e) = window.next() {
//            match e {
//                Input::Release(Button::Keyboard(key)) => {
//                    match key {
//                        Key::Up => self.player = Player::change_direction(player, Direction::Up),
//                        Key::Right => player = Player::change_direction(player, Direction::Right),
//                        Key::Left => player = Player::change_direction(player, Direction::Left),
//                        Key::Down => player = Player::change_direction(player, Direction::Down),
//                        _ => {}
//                    }
//                }
//                Input::Render(_) => {
//                    window.draw_2d(
//                        &e, |c, g| {
//                            //                        let transform = c.transform.trans(10.0, 100.0);
//
//                            clear(BLACK, g);
//
//                            let new_player_option = Player::advance(player, game_window_settings);
//
//                            match new_player_option {
//                                Some(new_player) => {
//                                    player = new_player;
//                                    rectangle(
//                                        WHITE,
//                                        square,
//                                        c.transform.trans(player.x as f64, player.y as f64),
//                                        g,
//                                    );
//                                }
//                                None => {
//                                    text::Text::new_color(WHITE, 30).draw(
//                                        &format!(
//                                            "You Went Out of Bounds!",
//                                        ),
//                                        &mut glyphs,
//                                        &c.draw_state,
//                                        c.transform
//                                            .trans(
//                                                game_window_settings.window_width as f64 /
//                                                    4 as f64,
//                                                game_window_settings.window_height as f64 /
//                                                    2 as f64,
//                                            ),
//                                        g,
//                                    );
//                                }
//                            }
//                        }
//                    );
//                }
//                _ => {}
//            }
//
//        }
//    }
}