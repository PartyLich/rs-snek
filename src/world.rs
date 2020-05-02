use rand::Rng;

use crate::{
    collision,
    snake::Snake,
    types::{self, Direction, Food, GameEvent, GameMode, Grid, SnakeEvent, WorldMap, FOOD_COLOR},
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

    /// The current ruleset
    game_mode: GameMode,

    /// Delay between gamestate updates. The simulation speed
    game_speed: u64,

    /// Simulation pause flag
    paused: bool,

    world_map: Option<WorldMap>,
}

impl Gamestate {
    pub fn new(rows: u32, cols: u32, game_mode: GameMode) -> Self {
        let world_map = match game_mode {
            GameMode::Map => Some(Self::generate_map(rows, cols)),
            _ => None,
        };

        Gamestate {
            grid: vec![],
            direction: Direction::Down,
            player: Snake::new(0, 0, None, Some(game_mode)),
            food: Food::new(rows / 2, cols / 2, Some(FOOD_COLOR), None),
            world_size: (rows, cols),
            score: 0,
            game_mode,
            game_speed: 200,
            paused: false,
            world_map,
        }
    }

    /// Create a new target object at a random location
    pub fn fresh_food(&mut self) {
        let mut row = rand::thread_rng().gen_range(0, self.grid.len());
        let mut col = rand::thread_rng().gen_range(0, self.grid[0].len());

        while self.grid[row][col] != types::BG_COLOR {
            row = rand::thread_rng().gen_range(0, self.grid.len());
            col = rand::thread_rng().gen_range(0, self.grid[0].len());
        }

        self.food = Food::new(row as u32, col as u32, Some(FOOD_COLOR), None);
    }

    /// Transition game state due to  player collision events
    pub fn handle_collision(&mut self, evt: &Option<SnakeEvent>) {
        let (rows, cols) = self.world_size;
        match evt {
            Some(evt @ SnakeEvent::Death) => {
                println!("event: {:?}", evt);
                // game over. restart
                println!("\n\tGAME OVER. restarting\n");
            }
            Some(SnakeEvent::Food) => {
                println!("event: {:?}", evt);
                self.score += 1;
                self.player.grow(&self.direction, cols as i32, rows as i32);
                self.fresh_food();
            }
            None => {
                self.player
                    .update_position(&self.direction, cols as i32, rows as i32);
            }
            _ => (),
        }
    }

    /// Change player movement direction according to input event
    pub fn handle_input(&mut self, input: Option<types::SnakeEvent>) {
        match input {
            Some(SnakeEvent::Input(d)) => {
                self.direction = d;
            }
            Some(SnakeEvent::Game(GameEvent::Pause)) => {
                self.toggle_pause();
            }
            _ => (),
        }
    }

    /// Updates the world state
    pub fn simulate(&mut self, dt: usize) -> Option<types::SnakeEvent> {
        if self.paused {
            return None;
        }

        let evt = collision::collision_check(&self.grid, &self.player, &self.direction);
        match evt {
            Some(SnakeEvent::Death) => {
                self.handle_collision(&evt);
                return evt;
            }
            Some(SnakeEvent::Food) => {
                if self.game_mode == GameMode::Tal {
                    self.game_speed = std::cmp::max(1, self.game_speed - 2);
                }
            }
            _ => {}
        }
        self.handle_collision(&evt);

        None
    }

    /// Returns the simulation speed
    pub fn speed(&self) -> u64 {
        if self.paused {
            return 0;
        }
        self.game_speed
    }

    /// Initialize grid
    ///
    /// Creates a width x height vector of `Cells`
    pub fn grid_init(&self) -> Grid {
        let (height, width) = self.world_size;
        let mut grid_vector = Vec::with_capacity(height as usize);

        for row in 0..height as usize {
            grid_vector.push(Vec::new());
            for _col in 0..width {
                grid_vector[row].push(types::BG_COLOR);
            }
        }

        if self.game_mode == GameMode::Map {
            let world_map = self.world_map.as_ref().unwrap();
            for (row, col) in world_map.walls.iter() {
                grid_vector[*row as usize][*col as usize] = world_map.color;
            }
        }

        grid_vector
    }

    /// Toggle the pause state
    fn toggle_pause(&mut self) {
        self.paused = !self.paused
    }

    fn generate_map(_rows: u32, _cols: u32) -> WorldMap {
        let walls = vec![
            // top
            (10, 12),
            (11, 12),
            (12, 12),
            (13, 12),
            (14, 12),
            (14, 11),
            (14, 10),
            (14, 9),
            (14, 8),
            // top
            (10, 24),
            (11, 24),
            (12, 24),
            (13, 24),
            (14, 24),
            (14, 25),
            (14, 26),
            (14, 27),
            (14, 28),
            // bottom
            (26, 24),
            (25, 24),
            (24, 24),
            (23, 24),
            (22, 24),
            (22, 25),
            (22, 26),
            (22, 27),
            (22, 28),
            // bottom
            (26, 12),
            (25, 12),
            (24, 12),
            (23, 12),
            (22, 12),
            (22, 11),
            (22, 10),
            (22, 9),
            (22, 8),
        ];

        WorldMap {
            color: types::WALL_COLOR,
            walls,
        }
    }
}
