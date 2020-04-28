use std::{thread, time};

use sdl2::{event::Event, keyboard::Keycode};

use rs_snake::{
    collision, gfx,
    types::{Direction, SnakeEvent},
    world::{self, Gamestate},
};

fn main() {
    const CANVAS_WIDTH: u32 = 720_u32;
    const CANVAS_HEIGHT: u32 = CANVAS_WIDTH;
    const ROWS: u32 = 36;
    const COLS: u32 = ROWS;

    let cell_width = CANVAS_WIDTH / ROWS;

    let (mut canvas, mut event_pump) = gfx::init(CANVAS_WIDTH, CANVAS_HEIGHT);
    let mut game_state = Gamestate::new(ROWS, COLS);

    thread::spawn(move || {});

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                // exit on escape key
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,

                // movement keys
                Event::KeyDown {
                    keycode: Some(k), ..
                } => set_direction(&mut game_state, k),

                _ => continue 'game,
            }
        }

        // fresh state for this game step
        game_state.grid = world::grid_init(ROWS, ROWS);
        // update grid with position of snake
        game_state.grid = game_state.player.render(game_state.grid);
        // update grid with position of food
        game_state.grid = game_state.food.render(game_state.grid);

        // display frame
        gfx::render_frame(&mut canvas, &game_state.grid, cell_width);
        gfx::display_frame(&mut canvas);

        // update position of snake
        let evt =
            collision::collision_check(&game_state.grid, &game_state.player, &game_state.direction);
        game_state.handle_collision(evt);

        thread::sleep(time::Duration::from_millis(200));
    }
}

/// Maps keycodes to player movement direction
// TODO: move some version of this into the gamestate impl. use domain events not tied to keycode
fn set_direction(game_state: &mut Gamestate, keycode: Keycode) {
    match keycode {
        Keycode::Up | Keycode::W => game_state.direction = Direction::Up,
        Keycode::Left | Keycode::A => game_state.direction = Direction::Left,
        Keycode::Right | Keycode::D => game_state.direction = Direction::Right,
        Keycode::Down | Keycode::S => game_state.direction = Direction::Down,
        _ => (),
    }
}
