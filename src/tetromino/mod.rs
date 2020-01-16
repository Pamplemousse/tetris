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
        self.position.x -= ATOM_SIZE as i32;
    }

    pub fn move_right(&mut self) {
        self.position.x += ATOM_SIZE as i32;
    }

    pub fn new(x :i32, y :i32, shape :Shape) -> Tetromino {
        let position = Position { x, y };
        Tetromino { shape, position }
    }
}
