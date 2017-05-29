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
use sdl2_window::Sdl2Window;

use direction::*;
use gamewindowsettings::*;
use game::*;

pub static GAME_NAME: &'static str = "Box Alive";
pub static WINDOW_HEIGHT: u32 = 480;
pub static WINDOW_WIDTH: u32 = 640;
pub static BLOCK_SIZE: u32 = 10;
// NOTE: WINDOW_HEIGHT and WINDOW_WIDTH should be divisible by BLOCK_SIZE

pub static GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {

    let game_window_settings = GameWindowSettings {
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        block_size: BLOCK_SIZE,
    };

    let mut game_window: PistonWindow<Sdl2Window> =
        new_game_window(game_window_settings.clone(), GAME_NAME.to_string()).unwrap();

    let mut glyphs = new_glyphs(&mut game_window).unwrap();

    let mut game_status = Game::new(game_window_settings);

    while let Some(input) = game_window.next() {
        match game_status.stage {

            GameStage::New => {
                match input {
                    Input::Release(Button::Keyboard(key)) => {
                        match key {
                            Key::N => game_status = game_status.activate(),
                            _ => (),
                        }
                    },
                    Input::Render(_) => {
                        game_window.draw_2d(
                            &input, |context, graphics| {
                                clear(BLACK, graphics);
                                text::Text::new_color(WHITE, 30).draw(
                                    &format!(
                                        "Functional Snake",
                                    ),
                                    &mut glyphs,
                                    &context.draw_state,
                                    context
                                        .transform
                                        .trans(
                                            game_window_settings.window_width as f64 /
                                                3.0 as f64,
                                            game_window_settings.window_height as f64 /
                                                2.0 as f64,
                                        ),
                                    graphics,
                                );
                                text::Text::new_color(WHITE, 16).draw(
                                    &format!(
                                        "Press N to Start",
                                    ),
                                    &mut glyphs,
                                    &context.draw_state,
                                    context
                                        .transform
                                        .trans(
                                            game_window_settings.window_width as f64 /
                                                2.5 as f64,
                                            game_window_settings.window_height as f64 /
                                                4.0 * 3.0 as f64,
                                        ),
                                    graphics,
                                )
                            }
                        );
                        ()
                    },
                    _ => ()
                }
            },

            GameStage::Active => {
                match input {
                    Input::Release(Button::Keyboard(key)) => {
                        game_status = game_status.change_player_direction(Direction::from_key(key))
                    },
                    Input::Render(_) => {
                        game_status = game_status.increment();
                        game_window.draw_2d(
                            &input, |context, graphics| {
                                // Clear Screen
                                clear(BLACK, graphics);
                                // Player Head
                                rectangle(
                                    WHITE,
                                    rectangle::square(
                                        0.0,
                                        0.0,
                                        game_status.game_window_settings.block_size as f64,
                                    ),
                                    context
                                        .transform
                                        .trans(game_status.player.x as f64, game_status.player.y as f64),
                                    graphics,
                                );

                                // Food

                            }
                        );
                        ()
                    }
                    _ => ()
                }
            },

            GameStage::GameOver => {
                match input {
                    Input::Release(Button::Keyboard(key)) => {
                        match key {
                            Key::R => game_status = Game::new(game_window_settings).activate(),
                            _ => (),
                        }
                    }
                    Input::Render(_) => {
                        game_window.draw_2d(
                            &input, |context, graphics| {
                                clear(BLACK, graphics);
                                text::Text::new_color(WHITE, 30).draw(
                                    &format!(
                                        "Game Over"
                                    ),
                                    &mut glyphs,
                                    &context.draw_state,
                                    context
                                        .transform
                                        .trans(
                                            game_window_settings.window_width as f64 /
                                                2.6 as f64,
                                            game_window_settings.window_height as f64 /
                                                2.0 as f64,
                                        ),
                                    graphics,
                                );
                                text::Text::new_color(WHITE, 16).draw(
                                    &format!(
                                        "Press R to Restart",
                                    ),
                                    &mut glyphs,
                                    &context.draw_state,
                                    context
                                        .transform
                                        .trans(
                                            game_window_settings.window_width as f64 /
                                                2.5 as f64,
                                            game_window_settings.window_height as f64 /
                                                4.0 * 3.0 as f64,
                                        ),
                                    graphics,
                                )
                            }
                        );
                        ()
                    }
                    _ => ()
                }

            }
        }

//        match input {
////            Input::Release(Button::Keyboard(key)) => {
////                match key {
////                    Key::R => player = Player::new(game_window_settings),
////                    _ => player = player.map(
////                        |p| {
////                            Player::change_direction(p, Direction::from_key(key))
////                        }
////                    ),
////                }
////
////            }
//            Input::Render(_) => {
//                let new_player_option = Player::advance(player, game_window_settings);
//                match new_player_option {
//                    Some(new_player) => {
//
//                        player = Some(new_player);
//                        game_window.draw_2d(
//                            &input, |context, graphics| {
//                                clear(BLACK, graphics);
//
//                                // Player Head
//                                rectangle(
//                                    WHITE,
//                                    rectangle::square(
//                                        0.0,
//                                        0.0,
//                                        game_window_settings.block_size as f64,
//                                    ),
//                                    context
//                                        .transform
//                                        .trans(new_player.x as f64, new_player.y as f64),
//                                    graphics,
//                                );
//
//                                // Food
//
//                            }
//                        );
//                        ()
//                    }
//                    None => {
//                        ;
//                        ()
//                    }
//                }
//            }
//            _ => {}
//        }

    }
}
