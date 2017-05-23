extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;


use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::*;
use graphics::*;
use graphics::Context;
use rand::*;


pub static WINDOW_HEIGHT: usize = 480;
pub static WINDOW_WIDTH: usize = 640;
pub static BLOCK_SIZE: usize = 10; // NOTE: WINDOW_HEIGHT and WINDOW_WIDTH should be divisible by this

pub static GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub static RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];


fn main() {
    assert!(WINDOW_WIDTH % BLOCK_SIZE == 0);
    assert!(WINDOW_WIDTH % BLOCK_SIZE == 0);
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32],
    )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let gl = GlGraphics::new(opengl);
    // Create a new game and run it.
    let mut app = App::new(gl);

    let mut events = Events::new(
        EventSettings {
            max_fps: 30,
            ups: 120,
            swap_buffers: true,
            bench_mode: false,
            lazy: false,
            ups_reset: DEFAULT_UPS_RESET,
        },
    );


    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r)
        }
        if let Some(i) = e.release_args() {
            app.on_input(i)
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub struct Grid {
    grid: Vec<Vec<Option<Block>>>,
    snake: Vec<Block>,
    new_block: Block,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Block {
    pub loc: Location,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

pub struct App {
    gl: GlGraphics,
    grid: Grid,
    started: bool,
    game_over: bool,
    direction: Direction,
}

impl Grid {
    pub fn new() -> Grid {

        let mut rows: Vec<Vec<Option<Block>>> = vec![];
        rows.reserve(WINDOW_HEIGHT / BLOCK_SIZE);
        let rows_range = std::ops::Range {
            start: 0,
            end: WINDOW_HEIGHT / BLOCK_SIZE,
        };
        for _ in rows_range {
            let mut column: Vec<Option<Block>> = vec![];
            let columns_range = std::ops::Range {
                start: 0,
                end: WINDOW_WIDTH / BLOCK_SIZE,
            };
            for _ in columns_range {
                column.reserve(WINDOW_WIDTH / BLOCK_SIZE);
                column.push(None);
            }
            rows.push(column);
        }


        let mut grid = Grid {
            grid: rows,
            snake: vec![
                Block::new(
                    Location::new(
                        WINDOW_WIDTH / BLOCK_SIZE / 2,
                        WINDOW_HEIGHT / BLOCK_SIZE / 2,
                    ),
                ),
            ],
            new_block: Block::new(Location::new(0, 0)),
        };
        grid.add_block();
        grid
    }

    pub fn insert(&mut self, block: Block) {
        let (x, y) = (block.loc.x, block.loc.y);
        if !self.valid(x, y) {
            return;
        }

        let gr_loc: Option<Block> = self.grid[x][y].clone();

        if gr_loc == None || gr_loc.unwrap() != block {
            self.grid[x][y] = Some(block);
        }
    }

    pub fn remove(&mut self, block: &Block) {
        if self.valid(block.loc.x, block.loc.y) {
            let mut i = 0;
            while i < self.snake.len() {
                if &self.snake[i] == block {
                    self.snake.remove(i);
                    break;
                }
                i += 1;
            }
            self.grid[block.loc.y][block.loc.x] = None;
        }
    }

    pub fn add_block(&mut self) {
        let x = rand::random::<usize>() % WINDOW_WIDTH / BLOCK_SIZE;
        let y = rand::random::<usize>() % WINDOW_HEIGHT / BLOCK_SIZE;
        let block = Block::new(Location::new(x, y));
        if self.contains(&block) {
            self.add_block();
        } else {
            self.insert(block.clone());
            self.new_block = block;
        }
    }

    pub fn move_snake(&mut self, direction: Direction) {
        let mut blocks = vec![];
        let mut oldblock = self.head().in_direction(self, direction).clone();

        self.grid[oldblock.loc.y][oldblock.loc.x] = Some(oldblock);
        for &block in self.snake.iter().rev() {
            blocks.push(oldblock);
            oldblock = block;
        }

        self.grid[oldblock.loc.y][oldblock.loc.x] = None;
        blocks.reverse();
        self.snake = blocks;
    }

    #[inline]
    pub fn add_to_snake(&mut self, block: Block) {
        self.snake.push(block);
    }

    #[inline]
    pub fn head(&self) -> Block {
        self.snake.last().unwrap().clone()
    }

    pub fn contains(&self, block: &Block) -> bool {
        if self.valid(block.loc.x, block.loc.y) {
            self.grid[block.loc.y][block.loc.x].is_some()
        } else {
            false
        }
    }

    #[inline]
    pub fn valid(&self, x: usize, y: usize) -> bool {
        self.valid_x(x) && self.valid_y(y)
    }

    #[inline]
    pub fn valid_x(&self, x: usize) -> bool {
        x < self.grid[0].len()
    }

    #[inline]
    pub fn valid_y(&self, y: usize) -> bool {
        y < self.grid.len()
    }

    #[inline]
    pub fn render(&self, gl: &mut GlGraphics, win_ctx: &Context) {
        for block in self.snake.iter() {
            block.render(gl, win_ctx);
        }
        self.new_block.render(gl, win_ctx);
    }
}

impl Block {
    #[inline]
    pub fn new(loc: Location) -> Block {
        Block { loc: loc }
    }

    pub fn in_direction(&self, grid: &Grid, dir: Direction) -> Block {
        let gridv = &grid.grid;
        let (x, y) = match dir {
            Direction::Up => (self.loc.x, self.loc.y - 1),
            Direction::Down => (self.loc.x, self.loc.y + 1),
            Direction::Left => (self.loc.x - 1, self.loc.y),
            Direction::Right => (self.loc.x + 1, self.loc.y),
        };
        Block::new(
            if grid.valid_x(x) {
                if grid.valid_y(y) {
                    Location::new(x, y)
                } else if y == gridv.len() {
                    Location::new(x, 0)
                } else {
                    Location::new(x, gridv.len() - 1)
                }
            } else if x == gridv[0].len() {
                Location::new(0, y)
            } else {
                Location::new(gridv[0].len() - 1, y)
            },
        )
    }

    #[inline]
    pub fn render(&self, gl: &mut GlGraphics, c: &Context) {
        let square = rectangle::rectangle_by_corners(
            (self.loc.x * BLOCK_SIZE) as f64,
            (self.loc.y * BLOCK_SIZE) as f64,
            BLOCK_SIZE as f64,
            BLOCK_SIZE as f64,
        );
        let center = c.transform.trans(300.0, 300.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        rectangle(red, square, center, gl);
    }
}

impl Location {
    #[inline]
    pub fn new(x: usize, y: usize) -> Location {
        assert!(x <= WINDOW_WIDTH / BLOCK_SIZE);
        assert!(y <= WINDOW_HEIGHT / BLOCK_SIZE);
        Location { x: x, y: y }
    }
}

impl App {
    #[inline]
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl: gl,
            grid: Grid::new(),
            started: true,
            game_over: false,
            direction: Direction::Up,
        }
    }

    fn on_input(&mut self, but: Button) {
        match but {
            Button::Keyboard(Key::Up) => {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up;
                }
            }
            Button::Keyboard(Key::Down) => {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down;
                }
            }
            Button::Keyboard(Key::Left) => {
                if self.direction != Direction::Right {
                    self.direction = Direction::Left;
                }
            }
            Button::Keyboard(Key::Right) => {
                self.grid = Grid::new();
                self.started = true;
                self.game_over = false;
            }
            _ => {}
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        (&mut self.gl).viewport(0, 0, args.width as i32, args.height as i32);
        let ref c = Context::new_abs(args.width as f64, args.height as f64);

        // c.rgb(1.0, 1.0, 1.0).draw(&mut self.gl);

        if self.game_over {
            // TODO: display game over on screen
        } else if self.started {
            self.render_logic();
        }

        self.grid.render(&mut self.gl, c);
    }

    #[inline]
    fn render_logic(&mut self) {
        let near_head = self.grid
            .head()
            .in_direction(&self.grid, self.direction.clone());
        if near_head == self.grid.new_block {
            let block = self.grid.new_block.clone();
            self.grid.add_to_snake(block);
            self.grid.add_block();
        } else if self.grid.contains(&near_head) {
            self.game_over = true;
        } else {
            self.grid.move_snake(self.direction.clone());
        }
    }
}
