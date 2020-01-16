use sdl2::render::Canvas;
use sdl2::pixels::Color as SDL2Color;
use sdl2::rect::Rect;
use sdl2::video::Window;

mod atom;

use atom::Atom;
use atom::ATOM_SIZE as ATOM_CONTENT_SIZE;
use crate::color::Color;
use crate::position::Position;


pub static ATOM_SIZE :u32 = ATOM_CONTENT_SIZE + MARGIN_SIZE;
pub static MARGIN_SIZE :u32 = 1;

#[derive(Clone)]
pub enum Shape {
    I, J, L, O, S, T, Z
}

impl Shape {
    pub fn atoms(&self) -> [Atom; 4] {
        let size_taken = ATOM_SIZE as i32;

        match self {
            Shape::I => [ Atom::from(0, 0), Atom::from(0, size_taken), Atom::from(0, 2*size_taken), Atom::from(0, 3*size_taken) ],
            Shape::J => [ Atom::from(size_taken, 0), Atom::from(size_taken, size_taken), Atom::from(size_taken, 2*size_taken), Atom::from(0, 2*size_taken) ],
            Shape::L => [ Atom::from(0, 0), Atom::from(0, size_taken), Atom::from(0, 2*size_taken), Atom::from(size_taken, 2*size_taken) ],
            Shape::O => [ Atom::from(0, 0), Atom::from(0, size_taken), Atom::from(size_taken, 0), Atom::from(size_taken, size_taken) ],
            Shape::S => [ Atom::from(size_taken, 0), Atom::from(2*size_taken, 0), Atom::from(0, size_taken), Atom::from(size_taken, size_taken) ],
            Shape::T => [ Atom::from(0, 0), Atom::from(size_taken, 0), Atom::from(2*size_taken, 0), Atom::from(size_taken, size_taken) ],
            Shape::Z => [ Atom::from(0, 0), Atom::from(size_taken, 0), Atom::from(size_taken, size_taken), Atom::from(2*size_taken, size_taken) ],
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

    pub fn draw_on(&self, canvas :&mut Canvas<Window>, position :Position, color :SDL2Color) {
        canvas.set_draw_color(color);

        for atom in self.atoms().iter() {
            let x = position.x + atom.position.x;
            let y = position.y + atom.position.y;
            let square :Rect = Rect::new(x, y, atom.size, atom.size);

            canvas.fill_rect(square);
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
