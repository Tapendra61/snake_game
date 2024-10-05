use macroquad::prelude::*;

pub mod window;

#[macroquad::main("Snake")]
async fn main() {
	let window_props = window::Window::new( 750.0, 750.0, Color::new(0.8, 0.8, 0.8, 1.0));
	request_new_screen_size(window_props.width, window_props.height);
	let mut fps = 0;
	loop {
		clear_background(window_props.clear_color);
		fps = get_fps();

		println!("fps: {fps}");
		next_frame().await
	}
}
