use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::position::Position;

pub mod shape;

use crate::WINDOW_WIDTH;
use shape::ATOM_SIZE;
use shape::Shape;


pub struct Tetromino {
    shape: Shape,
    pub position: Position,
}

impl Tetromino {
    pub fn draw_on(&self, canvas :&mut Canvas<Window>) {
        self.shape.draw_on(canvas, self.position, self.shape.color().rgb());
    }

    pub fn move_down(&mut self) {
        self.position.y += 1;
    }

    pub fn move_left(&mut self) {
        let new_x = self.position.x - ATOM_SIZE as i32;

        if new_x >= 0 {
            self.position.x = new_x;
        }
    }

    pub fn move_right(&mut self) {
        let new_x = self.position.x + ATOM_SIZE as i32;

        if new_x as u32 + self.width() <= WINDOW_WIDTH {
            self.position.x = new_x;
        }
    }

    pub fn new(x :i32, y :i32, shape :Shape) -> Tetromino {
        let position = Position { x, y };
        Tetromino { shape, position }
    }

    fn width(&self) -> u32 {
        let width_in_number_of_atoms = match self.shape {
            Shape::I => 1,
            Shape::J | Shape::L | Shape::O => 2,
            Shape::S | Shape::T | Shape::Z => 3
        };

        width_in_number_of_atoms * ATOM_SIZE
    }
}
