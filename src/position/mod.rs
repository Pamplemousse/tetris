#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Position {
    fn from(coordinates: (i32, i32)) -> Self {
        let (x, y) = coordinates;
        Position { x, y }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_can_be_instanciated_from_integers_tuple() {
        let coordinates :(i32, i32) = (41, 42);

        let position :Position = Position::from(coordinates);

        assert_eq!(position.x, 41);
        assert_eq!(position.y, 42);
    }
}
