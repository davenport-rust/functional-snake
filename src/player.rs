use direction::*;
use gamewindowsettings::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Player {
    pub x: u32,
    pub y: u32,
    pub direction: Direction,
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

    pub fn advance(player: Option<Player>, gw: GameWindowSettings) -> Option<Player> {
        match player {
            Some(p) => {
                let new_player = Player::update_position(p, gw);
                if Player::validate_position(new_player, gw) {
                    Some(new_player)
                } else {
                    None
                }
            }
            None => None
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