use crate::{
    component::MeshComponent,
    types::{Entity, GameMode, Position},
    world::Gamestate,
};

// pub mod collision;

/// Calculates a new position with direction values and the position from `Snake`
pub fn next_position(state: &mut Gamestate, entity: Entity) -> Result<Position, &str> {
    // get change in x and y
    let (dy, dx);
    if let Some(dir) = state.direction_components.get(entity as usize) {
        let (dy_, dx_) = dir.as_ref().unwrap().direction.value();
        dy = dy_;
        dx = dx_;
    } else {
        return Err("");
    }

    let neck;
    let (row, col) = if let Some(Some(mesh_component)) = state.mesh_components.get(entity as usize)
    {
        neck = mesh_component.mesh.get(1);

        *mesh_component
            .mesh
            .front()
            .expect("Mesh should have at least one Position")
    } else {
        return Err("");
    };

    let mut row = dy + row as i32;
    let mut col = dx + col as i32;
    let (height, width) = state.world_size;

    // boundary checks
    if let Some(y) = wrap_index(0, height as i32, row) {
        row = y;
        if state.game_mode == GameMode::Tal {
            col = mirror_index(0, width as i32, col);
        }
    }
    if let Some(x) = wrap_index(0, width as i32, col) {
        col = x;
        if state.game_mode == GameMode::Tal {
            row = mirror_index(0, height as i32, row);
        }
    }

    // dont let snake turn back on itself
    match neck {
        Some(x) if *x == (row as u32, col as u32) => {
            state.direction_components[entity as usize] = Some(
                state.direction_components[entity as usize]
                    .as_ref()
                    .unwrap()
                    .flip(),
            );
            next_position(state, entity)
        }
        _ => Ok((row as u32, col as u32)),
    }
}

/// Update the position of an `Entity`
pub fn update_position(state: &mut Gamestate, entity: Entity) -> Result<(), &str> {
    if let Some(component) = state.mesh_components[entity as usize].as_ref() {
        let mut mesh = component.mesh.clone();

        if let Ok(position) = next_position(state, entity) {
            mesh.pop_back();
            mesh.push_front(position);
            state.mesh_components[entity as usize] = Some(MeshComponent { mesh });
            return Ok(());
        }
    }

    Err("component not found for entity")
}

/// Update the position of an `Entity` while extending its length
pub fn grow(state: &mut Gamestate, entity: Entity) -> Result<(), &str> {
    if let Some(component) = state.mesh_components[entity as usize].as_ref() {
        let mut mesh = component.mesh.clone();

        if let Ok(position) = next_position(state, entity) {
            mesh.push_front(position);
            state.mesh_components[entity as usize] = Some(MeshComponent { mesh });
            return Ok(());
        }
    }

    Err("component not found for entity")
}

/// wrap an index within `lower` (inclusive) and `upper` (exclusive) bounds
fn wrap_index(lower: i32, upper: i32, i: i32) -> Option<i32> {
    match i {
        i if i < lower => Some(upper - 1),
        i if i >= upper => Some(lower),
        _ => None,
    }
}

/// mirror an index across the middle of the range  `lower` (inclusive) and `upper` (exclusive)
fn mirror_index(lower: i32, upper: i32, i: i32) -> i32 {
    let offset = (upper - 1) - i;
    lower + offset
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn mirrors_index() {
        let expected = 1;
        let actual = mirror_index(0, 4, 2);
        assert_eq!(actual, expected);
    }
}
