extern crate piston_window;
extern crate rand;
extern crate find_folder;
extern crate sdl2_window;

use piston_window::*;
use rand::*;
use sdl2_window::Sdl2Window;

pub static GAME_NAME: &'static str = "Box Alive";
pub static WINDOW_HEIGHT: u32 = 480;
pub static WINDOW_WIDTH: u32 = 640;
pub static BLOCK_SIZE: u32 = 10; // NOTE: WINDOW_HEIGHT and WINDOW_WIDTH should be divisible by this

pub static GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct GameWindowSettings {
    window_height: u32,
    window_width: u32,
    block_size: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Food {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Player {
    x: u32,
    y: u32,
    direction: Direction,
}

impl Player {
    pub fn change_direction(player: Player, direction: Direction) -> Player {
        use Direction::*;
        if player.direction == direction {
            player
        } else {
            match (player.direction, direction) {
                (Right, Left) => player,
                (Left, Right) => player,
                (Up, Down) => player,
                (Down, Up) => player,
                _ => {
                    Player {
                        x: player.x,
                        y: player.y,
                        direction,
                    }
                }
            }
        }
    }

    pub fn validate_position(player: Player, gw: GameWindowSettings) -> bool {
        if player.x > 0 && player.y > 0 && player.x <= gw.window_width - gw.block_size &&
            player.y <= gw.window_height - gw.block_size
        {
            true
        } else {
            false
        }
    }

    pub fn advance(player: Player, gw: GameWindowSettings) -> Option<Player> {
        let new_player = Player::update_position(player, gw);
        if Player::validate_position(new_player, gw) {
            Some(new_player)
        } else {
            None
        }
    }

    pub fn update_position(player: Player, gw: GameWindowSettings) -> Player {
        match player.direction {
            Direction::Up => {
                if player.y > 0 {
                    Player {
                        x: player.x,
                        y: player.y - gw.block_size,
                        direction: player.direction,
                    }
                } else {
                    player
                }
            }
            Direction::Down => {
                Player {
                    x: player.x,
                    y: player.y + gw.block_size,
                    direction: player.direction,
                }
            }
            Direction::Right => {
                Player {
                    x: player.x + gw.block_size,
                    y: player.y,
                    direction: player.direction,
                }
            }
            Direction::Left => {
                if player.x > 0 {
                    Player {
                        x: player.x - gw.block_size,
                        y: player.y,
                        direction: player.direction,
                    }
                } else {
                    player
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Game {
    active: bool,
    game_window_settings: GameWindowSettings,
    player: Player,
    tail: Vec<(u32, u32)>,
    food: Food,
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




fn main() {
    let game_window_settings: GameWindowSettings = GameWindowSettings {
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        block_size: BLOCK_SIZE,
    };

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new(
            GAME_NAME,
            (game_window_settings.window_width, game_window_settings.window_height),
        )
                .opengl(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    println!("{:?}", font);
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();
    window.set_lazy(false);


    let square = rectangle::square(0.0, 0.0, BLOCK_SIZE as f64);
    let mut player = Player {
        x: WINDOW_WIDTH / BLOCK_SIZE / 2 * BLOCK_SIZE,
        y: WINDOW_HEIGHT / BLOCK_SIZE / 2 * BLOCK_SIZE,
        direction: Direction::Right,
    };


    while let Some(e) = window.next() {
        match e {
            Input::Release(Button::Keyboard(key)) => {
                match key {
                    Key::Up => player = Player::change_direction(player, Direction::Up),
                    Key::Right => player = Player::change_direction(player, Direction::Right),
                    Key::Left => player = Player::change_direction(player, Direction::Left),
                    Key::Down => player = Player::change_direction(player, Direction::Down),
                    _ => {}
                }
            }
            Input::Render(_) => {
                window.draw_2d(
                    &e, |c, g| {
                        let transform = c.transform.trans(10.0, 100.0);

                        clear(BLACK, g);

                        let new_player_option = Player::advance(player, game_window_settings);

                        match new_player_option {
                            Some(new_player) => {
                                player = new_player;
                                rectangle(
                                    WHITE,
                                    square,
                                    c.transform.trans(player.x as f64, player.y as f64),
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
                                                4 as f64,
                                            game_window_settings.window_height as f64 /
                                                2 as f64,
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
