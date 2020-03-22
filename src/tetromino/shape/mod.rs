pub mod atom;

use atom::ATOM_SIZE as ATOM_CONTENT_SIZE;
use crate::color::Color;


pub static ATOM_SIZE :u32 = ATOM_CONTENT_SIZE + MARGIN_SIZE;
pub static MARGIN_SIZE :u32 = 1;

#[derive(Clone)]
pub enum Shape {
    I, J, L, O, S, T, Z
}


impl Shape {
    pub fn index(&self) -> usize {
        match self {
            Shape::I => 0,
            Shape::J => 1,
            Shape::L => 2,
            Shape::O => 3,
            Shape::S => 4,
            Shape::T => 5,
            Shape::Z => 6,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Shape::I => Color::TEAL,
            Shape::J => Color::BLUE,
            Shape::L => Color::ORANGE,
            Shape::O => Color::YELLOW,
            Shape::S => Color::GREEN,
            Shape::T => Color::PINK,
            Shape::Z => Color::RED,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_shape_has_its_own_color() {
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
            .map(|shape| shape.color())
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
