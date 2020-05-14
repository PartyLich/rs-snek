use std::collections::VecDeque;

use rand::Rng;

use crate::{
    component::*,
    map::Mapper,
    system::{self},
    types::{self, Direction, Entity, GameMode, Grid, SnakeEvent, WorldMap},
};

type ComponentManager<T> = Vec<Option<T>>;

/// The state of the gameworld
#[derive(Debug)]
pub struct Gamestate {
    pub grid: Grid,
    pub world_size: (u32, u32),

    /// The player's direction of travel
    pub direction: Direction,

    // Component managers
    pub direction_components: ComponentManager<DirectionComponent>,
    pub mesh_components: ComponentManager<MeshComponent>,
    pub collider_components: ComponentManager<ColliderComponent>,
    pub cell_components: ComponentManager<CellComponent>,
    pub input_components: ComponentManager<InputComponent>,
    pub mirror_components: ComponentManager<MirrorComponent>,

    /// The player's avatar
    pub player: Entity,

    /// A shadow of the player avatar. The player's passive nemesis
    pub evil: Entity,

    /// The player's objective
    pub food: Entity,

    /// The player's score
    pub score: usize,

    /// The current ruleset
    pub game_mode: GameMode,

    /// Delay between gamestate updates. The simulation speed
    game_speed: u64,

    /// Simulation pause flag
    pub paused: bool,

    /// Optional world map that lays out impassable terrain
    world_map: Option<WorldMap>,
}

impl Gamestate {
    /// Create a new instance of GameState
    pub fn new(rows: u32, cols: u32, game_mode: GameMode, mapper: Option<Box<dyn Mapper>>) -> Self {
        let world_map = match game_mode {
            GameMode::Map => Some(
                mapper
                    .unwrap_or_else(|| {
                        panic!("Mapper must be supplied when game mode is {:?}", game_mode)
                    })
                    .load_map()
                    .unwrap_or_else(|e| panic!("Failed to load map: {}", e)),
            ),
            _ => None,
        };

        let mut this = Gamestate {
            grid: vec![],
            direction: Direction::Down,
            // player: Snake::new(0, 0, None, Some(game_mode)),
            // players: Vec::new(),
            // evil: Snake::new(35, 35, Some(types::EVIL_COLOR), Some(game_mode)),
            // food: Food::new(rows / 2, cols / 2, Some(FOOD_COLOR), None),
            world_size: (rows, cols),
            score: 0,
            game_mode,
            game_speed: 200,
            paused: false,
            world_map,

            direction_components: Vec::new(),
            mesh_components: Vec::new(),
            collider_components: Vec::new(),
            cell_components: Vec::new(),
            input_components: Vec::new(),
            mirror_components: Vec::new(),

            player: 0,
            evil: 0,
            food: 0,
        };
        this.make_entities(rows, cols);

        this
    }

    /// Create a new target object at a random location
    fn fresh_food(&mut self) {
        let mut row = rand::thread_rng().gen_range(0, self.grid.len());
        let mut col = rand::thread_rng().gen_range(0, self.grid[0].len());

        while self.grid[row][col] != types::BG_COLOR {
            row = rand::thread_rng().gen_range(0, self.grid.len());
            col = rand::thread_rng().gen_range(0, self.grid[0].len());
        }
        self.mesh_components[self.food as usize].take();

        let mut new_pos = VecDeque::with_capacity(1);
        new_pos.push_back((row as u32, col as u32));

        self.mesh_components[self.food as usize] = Some(MeshComponent { mesh: new_pos });
    }

    /// Transition game state due to  player collision events
    fn handle_collision(&mut self, evt: &Option<SnakeEvent>) {
        match evt {
            Some(evt @ SnakeEvent::Death) => {
                println!("event: {:?}", evt);
                // game over. restart
                println!("\n\tGAME OVER. restarting\n");
            }
            Some(SnakeEvent::Food) => {
                println!("event: {:?}", evt);
                self.score += 1;
                system::motion::grow(self, self.player).expect("Grow update failed");
                system::motion::grow(self, self.evil).expect("Grow update failed");
                self.fresh_food();
            }
            None => {
                system::motion::update_position(self, self.player).expect("Position update failed");
                system::motion::update_position(self, self.evil).expect("Position update failed");
            }
            _ => (),
        }
    }

    /// Updates the world state
    pub fn simulate(&mut self, _dt: usize) -> Option<types::SnakeEvent> {
        if self.paused {
            return None;
        }

        let evt = system::collision::collision_check(self, self.player);
        self.handle_collision(&evt);
        match evt {
            Some(SnakeEvent::Death) => {
                return evt;
            }
            Some(SnakeEvent::Food) => {
                if self.game_mode == GameMode::Tal {
                    self.game_speed = std::cmp::max(1, self.game_speed - 2);
                }
            }
            _ => (),
        }

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
        let mut grid_vector = vec![vec![types::BG_COLOR; width as usize]; height as usize];

        if self.game_mode == GameMode::Map {
            let world_map = self.world_map.as_ref().unwrap();
            for (row, col) in world_map.walls.iter() {
                grid_vector[*row as usize][*col as usize] = world_map.color;
            }
        }

        grid_vector
    }

    /// Toggle the pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused
    }

    fn create_entity(&mut self) -> Entity {
        self.direction_components.push(None);
        self.mesh_components.push(None);
        self.collider_components.push(None);
        self.cell_components.push(None);
        self.input_components.push(None);
        self.mirror_components.push(None);

        (self.mesh_components.len() - 1) as Entity
    }

    /// Create a snake entity and its components
    fn make_snake(
        &mut self,
        cell: types::Cell,
        position: types::Position,
        direction: types::Direction, // entity: Entity,
    ) -> Entity {
        let entity = self.create_entity();

        // add components
        self.cell_components[entity as usize] = Some(CellComponent { cell });
        let mut mesh = VecDeque::new();
        mesh.push_back(position);
        self.mesh_components[entity as usize] = Some(MeshComponent { mesh });
        self.direction_components[entity as usize] = Some(DirectionComponent { direction });

        entity
    }

    fn make_food(&mut self, position: types::Position) -> Entity {
        let entity = self.create_entity();

        // add components
        self.cell_components[entity as usize] = Some(CellComponent {
            cell: types::FOOD_COLOR,
        });
        let mut mesh = VecDeque::new();
        mesh.push_back(position);
        self.mesh_components[entity as usize] = Some(MeshComponent { mesh });

        entity
    }

    /// Create ALL the game entities. Fortunately there's only a few, so this very procedural biz is
    /// tolerable I hope.
    fn make_entities(&mut self, rows: u32, cols: u32) {
        let direction = types::Direction::Down;

        // player avatar
        self.player = self.make_snake(types::SNAKE_COLOR, (0, 0), direction);
        self.input_components[self.player as usize] = Some(InputComponent {});
        self.collider_components[self.player as usize] = Some(ColliderComponent {});

        // evil mirror snake
        self.evil = self.make_snake(types::EVIL_COLOR, (35, 35), direction.flip());
        self.mirror_components[self.evil as usize] = Some(MirrorComponent {});

        // initial target
        self.food = self.make_food((rows / 2, cols / 2));
    }
}
