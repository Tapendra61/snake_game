use ::std::collections::*;
use macroquad::{color::Color, input::*, shapes::draw_rectangle};

use crate::grid::Grid;

#[derive(PartialEq, Clone, Copy)]
pub struct Link {
    pos: (i32, i32),
    link_color: Color,
}

pub struct Snake {
    links: LinkedList<Link>,
    pub move_direction: (i32, i32),
}

impl Snake {

	//Creates a new snake and initializes the head
    pub fn new(grid: &Grid) -> Self {
        Snake {
            links: LinkedList::new(),
            move_direction: (0, 0),
        }
        .initialize_head(grid)
    }

	//Initialize the head of the snake
    fn initialize_head(mut self, grid: &Grid) -> Self {
        let head = Link {
            pos: grid.generate_rand_coordinate(),
            link_color: Color::new(0.2, 0.2, 0.2, 1.0),
        };

        self.links.push_front(head);
        self
    }

	//Add new links/body parts to the snake
	pub fn add_link () {

	}

	//Returns the position of snake's head
	pub fn head_position (&self) -> (i32, i32) {
		self.links.front().unwrap().pos
	}

	//Update movement direction based on Key Press
	pub fn handle_input (&mut self) {
		if is_key_pressed(KeyCode::W) {
			if self.move_direction != (1, 0) {
				self.move_direction = (-1, 0);
			}
		}
		if is_key_pressed(KeyCode::S) {
			if self.move_direction != (-1, 0) {
				self.move_direction = (1, 0);
			}
		}
		if is_key_pressed(KeyCode::D) {
			if self.move_direction != (0, -1) {
				self.move_direction = (0, 1);
			}
		}
		if is_key_pressed(KeyCode::A) {
			if self.move_direction != (0, 1) {
				self.move_direction = (0, -1);
			}
		}
	}

	//Updates/Moves the snake on the grid based on the direction of the snake
	pub fn update_position(&mut self, gird: &Grid) {
		let front = self.links.front().copied().unwrap();
		for link in &mut self.links {
			if *link == front {
				link.pos.0 = (link.pos.0 + self.move_direction.0).rem_euclid(gird.cells.len() as i32);
				link.pos.1 = (link.pos.1 + self.move_direction.1).rem_euclid(gird.cells.len() as i32);
			}
		}
	}

	//Draws the snake on the grid
    pub fn draw(&self, grid: &Grid) {
        let draw_offset = (grid.cell_size / 2) - grid.cell_spacing;
        for link in &self.links {
            draw_rectangle(
                (grid.cells[link.pos.0 as usize][link.pos.1 as usize].0 - draw_offset) as f32,
                (grid.cells[link.pos.0 as usize][link.pos.1 as usize].1 - draw_offset) as f32,
                (grid.cell_size - (grid.cell_spacing * 2)) as f32,
                (grid.cell_size - (grid.cell_spacing * 2)) as f32,
                link.link_color,
            );
        }
    }
}
