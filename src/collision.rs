/// Provides collision detection functions
use crate::{
    snake::Snake,
    types::{Cell, Direction, SnakeEvent, BG_COLOR, FOOD_COLOR},
};

/// Check for collisions in the player's *next* position and return appropriate `SnakeEvent`
pub fn collision_check(
    grid: &[Vec<Cell>],
    player: &Snake,
    direction: &Direction,
) -> Option<SnakeEvent> {
    let (row, col) = player.next_position(direction, grid.len() as i32, grid[0].len() as i32);

    match grid[row as usize][col as usize] {
        _x if _x == BG_COLOR => None,
        _x if _x == FOOD_COLOR => Some(SnakeEvent::Food),
        _ => Some(SnakeEvent::Death),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{self, Cell};

    #[test]
    fn collision_food() {
        let grid = vec![vec![Cell::RGB(0, 0, 0), FOOD_COLOR]];
        let expected = Some(SnakeEvent::Food);
        let actual = collision_check(&grid, &Snake::new(0, 0, None, None), &Direction::Right);
        assert_eq!(actual, expected);
    }

    #[test]
    fn collision_death() {
        let grid = vec![vec![Cell::RGB(0, 0, 0), types::SNAKE_COLOR]];
        let expected = Some(SnakeEvent::Death);
        let actual = collision_check(&grid, &Snake::new(0, 0, None, None), &Direction::Left);
        assert_eq!(actual, expected);
    }

    #[test]
    fn collision_none() {
        let grid = vec![vec![types::BG_COLOR, types::BG_COLOR]];
        let expected = None;
        let actual = collision_check(&grid, &Snake::new(0, 0, None, None), &Direction::Left);
        assert_eq!(actual, expected);
    }
}
