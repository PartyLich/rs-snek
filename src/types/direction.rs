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
