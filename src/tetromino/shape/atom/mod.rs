use crate::position::Position;


pub static ATOM_SIZE :u32 = 20;

pub struct Atom {
    pub position: Position,
    pub size: u32,
}

impl Atom {
    pub fn from(x :i32, y :i32) -> Atom {
        let size = ATOM_SIZE;
        let position = Position { x, y };

        Atom { position, size }
    }
}

