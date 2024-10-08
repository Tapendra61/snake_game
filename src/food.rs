use macroquad::color::Color;
use macroquad::shapes::*;

use crate::{grid::Grid, snake::Snake};

struct Food {
	pub pos: (i32, i32),
	pub color: Color,
}

pub struct FoodGenerator {
	food_container: Option<Food>,
	food_exists: bool,
}

impl FoodGenerator {
	pub fn new () -> Self {
		FoodGenerator { food_container:None, food_exists: false }
	}

	pub fn generate_food (&mut self, grid: &Grid, snake_handler: &Snake) {
		if !self.food_exists {
			let mut food = Food { pos: grid.generate_rand_coordinate(), color: Color::new(0.3, 0.0, 0.0, 1.0)};
			while food.pos ==  snake_handler.head_position(){
				food.pos = grid.generate_rand_coordinate();
			}

			self.food_container = Some(food);
			self.food_exists = true;
		}
	}

	pub fn draw_food (&self, grid: &Grid) {
		let pos = &self.food_container.as_ref().unwrap().pos;
		let draw_offset = (grid.cell_size / 2) - grid.cell_spacing;
		draw_rectangle(
			(grid.cells[pos.0 as usize][pos.1 as usize].0 - draw_offset) as f32,
			(grid.cells[pos.0 as usize][pos.1 as usize].1 - draw_offset) as f32,
			(grid.cell_size - (grid.cell_spacing * 2)) as f32,
			(grid.cell_size - (grid.cell_spacing * 2)) as f32,
			self.food_container.as_ref().unwrap().color,
		);
	}

}