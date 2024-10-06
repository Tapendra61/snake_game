use macroquad::color::Color;

pub struct Grid {
    width: f32,
    height: f32,
    cells: Vec<Vec<i32>>,
    grid_color: Color,
    draw_grid: bool,
}

impl Grid {
    pub fn new(width: f32, height: f32, grid_color: Color, draw_grid: bool) -> Self {
        Grid {
            width,
            height,
            cells: Vec::new(),
            grid_color,
            draw_grid,
        }.build()
    }

    fn build(mut self) -> Self {
        
        self
    }

    pub fn draw(&self) {
		
	}
}
