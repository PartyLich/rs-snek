/// Provides collision detection functions
use crate::{
    system,
    types::{Entity, SnakeEvent, BG_COLOR, FOOD_COLOR},
    world::Gamestate,
};

/// Check for collisions in the player's *next* position and return appropriate `SnakeEvent`
pub fn collision_check(state: &mut Gamestate, entity: Entity) -> Option<SnakeEvent> {
    if let Ok((row, col)) = system::motion::next_position(state, entity) {
        match state.grid[row as usize][col as usize] {
            _x if _x == BG_COLOR => return None,
            _x if _x == FOOD_COLOR => return Some(SnakeEvent::Food),
            _ => return Some(SnakeEvent::Death),
        }
    }

    None
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::types::{self, Cell};

//     #[test]
//     fn collision_food() {
//         let grid = vec![vec![Cell::RGB(0, 0, 0), FOOD_COLOR]];
//         let expected = Some(SnakeEvent::Food);
//         let actual = collision_check(&grid, &Snake::new(0, 0, None, None), &Direction::Right);
//         assert_eq!(actual, expected);
//     }

//     #[test]
//     fn collision_death() {
//         let grid = vec![vec![Cell::RGB(0, 0, 0), types::SNAKE_COLOR]];
//         let expected = Some(SnakeEvent::Death);
//         let actual = collision_check(&grid, &Snake::new(0, 0, None, None), &Direction::Left);
//         assert_eq!(actual, expected);
//     }

//     #[test]
//     fn collision_none() {
//         let grid = vec![vec![types::BG_COLOR, types::BG_COLOR]];
//         let expected = None;
//         let actual = collision_check(&grid, &Snake::new(0, 0, None, None), &Direction::Left);
//         assert_eq!(actual, expected);
//     }
// }
