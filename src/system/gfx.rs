use sdl2::{rect::Rect, render::Canvas, ttf, video::Window, EventPump};

use crate::{
    menu,
    types::{self, Cell, Entity},
    world::Gamestate,
};

/// Initialize the canvas
pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().expect("Failed to init SDL");
    let video_subsystem = sdl_context.video().expect("Failed to init video subsystem");

    let window = video_subsystem
        .window(types::GAME_NAME, width + 1, height + 1)
        .position_centered()
        .build()
        .expect("Failed to build window");
    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to get canvas from window");
    let event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL2 event pump");

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
    cell: Cell,
    cell_width: u32,
) {
    let cell_height = cell_width; // All cells are square
    let x = cell_width * col;
    let y = cell_width * row;

    renderer.set_draw_color(cell);
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
            let cell = grid[row as usize][col as usize];
            display_cell(renderer, row, col, cell, cell_width);
        }
    }
}

/// Move the draw buffer to the display (ie swap back buffer to front)
pub fn display_frame(renderer: &mut Canvas<Window>) {
    renderer.present();
}

/// Initialize a TrueType Font
// lifetime specifiers from https://users.rust-lang.org/t/rust-sdl2-does-not-live-long-enought-fighting-the-borrow-checher/9464/8
pub fn init_font<'a, 'b>(
    ttf_context: &'a ttf::Sdl2TtfContext,
    path: &'static str,
    size: u16,
) -> ttf::Font<'a, 'b> {
    ttf_context.load_font(path, size).unwrap()
}

/// Display a text `&str` at the top center of the window
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

/// Render a `Menu`
pub fn render_menu(renderer: &mut Canvas<Window>, font: &ttf::Font, menu: &menu::Menu) {
    clear_frame(renderer);

    // render each menu item
    let (mut x, mut y) = renderer.window().size();
    x /= 2;
    y /= 4;
    let vertical_step = font.height() as u32;

    for (i, item) in menu.menu_items.iter().enumerate() {
        let selected = i == menu.selection();
        render_menu_item(renderer, font, item, selected, x as i32, y as i32);
        y += vertical_step;
    }
}

/// Render a `MenuItem`
pub fn render_menu_item(
    renderer: &mut Canvas<Window>,
    font: &ttf::Font,
    item: &menu::MenuItem,
    selected: bool,
    x: i32,
    y: i32,
) {
    let (color, text) = if selected {
        (types::TEXT_SELECTED, format!("> {}", item.label))
    } else {
        (types::TEXT_COLOR, item.label.to_string())
    };

    let surface = font.render(&text).solid(color).unwrap();
    let width = surface.width();
    let height = surface.height();

    let texture_creator = renderer.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();

    renderer
        .copy(&texture, None, Rect::from_center((x, y), width, height))
        .unwrap();
}

/// Update grid to display this `Snake`
pub fn render_entity(state: &mut Gamestate, entity: Entity) {
    // get entity data
    let mesh_component = match state.mesh_components[entity as usize].as_ref() {
        Some(m) => m,
        _ => return,
    };
    let cell_component = match state.cell_components[entity as usize].as_ref() {
        Some(c) => c,
        _ => return,
    };

    for (row, col) in mesh_component.mesh.iter() {
        state.grid[*row as usize][*col as usize] = cell_component.cell;
    }
}
