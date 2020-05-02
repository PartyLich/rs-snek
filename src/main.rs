use std::{thread, time};

use sdl2::{event::Event, keyboard::Keycode, render::Canvas, ttf, video::Window};

use rs_snake::{
    gfx, input,
    menu::{self, MenuEvent},
    types::{self, GameMode, SnakeEvent},
    world::{self, Gamestate},
};

fn main() {
    const CANVAS_WIDTH: u32 = 720_u32;
    const CANVAS_HEIGHT: u32 = CANVAS_WIDTH;
    const ROWS: u32 = 36;
    // const COLS: u32 = ROWS;

    let cell_width = CANVAS_WIDTH / ROWS;

    let (mut canvas, mut event_pump) = gfx::init(CANVAS_WIDTH, CANVAS_HEIGHT);

    // fonts. apparently i have to keep the ttf context on the stack, can't move it, etc
    let ttf_context = ttf::init().unwrap();
    let menu_font = gfx::init_font(&ttf_context, types::FONT_PATH, types::FONT_SIZE_MD);
    let game_font = gfx::init_font(&ttf_context, types::FONT_PATH, types::FONT_SIZE_SM);

    'menu: loop {
        match menu::main_menu(&mut canvas, &mut event_pump, &menu_font) {
            MenuEvent::Start(game_mode) => run_game(
                &mut canvas,
                &mut event_pump,
                &game_font,
                cell_width,
                game_mode,
            ),
            MenuEvent::Quit => break 'menu,
        }
    }
}

fn run_game(
    canvas: &mut Canvas<Window>,
    event_pump: &mut sdl2::EventPump,
    font: &ttf::Font,
    cell_width: u32,
    game_mode: GameMode,
) {
    const ROWS: u32 = 36;
    const COLS: u32 = ROWS;
    let mut game_state = Gamestate::new(ROWS, COLS, game_mode);

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                // exit on escape key
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,

                // pause on P key
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    game_state.toggle_pause();
                    continue 'game;
                }

                // movement keys
                Event::KeyDown {
                    keycode: Some(k), ..
                } => game_state.handle_input(input::map_key_input(k)),

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
        gfx::render_frame(canvas, &game_state.grid, cell_width);
        gfx::render_text(font, canvas, &format!("Score: {}", game_state.score));
        gfx::display_frame(canvas);

        // update world state
        if let Some(SnakeEvent::Death) = game_state.simulate(1) {
            thread::sleep(time::Duration::from_millis(800));
            break 'game;
        }

        thread::sleep(time::Duration::from_millis(game_state.speed()));
    }
}
