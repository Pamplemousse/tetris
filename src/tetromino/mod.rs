use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::color::Color;
use crate::position::Position;

pub mod shape;
use shape::Shape;


pub struct Tetromino {
    shape: Shape,
    color: Color,
    position: Position,
}

impl Tetromino {
    pub fn draw_on(&self, canvas :&mut Canvas<Window>) {
        self.shape.draw_on(canvas, self.position, self.color.rgb());
    }

    pub fn new(x :i32, y :i32, shape :Shape) -> Tetromino {
        let position = Position { x, y };
        let color = match shape {
            Shape::I => Color::TEAL,
            Shape::J => Color::BLUE,
            Shape::L => Color::ORANGE,
            Shape::O => Color::YELLOW,
            Shape::S => Color::GREEN,
            Shape::T => Color::PINK,
            Shape::Z => Color::RED,
        };

        Tetromino { shape, color, position }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_tetromino_has_its_own_color() {
        let result: Vec<Color> = [
            Shape::I,
            Shape::J,
            Shape::L,
            Shape::O,
            Shape::S,
            Shape::T,
            Shape::Z
        ]
            .iter()
            .cloned()
            .map(|shape| Tetromino::new(0, 0, shape))
            .map(|tetromino| tetromino.color)
            .collect();

        let expected_result: Vec<Color> = [
            Color::TEAL,
            Color::BLUE,
            Color::ORANGE,
            Color::YELLOW,
            Color::GREEN,
            Color::PINK,
            Color::RED
        ].to_vec();

        assert_eq!(result, expected_result);
    }
}
