use crate::position::Position;


pub static ATOM_SIZE :u32 = 20;

pub struct Atom {
    pub position: Position,
    pub size: u32,
}

impl Atom {
    pub fn from(position :Position) -> Atom {
        let size = ATOM_SIZE;

        Atom { position, size }
    }
}

