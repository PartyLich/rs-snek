use rand::Rng;

use crate::{
    snake::Snake,
    types::{self, Direction, Food, Grid, FOOD_COLOR, SNAKE_COLOR},
};

/// The state of the gameworld
#[derive(Debug)]
pub struct Gamestate {
    pub grid: Grid,
    pub world_size: (u32, u32),

    /// The player's direction of travel
    pub direction: Direction,

    /// The player's avatar
    pub player: crate::snake::Snake,

    /// The player's objective
    pub food: Food,

    /// The player's score
    pub score: usize,
}

impl Gamestate {
    pub fn new(rows: u32, cols: u32) -> Self {
        Gamestate {
            grid: grid_init(cols, rows),
            direction: Direction::Down,
            player: Snake::new(0, 0, None),
            food: Food::new(rows / 2, cols / 2, Some(FOOD_COLOR)),
            world_size: (rows, cols),
            score: 0,
        }
    }

    pub fn fresh_food(&mut self) {
        let mut row = rand::thread_rng().gen_range(0, self.grid.len());
        let mut col = rand::thread_rng().gen_range(0, self.grid[0].len());

        while self.grid[row][col] == SNAKE_COLOR {
            row = rand::thread_rng().gen_range(0, self.grid.len());
            col = rand::thread_rng().gen_range(0, self.grid[0].len());
        }

        self.food = Food::new(row as u32, col as u32, Some(FOOD_COLOR));
    }
}

/// Initialize grid
///
/// Creates a width x height vector of `Cells`
pub fn grid_init(width: u32, height: u32) -> Grid {
    let mut grid_vector = Vec::with_capacity(height as usize);

    for row in 0..height as usize {
        grid_vector.push(Vec::new());
        for _col in 0..width {
            grid_vector[row].push(types::BG_COLOR);
        }
    }

    grid_vector
}
