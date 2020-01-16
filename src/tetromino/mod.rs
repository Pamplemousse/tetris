use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::position::Position;

pub mod shape;
use shape::Shape;


pub struct Tetromino {
    shape: Shape,
    position: Position,
}

impl Tetromino {
    pub fn draw_on(&self, canvas :&mut Canvas<Window>) {
        self.shape.draw_on(canvas, self.position, self.shape.color().rgb());
    }

    pub fn new(x :i32, y :i32, shape :Shape) -> Tetromino {
        let position = Position { x, y };
        Tetromino { shape, position }
    }
}
