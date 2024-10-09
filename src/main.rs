use macroquad::prelude::*;

pub mod grid;
pub mod snake;
pub mod window;
pub mod food;
pub mod collision;

#[macroquad::main("Snake")]
async fn main() {
    let window_props = window::Window::new(840.0, 840.0, Color::new(0.45, 0.45, 0.45, 1.0));
    request_new_screen_size(window_props.width, window_props.height);
    let grid_handler = grid::Grid::new(
        window_props.width,
        20,
        2,
        Color::new(0.6, 0.6, 0.6, 1.0),
        true,
    );
    let mut food_handler = food::FoodGenerator::new();
    let mut snake_handler = snake::Snake::new(&grid_handler);
    let mut collision_handler = collision::CollisionHandler::new();

    let mut draw_time:f32 = 0.0;
    loop {
        clear_background(window_props.clear_color);
        grid_handler.draw();
        snake_handler.handle_input();
        food_handler.generate_food(&grid_handler, &snake_handler);
        food_handler.draw_food(&grid_handler);
        collision_handler.check_collision(&mut food_handler, &mut snake_handler, &grid_handler);

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
