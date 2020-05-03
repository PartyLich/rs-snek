use super::*;

// A `Mapper` implementation with hardcoded/in-mem map(s)
pub struct MemMapper {}

impl Mapper for MemMapper {
    fn load_map(&self) -> Result<WorldMap, Box<dyn Error>> {
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

        Ok(WorldMap {
            color: types::WALL_COLOR,
            walls,
        })
    }
}
