use sdl2::{rect::Rect, render::Canvas, ttf, video::Window, EventPump};

use crate::types::{self, Cell, Grid};

/// Initialize the canvas
pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().expect("Failed to init SDL");
    let video_subsystem = sdl_context.video().expect("Failed to init video subsystem");

    let window = video_subsystem
        .window(types::GAME_NAME, width + 1, height + 1)
        .position_centered()
        .build()
        .expect("Failed to build window");

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    clear_frame(&mut canvas);
    display_frame(&mut canvas);

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

/// Clear the current draw buffer
fn clear_frame(renderer: &mut Canvas<Window>) {
    renderer.set_draw_color(types::BG_COLOR);
    renderer.clear();
}

/// Render a single cell from the grid
///
/// Translates from game space to pixel space
pub fn display_cell(
    renderer: &mut Canvas<Window>,
    row: u32,
    col: u32,
    cell: &Cell,
    cell_width: u32,
) {
    let cell_height = cell_width; // All cells are square
    let x = cell_width * col;
    let y = cell_width * row;

    renderer.set_draw_color(*cell);
    if let Err(e) = renderer.fill_rect(Rect::new(x as i32, y as i32, cell_width, cell_height)) {
        println!("{}", e)
    }
}

/// Render a `Grid` on the current draw buffer
// pub fn render_frame(renderer: &mut Canvas<Window>, grid: &Grid, cell_width: u32) {
pub fn render_frame(renderer: &mut Canvas<Window>, grid: &[Vec<Cell>], cell_width: u32) {
    clear_frame(renderer);

    for row in 0..grid.len() as u32 {
        for col in 0..grid[0].len() as u32 {
            let cell = &grid[row as usize][col as usize];
            display_cell(renderer, row, col, cell, cell_width);
        }
    }
}

/// Move the draw buffer to the display (ie swap back buffer to front)
pub fn display_frame(renderer: &mut Canvas<Window>) {
    renderer.present();
}

// lifetime specifiers from https://users.rust-lang.org/t/rust-sdl2-does-not-live-long-enought-fighting-the-borrow-checher/9464/8
pub fn init_font<'a, 'b>(
    ttf_context: &'a ttf::Sdl2TtfContext,
    path: &'static str,
) -> ttf::Font<'a, 'b> {
    let font = ttf_context.load_font(path, types::FONT_SIZE_SM).unwrap();
    font
}

pub fn render_text(font: &ttf::Font, renderer: &mut Canvas<Window>, text: &str) {
    let surface = font.render(text).blended(types::TEXT_COLOR).unwrap();
    let width = surface.width();
    let height = surface.height();

    let (window_width, _) = renderer.window().size();
    let text_x = window_width / 2;
    let text_y = height / 2 + 5;
    let text_center = (text_x as i32, text_y as i32);

    let texture_creator = renderer.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();

    renderer
        .copy(
            &texture,
            None,
            Rect::from_center(text_center, width, height),
        )
        .unwrap();
}
