use sdl2::render::Canvas;
use sdl2::pixels::Color as SDL2Color;
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
    let size_taken = ATOM_SIZE as i32;

    match shape {
        Shape::I => [ Atom::from(0, 0), Atom::from(0, size_taken), Atom::from(0, 2*size_taken), Atom::from(0, 3*size_taken) ],
        Shape::J => [ Atom::from(size_taken, 0), Atom::from(size_taken, size_taken), Atom::from(size_taken, 2*size_taken), Atom::from(0, 2*size_taken) ],
        Shape::L => [ Atom::from(0, 0), Atom::from(0, size_taken), Atom::from(0, 2*size_taken), Atom::from(size_taken, 2*size_taken) ],
        Shape::O => [ Atom::from(0, 0), Atom::from(0, size_taken), Atom::from(size_taken, 0), Atom::from(size_taken, size_taken) ],
        Shape::S => [ Atom::from(size_taken, 0), Atom::from(2*size_taken, 0), Atom::from(0, size_taken), Atom::from(size_taken, size_taken) ],
        Shape::T => [ Atom::from(0, 0), Atom::from(size_taken, 0), Atom::from(2*size_taken, 0), Atom::from(size_taken, size_taken) ],
        Shape::Z => [ Atom::from(0, 0), Atom::from(size_taken, 0), Atom::from(size_taken, size_taken), Atom::from(2*size_taken, size_taken) ],
    }
}


impl Tetromino {
    pub fn draw_on(&self, canvas :&mut Canvas<Window>) {
        canvas.set_draw_color(self.shape.color().rgb());

        for atom in self.atoms.iter() {
            let x = self.position.x + atom.position.x;
            let y = self.position.y + atom.position.y;
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

    pub fn new(x :i32, y :i32, shape :Shape) -> Tetromino {
        let atoms = init_atoms(shape.clone());
        let position = Position { x, y };

        Tetromino { shape, position, atoms }
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
