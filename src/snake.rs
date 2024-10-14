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
    fn initialize_head(mut self, grid_handler: &Grid) -> Self {
        let head = Link {
            pos: grid_handler.generate_rand_coordinate(),
            link_color: Color::new(0.2, 0.2, 0.2, 1.0),
        };

        self.links.push_front(head);
        self
    }

	//Add new links/body parts to the snake
	pub fn add_link (&mut self, grid_handler: &Grid) {
		let add_direction:(i32, i32);
		let mut new_position = (0, 0);

		//TODO: Finish adding link if the head is not the only link in the list
		if self.links.len() == 1 {
			add_direction = (self.move_direction.0 * -1, self.move_direction.1 * -1);
			let front = self.links.front().unwrap();
			new_position = (front.pos.0 + add_direction.0, front.pos.1 + add_direction.1);
		}
		else {

		}

		let new_link = Link {
			pos: new_position,
			link_color: Color::new(0.2, 0.2, 0.2, 1.0),
		};

		self.links.push_back(new_link);
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


	//TODO: Finish updating positions for links that are not the head in the list
	//Updates/Moves the snake on the grid based on the direction of the snake
	pub fn update_position(&mut self, gird_handler: &Grid) {
		if let Some(front) = self.links.front_mut() {
			front.pos.0 =  (front.pos.0 + self.move_direction.0).rem_euclid(gird_handler.cells.len() as i32);
			front.pos.1 =  (front.pos.1 + self.move_direction.1).rem_euclid(gird_handler.cells.len() as i32);
		}
		
		for link in &mut self.links.iter_mut().skip(1) {
			link.pos.0 = (link.pos.0 + self.move_direction.0).rem_euclid(gird_handler.cells.len() as i32);
			link.pos.1 = (link.pos.1 + self.move_direction.1).rem_euclid(gird_handler.cells.len() as i32);
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
