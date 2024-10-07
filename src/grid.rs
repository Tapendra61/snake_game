use macroquad::color::Color;
use macroquad::shapes::*;
use rand::Rng;

pub struct Grid {
    grid_size: f32,
    pub cells: Vec<Vec<(i32, i32)>>,
    cell_count: i32,
    pub cell_size: i32,
    pub cell_spacing: i32,
    grid_color: Color,
    draw_grid: bool,
}

impl Grid {

    //Initializes and returns a grid
    pub fn new(
        grid_size: f32,
        cell_count: i32,
        cell_spacing: i32,
        grid_color: Color,
        draw_grid: bool,
    ) -> Self {
        Grid {
            grid_size,
            cells: Vec::new(),
            cell_count,
            cell_size: 0,
            cell_spacing,
            grid_color,
            draw_grid,
        }
        .build()
    }

    //generate the co-ordinates for the actual grid and stores them in the cells property
    fn build(mut self) -> Self {
        self.cell_size = (self.grid_size / self.cell_count as f32) as i32;

        let mut pos: (i32, i32) = (0, 0);
        for j in 0..self.cell_count {
            let mut row: Vec<(i32, i32)> = Vec::new();
            for i in 0..self.cell_count {
                pos.0 = ((i * self.cell_size) + self.cell_size) - (self.cell_size / 2);
                pos.1 = ((j * self.cell_size) + self.cell_size) - (self.cell_size / 2);

                row.push(pos);
            }
            self.cells.push(row);
        }
        self
    }

    //Returns a random co-ordinate on the grid
    pub fn generate_rand_coordinate (&self) -> (i32, i32) {
        let mut rand_pos = (0, 0);

        let mut rng = rand::thread_rng();
        rand_pos.0 = rng.gen_range(0..self.cells.len()) as i32;
        rand_pos.1 = rng.gen_range(0..self.cells.len()) as i32;

        rand_pos
    }

    //Draw the grid on screen
    pub fn draw(&self) {
        if self.draw_grid {
            let draw_offset = (self.cell_size / 2) - self.cell_spacing;
            for row in &self.cells {
                for position in row {
                    draw_rectangle(
                        (position.0 - draw_offset) as f32,
                        (position.1 - draw_offset) as f32,
                        (self.cell_size - (self.cell_spacing * 2)) as f32,
                        (self.cell_size - (self.cell_spacing * 2)) as f32,
                        self.grid_color,
                    );
                }
            }
        }
    }

    //Output the grid on stdout
    pub fn debug_grid (&self) {
        for row in &self.cells {
            for cells in row {
                print!("{:?} ", cells);
            }
            print!("\n");
        }
    }
}
