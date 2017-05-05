


fn main() {
    println!("Hello, world!");


}


pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Board {
    rows: usize,
    cols: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Snake {
    body: Vec<Position>,
}

impl Snake {
    pub fn move_directions(&self, direction: Direction) -> Self {
        match direction {
            Direction::Left => Snake { body: self.body.clone() },
            Direction::Right => {
                let new_vec: Vec<Position> = self.body.clone();
                new_vec.last().cloned();

                Snake { body: new_vec }
            }
            Direction::Up => {
                let new_vec: Vec<Position> = self.body.clone();
                Snake { body: new_vec }
            }
            Direction::Down => {
                let new_vec: Vec<Position> = self.body.clone();
                Snake { body: new_vec }
            }

        }
    }

    fn new() -> Self {
        Snake { body: vec![Position { x: 0, y: 0 }] }
    }
}
