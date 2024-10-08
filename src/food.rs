use macroquad::color::Color;

pub struct Food {
	pub pos: (i32, i32),
	pub color: Color,
}


pub struct FoodGenerator {
	food_exists: bool,
}

impl FoodGenerator {
	pub fn new () -> Self {
		FoodGenerator { food_exists: false }
	}
}