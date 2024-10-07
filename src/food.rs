use macroquad::color::Color;

pub struct Food {
	pub pos: (i32, i32),
	pub color: Color,
}

pub struct FoodHandler {
	food_exists: bool,
}

