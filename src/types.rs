use sdl2::pixels::Color;

pub const GAME_NAME: &str = "rs-snek";

/// A single square on the game board
pub type Cell = Color;

/// The gameboard
pub type Grid = Vec<Vec<Cell>>;

/// The player's objective, an edible object that causes the player to grow
pub type Food = crate::snake::Snake;

pub const FOOD_COLOR: Cell = Cell::RGB(188, 13, 36);
pub const SNAKE_COLOR: Cell = Cell::RGB(141, 141, 139);
pub const BG_COLOR: Cell = Cell::RGB(42, 42, 42);

/// Valid directions of travel
#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    /// Returns a tuple of `(row, col)` values representing change in position for a `Direction`
    pub fn value(&self) -> (i32, i32) {
        match *self {
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
        }
    }

    /// Returns the `Direction` opposite (180 degrees) this `Direction`
    pub fn flip(&self) -> Self {
        match *self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_flip() {
        assert_eq!(Direction::Down.flip(), Direction::Up);
        assert_eq!(Direction::Up.flip(), Direction::Down);
        assert_eq!(Direction::Right.flip(), Direction::Left);
        assert_eq!(Direction::Left.flip(), Direction::Right);
    }
}
