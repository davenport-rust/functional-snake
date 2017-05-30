use direction::*;
use gamewindowsettings::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Player {
    pub x: u32,
    pub y: u32,
    pub direction: Direction,
}

impl Player {
    pub fn new(game_window_settings: GameWindowSettings) -> Player {
        Player {
            x: game_window_settings.window_width / game_window_settings.block_size / 2 *
                game_window_settings.block_size,
            y: game_window_settings.window_height / game_window_settings.block_size / 2 *
                game_window_settings.block_size,
            direction: Direction::Right,
        }
    }

    pub fn change_direction(player: Player, direction_opt: Option<Direction>) -> Player {
        use Direction::*;

        let new_direction = direction_opt
            .map(|direction| Direction::update_direction(player.direction, direction))
            .unwrap_or(player.direction);

        Player {
            x: player.x,
            y: player.y,
            direction: new_direction,
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
        if Player::validate_position(player, gw) {
            let new_player = Player::update_position(player, gw);
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
