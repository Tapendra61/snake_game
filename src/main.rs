use macroquad::prelude::*;

pub mod grid;
pub mod window;

#[macroquad::main("Snake")]
async fn main() {
    let window_props = window::Window::new(750.0, 750.0, Color::new(0.8, 0.8, 0.8, 1.0));
    let grid_handler = grid::Grid::new(
        window_props.width,
        window_props.height,
        Color::new(0.3, 0.3, 0.3, 1.0),
        true,
    );

    request_new_screen_size(window_props.width, window_props.height);

    loop {
        clear_background(window_props.clear_color);
		grid_handler.draw();
        next_frame().await
    }
}
