use sdl2::{event::Event, keyboard::Keycode, render::Canvas, ttf, video::Window, EventPump};

use crate::{gfx, types::GameMode};

#[derive(Debug, PartialEq, Clone)]
pub enum MenuEvent {
    Start(GameMode),
    Quit,
}

/// A selectable item in a `Menu`
#[derive(Debug, PartialEq)]
pub struct MenuItem {
    pub label: &'static str,
    pub event: MenuEvent,
}

impl MenuItem {
    pub fn new(label: &'static str, event: MenuEvent) -> Self {
        MenuItem { label, event }
    }
}

pub fn menu(
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

/// A user menu for selecting...things
#[derive(Debug, PartialEq)]
pub struct Menu {
    selection: usize,
    pub menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(menu_items: Vec<MenuItem>) -> Self {
        Menu {
            menu_items,
            selection: 0,
        }
    }

    /// Get the current selection index
    pub fn selection(&self) -> usize {
        self.selection
    }

    /// Modify `current` by `change` within the bounds `min` to `max` (inclusive)
    fn update_selection(&mut self, change: i32) {
        let (min, max) = (0, self.menu_items.len() - 1);
        let mut i = change + self.selection as i32;
        i = std::cmp::min(i, max as i32);
        i = std::cmp::max(i, min as i32);
        self.selection = i as usize;
    }

    /// Increment the menu selection
    pub fn inc_selection(&mut self) {
        self.update_selection(1);
    }

    /// Decrement the menu selection
    pub fn dec_selection(&mut self) {
        self.update_selection(-1);
    }

    /// Return domain event for current selection
    pub fn select_item(&self) -> &MenuEvent {
        &self.menu_items.get(self.selection).unwrap().event
    }
}
