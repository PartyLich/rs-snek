use super::{Cell, Position};

/// World map that lays out impassable terrain
#[derive(Debug, PartialEq)]
pub struct WorldMap {
    /// List of wall `Position`s
    pub walls: Vec<Position>,
    /// Wall color
    pub color: Cell,
}
