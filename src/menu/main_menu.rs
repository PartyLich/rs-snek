use sdl2::{event::Event, keyboard::Keycode, render::Canvas, ttf, video::Window, EventPump};

use super::*;
use crate::{gfx, types::GameMode};

pub fn main_menu(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
    font: &ttf::Font,
) -> MenuEvent {
    let mut main_menu = Menu::new(vec![
        MenuItem::new("Normal Mode", MenuEvent::Start(GameMode::Normal)),
        MenuItem::new("Tal'ke Challenge", MenuEvent::Start(GameMode::Tal)),
        MenuItem::new("Quit", MenuEvent::Quit),
    ]);

    'menu: loop {
        for event in event_pump.poll_iter() {
            match event {
                // exit on escape key
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'menu MenuEvent::Quit,

                // movement keys
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => main_menu.inc_selection(),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => main_menu.dec_selection(),
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => return main_menu.select_item().clone(),

                _ => continue 'menu,
            }
        }

        // display frame
        gfx::render_menu(canvas, font, &main_menu);
        gfx::display_frame(canvas);
    }
}
