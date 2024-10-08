use macroquad::color::Color;

use crate::grid::Grid;

pub struct Food {
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

	pub fn generate_food (&mut self, grid_handler: &Grid) {
		if !self.food_exists {
			let food = Food { pos: grid_handler.generate_rand_coordinate(), color: Color::new(0.3, 0.0, 0.0, 1.0)};
			self.food_container = Some(food);
		}
	}
}