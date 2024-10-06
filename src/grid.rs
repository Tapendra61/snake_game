use macroquad::color::Color;
use macroquad::shapes::*;

pub struct Grid {
    grid_size: f32,
    pub cells: Vec<Vec<(i32, i32)>>,
    cell_count: i32,
    cell_size: i32,
    grid_color: Color,
    draw_grid: bool,
}

impl Grid {
    pub fn new(grid_size: f32, cell_count: i32, grid_color: Color, draw_grid: bool) -> Self {
        Grid {
            grid_size,
            cells: Vec::new(),
            cell_count,
            cell_size: 0,
            grid_color,
            draw_grid,
        }.build()
    }

    fn build(mut self) -> Self {
        self.cell_size = (self.grid_size / self.cell_count as f32) as i32;
        
        let mut pos:(i32, i32) = (0, 0);
        for j in 0..self.cell_count {
            let mut row: Vec<(i32, i32)> = Vec::new();
            for i in 0..self.cell_count {
                pos.0 = ((i * self.cell_size) + self.cell_size) - (self.cell_size/2);
                pos.1 = ((j * self.cell_size) + self.cell_size) - (self.cell_size/2);

                row.push(pos);
            }
            self.cells.push(row);
        }
        self
    }

    pub fn draw(&self) {
		if self.draw_grid {
            for row in &self.cells {
                for position in row {
                    draw_rectangle((position.0 - 4) as f32, (position.1 - 4) as f32, 8.0, 8.0, self.grid_color);
                }
            }
        }
	}
}
