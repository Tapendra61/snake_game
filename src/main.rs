use macroquad::prelude::*;

pub mod grid;
pub mod window;

#[macroquad::main("Snake")]
async fn main() {
    let window_props = window::Window::new(720.0, 720.0, Color::new(0.5, 0.5, 0.5, 1.0));
    let grid_handler = grid::Grid::new(
        window_props.width,
        20,
        2,
        Color::new(0.6, 0.6, 0.6, 1.0),
        true,
    );

    println!("{:?}", grid_handler.cells);

    request_new_screen_size(window_props.width, window_props.height);

    loop {
        clear_background(window_props.clear_color);
		grid_handler.draw();
        next_frame().await
    }
}
