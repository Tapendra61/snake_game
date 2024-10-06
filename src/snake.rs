use ::std::collections::*;
use macroquad::{color::Color, shapes::draw_rectangle};

use crate::grid::Grid;

pub struct Link {
    pos: (i32, i32),
    link_color: Color,
}

pub struct Snake {
    links: LinkedList<Link>,
    move_direction: (i32, i32),
}

impl Snake {
    pub fn new(grid: &Grid) -> Self {
        Snake {
            links: LinkedList::new(),
            move_direction: (0, 0),
        }
        .initialize_head(grid)
    }

    fn initialize_head(mut self, grid: &Grid) -> Self {
        let head = Link {
            pos: (0, 0),
            link_color: Color::new(0.2, 0.2, 0.2, 1.0),
        };
        self.links.push_front(head);
        self
    }

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
