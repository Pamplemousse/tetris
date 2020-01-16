use sdl2::pixels::Color as SDL2Color;
use std::fmt::{Debug, Formatter, Error};


#[derive(Clone, PartialEq)]
pub enum Color {
    BLUE,
    GREEN,
    ORANGE,
    PINK,
    RED,
    TEAL,
    WHITE,
    YELLOW
}


impl Color {
    pub fn rgb(&self) -> SDL2Color {
        match self {
            Color::BLUE => SDL2Color::RGB(0, 0, 255),
            Color::GREEN => SDL2Color::RGB(0, 255, 0),
            Color::ORANGE => SDL2Color::RGB(255, 165, 0),
            Color::PINK => SDL2Color::RGB(255, 192, 203),
            Color::RED => SDL2Color::RGB(255, 0, 0),
            Color::TEAL => SDL2Color::RGB(0, 255, 255),
            Color::WHITE => SDL2Color::RGB(255, 255, 255),
            Color::YELLOW => SDL2Color::RGB(255, 255, 0),
        }
    }
}


impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Color::BLUE => write!(f, "Color::BLUE"),
            Color::GREEN => write!(f, "Color::GREEN"),
            Color::ORANGE => write!(f, "Color::ORANGE"),
            Color::PINK => write!(f, "Color::PINK"),
            Color::RED => write!(f, "Color::RED"),
            Color::TEAL => write!(f, "Color::TEAL"),
            Color::WHITE => write!(f, "Color::WHITE"),
            Color::YELLOW => write!(f, "Color::YELLOW"),
        }
    }
}
