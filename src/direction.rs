extern crate piston_window;

use piston_window::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl Direction {
    pub fn from_key(key: Key) -> Option<Direction> {
       match key {
           Key::Right => Some(Direction::Right),
           Key::Left => Some(Direction::Left),
           Key::Down => Some(Direction::Down),
           Key::Up => Some(Direction::Up),
           _ => None
       }
    }
}