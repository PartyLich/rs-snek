use sdl2::pixels::Color;

mod direction;
mod worldmap;

pub use direction::Direction;
pub use worldmap::WorldMap;

pub const GAME_NAME: &str = "rs-snek";

/// A single square on the game board
pub type Cell = Color;

/// The gameboard
pub type Grid = Vec<Vec<Cell>>;

/// The player's objective, an edible object that causes the player to grow
// pub type Food = crate::snake::Snake;

/// A location on the 2dimensional gameboard
pub type Position = (u32, u32);

pub type Entity = u32;

pub const FOOD_COLOR: Cell = Cell::RGB(188, 13, 36);
pub const SNAKE_COLOR: Cell = Cell::RGB(141, 141, 139);
pub const BG_COLOR: Cell = Cell::RGB(42, 42, 42);
pub const WALL_COLOR: Cell = Cell::RGB(0, 102, 102);
pub const EVIL_COLOR: Cell = Cell::RGB(255 - 141, 255 - 141, 255 - 139);
// Rgba([max - rgba[0], max - rgba[1], max - rgba[2], rgba[3]])

pub const TEXT_COLOR: Cell = Cell::RGB(225, 225, 225);
pub const TEXT_SELECTED: Cell = FOOD_COLOR;
pub const FONT_PATH: &str = "./resource/NotoSans-Regular.ttf";
// pub const FONT_PATH: &str = std::path::Path::new("../resource/NotoSans-Regular.ttf").to_str().unwrap();
pub const FONT_SIZE_SM: u16 = 12;
pub const FONT_SIZE_MD: u16 = 18;

#[derive(Debug, PartialEq)]
pub enum GameEvent {
    Pause,
    Menu,
}

/// Events that may affect the player
#[derive(Debug, PartialEq)]
pub enum SnakeEvent {
    /// Player target acquisition event
    Food,
    /// Player death event
    Death,
    /// Player input/control event
    Input(Direction),
    ///
    Game(GameEvent),
}

/// Available modes of play
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameMode {
    Normal,
    Tal,
    Map,
}

#[cfg(test)]
mod tests {
    // use super::*;
}
