extern crate arraymap;

use arraymap::ArrayMap;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::video::Window;

use crate::position::Position;

pub mod shape;

use crate::WINDOW_WIDTH;
use shape::ATOM_SIZE;
use shape::Shape;
use shape::atom::Atom;


pub struct Tetromino {
    shape: Shape,
    pub position: Position,
    atoms: [Atom; 4],
}

fn init_atoms(shape :Shape) -> [Atom; 4] {
    let size = ATOM_SIZE as i32;

    // Coordinates are computed in the boxes containing the shapes, following:
    // (-2, -2) (-1, -2) (0, -2) (1, -2)
    // (-2, -1) (-1, -1) (0, -1) (1, -1)
    // (-2,  0) (-1,  0) (0,  0) (1,  0)
    // (-2,  1) (-1,  1) (0,  1) (1,  1)
    let coordinates :[(i32, i32); 4] = match shape {
        Shape::I => [ (-2, -1), (-1, -1), (0, -1), (1, -1) ],
        Shape::J => [ (-2, -2), (-2, -1), (-1, -1), (0, -1) ],
        Shape::L => [ (-2, -1), (-1, -1), (0, -1), (-2, 0) ],
        Shape::O => [ (-1, -1), (-1, 0), (0, -1), (0, 0) ],
        Shape::S => [ (-2, -1), (-1, -1), (-1, -2), (0, -2) ],
        Shape::T => [ (-2, -1), (-1, -1), (0, -1), (-1, -2) ],
        Shape::Z => [ (-2, -2), (-1, -2), (-1, -1), (0, -1) ],
    };

    coordinates
        .map(|(x, y)| ((*x) * size, (*y) * size))
        .map(|coordinates| Position::from(*coordinates))
        .map(|position| Atom::from(*position))
}


impl Tetromino {
    pub fn draw_on(&self, canvas :&mut Canvas<Window>) {
        let size = ATOM_SIZE as i32;

        canvas.set_draw_color(self.shape.color().rgb());

        for atom in self.atoms.iter() {
            // The "center" of the box containing the shape is 2 units away from the left, and from
            // the top (see coordinate system above).
            let x = self.position.x + atom.position.x + 2 * size;
            let y = self.position.y + atom.position.y + 2 * size;
            let square :Rect = Rect::new(x, y, atom.size, atom.size);

            canvas.fill_rect(square);
        }
    }

    pub fn move_down(&mut self) {
        self.position.y += ATOM_SIZE as i32;
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

    pub fn new(position :Position, shape :Shape) -> Tetromino {
        let atoms = init_atoms(shape.clone());

        Tetromino { shape, position, atoms }
    }

    fn width(&self) -> u32 {
        let mut x_coordinates :Vec<i32> = self.atoms
            .iter()
            .map(|atom| atom.position.x)
            .collect();

        x_coordinates.sort();
        x_coordinates.dedup();

        x_coordinates.len() as u32 * ATOM_SIZE
    }
}
