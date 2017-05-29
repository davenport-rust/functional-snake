extern crate piston_window;
extern crate rand;
extern crate find_folder;
extern crate sdl2_window;
mod game;
mod direction;
mod player;
mod gamewindowsettings;
mod food;

use piston_window::*;
use rand::*;
use sdl2_window::Sdl2Window;

use player::*;
use direction::*;
use gamewindowsettings::*;


pub static GAME_NAME: &'static str = "Box Alive";
pub static WINDOW_HEIGHT: u32 = 480;
pub static WINDOW_WIDTH: u32 = 640;
pub static BLOCK_SIZE: u32 = 10; // NOTE: WINDOW_HEIGHT and WINDOW_WIDTH should be divisible by this

pub static GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {

    let mut game_window_settings = GameWindowSettings {
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        block_size: BLOCK_SIZE,
    };

    let mut game_window =
        GameWindow::new_window(game_window_settings.clone(), GAME_NAME.to_string())
            .unwrap();

    let mut glyphs =
        GameWindow::new_glyphs(&mut game_window)
        .unwrap();

    let square = rectangle::square(0.0, 0.0, BLOCK_SIZE as f64);
    let mut player : Option<Player> = Some(Player {
        x: WINDOW_WIDTH / BLOCK_SIZE / 2 * BLOCK_SIZE,
        y: WINDOW_HEIGHT / BLOCK_SIZE / 2 * BLOCK_SIZE,
        direction: Direction::Right,
    });


    while let Some(e) = game_window.next() {
        match e {
            Input::Release(Button::Keyboard(key)) => {
                player = player.and_then(|p|
                    Direction::from_key(key)
                        .map(|dir| Player::change_direction(p, dir))
                )

            }
            Input::Render(_) => {
                game_window.draw_2d(
                    &e, |c, g| {
                        clear(BLACK, g);

                        let new_player_option = Player::advance(player, game_window_settings);

                        match new_player_option {
                            Some(new_player) => {
                                player = Some(new_player);
                                rectangle(
                                    WHITE,
                                    square,
                                    c.transform.trans(new_player.x as f64, new_player.y as f64),
                                    g,
                                );
                            }
                            None => {
                                text::Text::new_color(WHITE, 30).draw(
                                    &format!(
                                        "You Went Out of Bounds!",
                                    ),
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform
                                        .trans(
                                            game_window_settings.window_width as f64 /
                                                4.0 as f64,
                                            game_window_settings.window_height as f64 /
                                                2.0 as f64,
                                        ),
                                    g,
                                );
                            }
                        }
                    }
                );
            }
            _ => {}
        }

    }
}
