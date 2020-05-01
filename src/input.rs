use sdl2::keyboard::Keycode;

use crate::types::{Direction, SnakeEvent};

/// Maps keycodes to player movement direction
// TODO: maybe just use an actual Map?
pub fn map_key_input(keycode: Keycode) -> Option<SnakeEvent> {
    match keycode {
        Keycode::Up | Keycode::W => Some(SnakeEvent::Input(Direction::Up)),
        Keycode::Left | Keycode::A => Some(SnakeEvent::Input(Direction::Left)),
        Keycode::Right | Keycode::D => Some(SnakeEvent::Input(Direction::Right)),
        Keycode::Down | Keycode::S => Some(SnakeEvent::Input(Direction::Down)),
        _ => None,
    }
}
