use std::iter::Cycle;
use std::slice::Iter;

use arraymap::ArrayMap;

use crate::position::Position;
use super::shape::ATOM_SIZE;
use super::shape::Shape;


pub type AtomsPositions = [Position; 4];
pub type RotationCycleForShape = Cycle<Iter<'static, AtomsPositions>>;

lazy_static! {
    static ref ROTATIONS: [[AtomsPositions; 4] ; 7] = generate_rotations();
}

fn generate_rotations() -> [[AtomsPositions; 4]; 7] {
    // Coordinates are computed in the boxes containing the shapes, following:
    // (-2, -2) (-1, -2) (0, -2) (1, -2)
    // (-2, -1) (-1, -1) (0, -1) (1, -1)
    // (-2,  0)  (-1,  0) (0,  0) (1,  0)
    // (-2,  1)  (-1,  1) (0,  1) (1,  1)

    let rotations = [
        [   // I
            [ (-2, -1), (-1, -1), (0, -1), (1, -1) ],
            [ (0, -2), (0, -1), (0, 0), (0, 1) ],
            [ (-2, 0), (-1, 0), (0, 0), (1, 0) ],
            [ (-1, -2), (-1, -1), (-1, 0), (-1, 1) ],
        ],
        [   // J
            [ (-2, -2), (-2, -1), (-1, -1), (0, -1) ],
            [ (-1, -2), (-1, -1), (-1, 0), (0, -2) ],
            [ (-2, -1), (-1, -1), (0, -1), (0, 0) ],
            [ (-1, -2), (-1, -1), (-1, 0), (-2, 0) ],
        ],
        [   // L
            [ (-2, -1), (-1, -1), (0, -1), (0, -2) ],
            [ (0, -2), (0, -1), (0, 0), (1, 0) ],
            [ (-2, -1), (-1, -1), (0, -1), (-2, 0) ],
            [ (0, -2), (0, -1), (0, 0), (-1, -2) ],
        ],
        [   // O
            [ (-1, -1), (-1, 0), (0, -1), (0, 0) ],
            [ (-1, -1), (-1, 0), (0, -1), (0, 0) ],
            [ (-1, -1), (-1, 0), (0, -1), (0, 0) ],
            [ (-1, -1), (-1, 0), (0, -1), (0, 0) ],
        ],
        [   // S
            [ (-2, -1), (-1, -1), (-1, -2), (0, -2) ],
            [ (-1, -2), (-1, -1), (0, -1), (0, 0) ],
            [ (-2, 0), (-1, 0), (-1, -1), (0, -1) ],
            [ (-2, -2), (-2, -1), (-1, -1), (-1, 0) ],
        ],
        [   // T
            [ (-2, -1), (-1, -1), (0, -1), (-1, -2) ],
            [ (-1, -2), (-1, -1), (-1, 0), (0, -1) ],
            [ (-2, -1), (-1, -1), (-1, 0), (0, -1) ],
            [ (-1, -2), (-1, -1), (-1, 0), (-2, -1) ],
        ],
        [   // Z
            [ (-2, -2), (-1, -2), (-1, -1), (0, -1) ],
            [ (0, -2), (0, -1), (-1, -1), (-1, 0) ],
            [ (-2, -1), (-1, -1), (-1, 0), (0, 0) ],
            [ (-2, 0), (-2, -1), (-1, -1), (-1, -2) ],
        ],
    ];

    rotations
        .map(|rotations_for_shape_coordinates| {
            atoms_positions_from_lists_of_coordinates(*rotations_for_shape_coordinates)
        })
}

fn atoms_positions_from_lists_of_coordinates(rotations_coordinates :[[(i32, i32); 4]; 4]) -> [AtomsPositions; 4] {
    let size = ATOM_SIZE as i32;

    rotations_coordinates
        .map(|list_of_coordinates| {
            list_of_coordinates
                .map(|(x, y)| ((*x) * size, (*y) * size))
                .map(|coordinates| Position::from(*coordinates))
        })
}

pub fn new_rotations_for_shape(shape :Shape) -> RotationCycleForShape {
    ROTATIONS[shape.index()]
        .iter()
        .cycle()
}
