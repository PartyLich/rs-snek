use std::collections::VecDeque;

use crate::types::{Cell, Direction, Grid, SNAKE_COLOR};

pub type Position = (u32, u32);

/// Snake represents the player.
#[derive(Debug)]
pub struct Snake {
    /// Snake head cell info (color)
    pub cell: Cell,

    /// List of positions that comprise the Snake's body
    pub body: VecDeque<Position>,
}

impl Snake {
    /// Creates a new instance of `Snake`
    pub fn new(row: u32, col: u32, mut cell: Option<Cell>) -> Self {
        if cell.is_none() {
            cell = Some(SNAKE_COLOR);
        }
        let mut body = VecDeque::new();
        body.push_front((row, col));

        Self {
            cell: cell.unwrap(),
            body,
        }
    }

    /// Returns this `Snake`'s head location
    pub fn position(&self) -> &Position {
        self.body.front().unwrap()
    }

    /// Calculates a new position with direction values and the position from `Snake`
    pub fn next_position(&self, direction: &Direction, height: i32, width: i32) -> Position {
        let (dy, dx) = direction.value();
        let (row, col) = *self.position();
        let mut row = dy + row as i32;
        let mut col = dx + col as i32;

        // boundary checks
        if let Some(y) = wrap_index(0, height, row) {
            row = y;
        }
        if let Some(x) = wrap_index(0, width, col) {
            col = x;
        }

        // dont let snake turn back on itself
        match self.body.get(1) {
            Some(x) if *x == (row as u32, col as u32) => {
                self.next_position(&direction.flip(), height, width)
            }
            _ => (row as u32, col as u32),
        }
    }

    /// Update the position of this `Snake`
    pub fn update_position(&mut self, direction: &Direction, width: i32, height: i32) -> &Self {
        let position = self.next_position(direction, height, width);

        self.body.pop_back();
        self.body.push_front(position);

        self
    }

    /// Update the position of this `Snake` while extending its length
    pub fn grow(&mut self, direction: &Direction, width: i32, height: i32) -> &Self {
        let position = self.next_position(direction, width, height);
        self.body.push_front(position);

        self
    }

    /// Update grid to display this `Snake`
    pub fn render(&self, mut grid: Grid) -> Grid {
        for (row, col) in self.body.iter() {
            grid[*row as usize][*col as usize] = self.cell;
        }

        grid
    }
}

/// wrap an index within `lower` (inclusive) and `upper` (exclusive) bounds
fn wrap_index(lower: i32, upper: i32, i: i32) -> Option<i32> {
    match i {
        i if i < lower => Some(upper - 1),
        i if i >= upper => Some(lower),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_renders() {
        let grid = vec![vec![Cell::RGB(0, 0, 0)]];
        let expected = vec![vec![SNAKE_COLOR]];
        let actual = Snake::new(0, 0, None).render(grid);
        assert_eq!(actual, expected);
    }

    #[test]
    fn snake_default_color() {
        let expected = SNAKE_COLOR;
        let actual = Snake::new(0, 0, None).cell;
        assert_eq!(actual, expected);
    }

    #[test]
    fn snake_specified_color() {
        let expected = Cell::RGB(1, 2, 2);
        let actual = Snake::new(0, 0, Some(Cell::RGB(1, 2, 2))).cell;
        assert_eq!(actual, expected);
    }

    #[test]
    fn snake_position() {
        let expected = (10, 20);
        let actual = *Snake::new(10, 20, None).position();
        assert_eq!(actual, expected);
    }

    #[test]
    fn wraps_index_upper() {
        let expected = Some(0);
        let actual = wrap_index(expected.unwrap(), 10, 11);
        assert_eq!(actual, expected);
    }

    #[test]
    fn wraps_index_lower() {
        let expected = Some(9);
        let actual = wrap_index(0, 10, -5);
        assert_eq!(actual, expected);
    }
}
