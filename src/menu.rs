use crate::types::GameMode;

mod main_menu;
pub use main_menu::main_menu;

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

/// A user menu for selecting...things
#[derive(Debug, PartialEq)]
pub struct Menu {
    selection: usize,
    pub menu_items: Vec<MenuItem>,
}

impl Menu {
    /// Create a new `Menu` containing the provided `MenuItems`
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
