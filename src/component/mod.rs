use std::collections::VecDeque;

use crate::types;

#[derive(Debug)]
pub struct DirectionComponent {
    pub direction: types::Direction,
}

impl DirectionComponent {
    pub fn flip(&self) -> Self {
        DirectionComponent {
            direction: self.direction.flip(),
        }
    }
}

#[derive(Debug)]
pub struct MeshComponent {
    /// List of positions that comprise the Entity
    pub mesh: VecDeque<types::Position>,
}

#[derive(Debug)]
pub struct CellComponent {
    /// Entity cell info (color)
    pub cell: types::Cell,
}

#[derive(Debug)]
pub struct ColliderComponent {}

#[derive(Debug)]
pub struct InputComponent {}

#[derive(Debug)]
pub struct MirrorComponent {}
