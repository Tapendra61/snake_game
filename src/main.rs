use macroquad::prelude::*;

pub mod grid;
pub mod snake;
pub mod window;
pub mod food;

#[macroquad::main("Snake")]
async fn main() {
    let window_props = window::Window::new(720.0, 720.0, Color::new(0.45, 0.45, 0.45, 1.0));
    request_new_screen_size(window_props.width, window_props.height);
    let grid_handler = grid::Grid::new(
        window_props.width,
        20,
        2,
        Color::new(0.6, 0.6, 0.6, 1.0),
        true,
    );

    let mut snake_handler = snake::Snake::new(&grid_handler);
    let mut draw_time:f32 = 0.0;
    loop {
        clear_background(window_props.clear_color);
        grid_handler.draw();
        snake_handler.handle_input();

        if draw_time >= 0.125 {
            snake_handler.update_position(&grid_handler);
            draw_time = 0.0;
        }else {
            draw_time += get_frame_time();
        }
        
        snake_handler.draw(&grid_handler);
        next_frame().await
    }
}
