extern crate arraymap;

use arraymap::ArrayMap;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::video::Window;

mod rotation;
pub mod shape;

use crate::WINDOW_WIDTH;
use crate::position::Position;
use rotation::new_rotations_for_shape;
use rotation::AtomsPositions;
use rotation::RotationCycleForShape;
use shape::ATOM_SIZE;
use shape::Shape;
use shape::atom::Atom;


fn atoms_from_positions(positions :AtomsPositions) -> [Atom; 4] {
    positions
        .map(|position| Atom::from(*position))
}

pub struct Tetromino {
    atoms: [Atom; 4],
    // TODO: keep pub?
    pub position: Position,
    rotations_iterator: RotationCycleForShape,
    shape: Shape,
}

impl Tetromino {
    pub fn draw_on(&self, canvas :&mut Canvas<Window>) {
        let size = ATOM_SIZE as i32;

        canvas.set_draw_color(self.shape.color().rgb());

        for atom in self.atoms.iter() {
            // The "center" of the box containing the shape is 2 units away from the left, and from
            // the top (see coordinate system in src/tetromino/rotation/mod.rs).
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
        // TODO: debug
        let new_x = self.position.x + ATOM_SIZE as i32;

        if new_x as u32 + self.width() <= WINDOW_WIDTH {
            self.position.x = new_x;
        }
    }

    pub fn new(position :Position, shape :Shape) -> Tetromino {
        let mut rotations_iterator = new_rotations_for_shape(shape.clone());
        let atoms = atoms_from_positions(
            *rotations_iterator.next().unwrap()
        );

        Tetromino { shape, position, atoms, rotations_iterator }
    }

    pub fn rotate_clockwise(&mut self) {
        let atoms_positions = self.rotations_iterator.next().unwrap();
        self.atoms = atoms_from_positions(*atoms_positions)
    }

    fn width(&self) -> u32 {
        let mut x_coordinates :Vec<i32> = self.atoms
            .iter()
            .map(|atom| atom.position.x)
            .collect();

        x_coordinates.sort();
        x_coordinates.dedup();

        println!("{:?}", x_coordinates[x_coordinates.len() - 1]);
        println!("{:?}", x_coordinates[0]);

        x_coordinates.len() as u32 * ATOM_SIZE
    }
}
