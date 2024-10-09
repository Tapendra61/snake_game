use crate::{food::FoodGenerator, grid::Grid, snake::Snake};

pub struct CollisionHandler;

impl CollisionHandler {
	pub fn new() -> CollisionHandler {
		CollisionHandler
	}
	pub fn check_collision(&self,food_handler: &mut FoodGenerator, snake_handler: &mut Snake, grid_handler: &Grid) {
		let food_pos = food_handler.food_container.as_ref().unwrap().pos;
		let head_pos  = snake_handler.head_position();
		
		if food_pos == head_pos {
			food_handler.destroy_food();
			snake_handler.add_link(grid_handler);
		}
	}
}