use sdl2::keyboard::Keycode;

use crate::{
    component::DirectionComponent,
    types::{Direction, GameEvent, SnakeEvent},
    world::Gamestate,
};

/// Maps keycodes to player movement direction
// TODO: maybe just use an actual Map?
pub fn map_key_input(keycode: Keycode) -> Option<SnakeEvent> {
    match keycode {
        Keycode::Up | Keycode::W => Some(SnakeEvent::Input(Direction::Up)),
        Keycode::Left | Keycode::A => Some(SnakeEvent::Input(Direction::Left)),
        Keycode::Right | Keycode::D => Some(SnakeEvent::Input(Direction::Right)),
        Keycode::Down | Keycode::S => Some(SnakeEvent::Input(Direction::Down)),
        Keycode::P => Some(SnakeEvent::Game(GameEvent::Pause)),
        _ => None,
    }
}

/// Change player movement direction according to input event
pub fn handle_input(state: &mut Gamestate, keycode: Keycode) {
    let event = map_key_input(keycode);

    match event {
        Some(SnakeEvent::Input(direction)) => {
            state.direction_components[state.player as usize] =
                Some(DirectionComponent { direction });

            // mirror input
            let mirror_entities = state
                .mirror_components
                .iter()
                .enumerate()
                .filter(|(_, opt)| opt.is_some())
                .map(|(e, _)| e);
            for e in mirror_entities {
                state.direction_components[e as usize] = Some(DirectionComponent {
                    direction: direction.flip(),
                });
            }
        }
        Some(SnakeEvent::Game(GameEvent::Pause)) => {
            state.toggle_pause();
        }
        _ => (),
    }
}
